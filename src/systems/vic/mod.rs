use crate::cpu::Mos6502;
use crate::keyboard::{
  commodore::{C64KeyboardAdapter, C64SymbolAdapter},
  KeyAdapter, KeyMappingStrategy, SymbolAdapter,
};
use crate::memory::mos652x::Via;
use crate::memory::{BlockMemory, BranchMemory, NullMemory, NullPort, Port, SystemInfo};
use crate::platform::{PlatformProvider, WindowConfig};
use crate::roms::RomFile;
use crate::systems::System;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::Arc;

mod chip;
mod keyboard;
use self::keyboard::KEYBOARD_MAPPING;
use chip::{VicChip, VicChipIO};

use instant::Duration;
#[cfg(target_arch = "wasm32")]
use js_sys::Reflect;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

#[cfg(target_arch = "wasm32")]
use js_sys::Uint8Array;

use super::SystemBuilder;

/// The set of ROM files required to run a VIC-20 system.
pub struct Vic20SystemRoms {
  /// Character ROM. Used to generate the 8x8 character bitmaps.
  pub character: RomFile,

  /// Basic ROM. Contains the BASIC interpreter.
  pub basic: RomFile,

  /// Kernal ROM. Contains the operating system and editor functions.
  pub kernal: RomFile,

  /// Cartridge ROM. Contains the contents of a cartridge, if one is inserted.
  pub cartridge: Option<RomFile>,
}

impl Vic20SystemRoms {
  /// Load the ROM files from files.
  #[cfg(not(target_arch = "wasm32"))]
  pub fn from_disk(cartridge_path: Option<&str>) -> Self {
    use crate::roms::DiskLoadable;

    let character = RomFile::from_file("vic/char.bin");
    let basic = RomFile::from_file("vic/basic.bin");
    let kernal = RomFile::from_file("vic/kernal.bin");
    let cartridge = cartridge_path.map(RomFile::from_file);

    Self {
      character,
      basic,
      kernal,
      cartridge,
    }
  }

  #[cfg(target_arch = "wasm32")]
  pub fn from_jsvalue(value: &JsValue) -> Self {
    use crate::roms::JsValueLoadable;

    let character = Reflect::get(value, &JsValue::from_str("char"))
      .unwrap()
      .dyn_into::<Uint8Array>()
      .unwrap();
    let basic = Reflect::get(value, &JsValue::from_str("basic"))
      .unwrap()
      .dyn_into::<Uint8Array>()
      .unwrap();
    let kernal = Reflect::get(value, &JsValue::from_str("kernal"))
      .unwrap()
      .dyn_into::<Uint8Array>()
      .unwrap();
    let cartridge = Reflect::get(value, &JsValue::from_str("cartridge"))
      .unwrap()
      .dyn_into::<Uint8Array>()
      .unwrap();

    Self {
      character: RomFile::from_uint8array(&character),
      basic: RomFile::from_uint8array(&basic),
      kernal: RomFile::from_uint8array(&kernal),
      cartridge: Some(RomFile::from_uint8array(&cartridge)),
    }
  }
}

/// Port A on the first VIA chip.
/// This is used to read the state from the joystick.
pub struct VicVia1PortA {
  platform: Arc<dyn PlatformProvider>,
  joy_pin_3: Rc<Cell<bool>>,
}

impl VicVia1PortA {
  pub fn new(platform: Arc<dyn PlatformProvider>) -> Self {
    Self {
      platform,
      joy_pin_3: Rc::new(Cell::new(true)),
    }
  }

  /// Return a reference to the joystick's pin 3 state.
  pub fn get_joy_pin_3(&self) -> Rc<Cell<bool>> {
    self.joy_pin_3.clone()
  }
}

impl Port for VicVia1PortA {
  fn read(&mut self) -> u8 {
    let joystick = self.platform.get_joystick_state();

    let pin_0 = !joystick.up;
    let pin_1 = !joystick.down;
    let pin_2 = !joystick.left;
    self.joy_pin_3.set(!joystick.right);
    let lightpen_fire = !joystick.fire;

    (pin_0 as u8) << 2 | (pin_1 as u8) << 3 | (pin_2 as u8) << 4 | (lightpen_fire as u8) << 5
  }

  fn write(&mut self, _value: u8) {}

  fn poll(&mut self, _cycles: u32, _info: &SystemInfo) -> bool {
    false
  }

  fn reset(&mut self) {}
}

/// Port B on the second VIA chip.
/// This is used to set the active columns on the keyboard matrix,
/// and to read the third pin of the joystick.
pub struct VicVia2PortB {
  keyboard_col: Rc<Cell<u8>>,
  joy_pin_3: Rc<Cell<bool>>,
}

impl VicVia2PortB {
  pub fn new(joy_pin_3: Rc<Cell<bool>>) -> Self {
    Self {
      keyboard_col: Rc::new(Cell::new(0)),
      joy_pin_3,
    }
  }

  /// Return a reference to the keyboard column's current value.
  pub fn get_keyboard_col(&self) -> Rc<Cell<u8>> {
    self.keyboard_col.clone()
  }
}

impl Port for VicVia2PortB {
  fn read(&mut self) -> u8 {
    self.keyboard_col.get() | (self.joy_pin_3.get() as u8) << 7
  }

  fn write(&mut self, value: u8) {
    self.keyboard_col.set(value);
  }

  fn poll(&mut self, _cycles: u32, _info: &SystemInfo) -> bool {
    false
  }

  fn reset(&mut self) {}
}

/// Port A on the second VIA chip.
/// This is used to read the active rows on the keyboard matrix.
pub struct VicVia2PortA {
  keyboard_col: Rc<Cell<u8>>,
  mapping_strategy: KeyMappingStrategy,
  platform: Arc<dyn PlatformProvider>,
}

impl VicVia2PortA {
  /// Create a new instance of the port, with the given keyboard column,
  /// reading the key status from the given platform.
  pub fn new(
    keyboard_col: Rc<Cell<u8>>,
    mapping_strategy: KeyMappingStrategy,
    platform: Arc<dyn PlatformProvider>,
  ) -> Self {
    Self {
      keyboard_col,
      mapping_strategy,
      platform,
    }
  }
}

impl Port for VicVia2PortA {
  fn read(&mut self) -> u8 {
    let col_mask = self.keyboard_col.get();

    let mut value = 0b1111_1111;

    let state = match &self.mapping_strategy {
      KeyMappingStrategy::Physical => C64KeyboardAdapter::map(&self.platform.get_key_state()),
      KeyMappingStrategy::Symbolic => {
        C64SymbolAdapter::map(&SymbolAdapter::map(&self.platform.get_key_state()))
      }
    };

    for (y, row) in KEYBOARD_MAPPING.iter().enumerate() {
      for (x, key) in row.iter().enumerate() {
        if ((!col_mask & (1 << x)) != 0) && state.is_pressed(*key) {
          value &= !(1 << y);
        }
      }
    }

    value
  }

  fn write(&mut self, _value: u8) {}

  fn poll(&mut self, _cycles: u32, _info: &SystemInfo) -> bool {
    false
  }

  fn reset(&mut self) {}
}

/// Configuration for a VIC-20 system.
pub struct Vic20SystemConfig {
  pub mapping: KeyMappingStrategy,
}

/// A factory for creating a VIC-20 system.
pub struct Vic20SystemBuilder;

impl SystemBuilder<Vic20System, Vic20SystemRoms, Vic20SystemConfig> for Vic20SystemBuilder {
  fn build(
    roms: Vic20SystemRoms,
    config: Vic20SystemConfig,
    platform: Arc<dyn PlatformProvider>,
  ) -> Box<dyn System> {
    let low_ram = BlockMemory::ram(0x0400);
    let main_ram = BlockMemory::ram(0x0E00);

    let vic_chip = Rc::new(RefCell::new(VicChip::new(platform.clone())));

    let v1a = VicVia1PortA::new(platform.clone());
    let v2b = VicVia2PortB::new(v1a.get_joy_pin_3());
    let v2a = VicVia2PortA::new(v2b.get_keyboard_col(), config.mapping, platform);

    let via1 = Via::new(Box::new(v1a), Box::new(NullPort::new()));
    let via2 = Via::new(Box::new(v2a), Box::new(v2b));

    let basic_rom = BlockMemory::from_file(0x2000, roms.basic);
    let kernel_rom = BlockMemory::from_file(0x2000, roms.kernal);

    let vram = BlockMemory::ram(0x0200);
    let characters = BlockMemory::from_file(0x1000, roms.character);
    let colors = BlockMemory::ram(0x0200);
    let chip_io = VicChipIO::new(vic_chip.clone());

    let cartridge = match roms.cartridge {
      Some(rom) => BlockMemory::from_file(0x4000, rom),
      None => BlockMemory::ram(0x4000),
    };

    let memory = BranchMemory::new()
      .map(0x0000, Box::new(low_ram))
      .map(0x0400, Box::new(NullMemory::new()))
      .map(0x1000, Box::new(main_ram))
      .map(0x1E00, Box::new(vram))
      .map(0x2000, Box::new(NullMemory::new()))
      // .map(0x2000, Box::new(expansion_ram))
      .map(0x8000, Box::new(characters))
      .map(0x9000, Box::new(chip_io))
      .map(0x9010, Box::new(NullMemory::new()))
      .map(0x9110, Box::new(via1))
      .map(0x9120, Box::new(via2))
      .map(0x9130, Box::new(NullMemory::new()))
      .map(0x9600, Box::new(colors))
      .map(0xA000, Box::new(cartridge))
      .map(0xC000, Box::new(basic_rom))
      .map(0xE000, Box::new(kernel_rom));

    let cpu = Mos6502::new(Box::new(memory));

    Box::new(Vic20System { cpu, vic: vic_chip })
  }
}

/// The VIC-20 system by Commodore.
pub struct Vic20System {
  cpu: Mos6502,
  vic: Rc<RefCell<VicChip>>,
}

impl System for Vic20System {
  fn tick(&mut self) -> instant::Duration {
    Duration::from_secs_f64(1.0 / 1_000_000.0) * self.cpu.tick() as u32
  }

  fn reset(&mut self) {
    self.cpu.reset();
  }

  fn render(&mut self, framebuffer: &mut [u8], _config: WindowConfig) {
    self
      .vic
      .borrow_mut()
      .redraw_screen(&mut self.cpu.memory, framebuffer);
  }
}
