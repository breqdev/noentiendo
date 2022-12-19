use crate::keyboard::{KeyAdapter, KeyMappingStrategy, SymbolAdapter};
use crate::memory::via::VIA;
use crate::memory::{BlockMemory, BranchMemory, NullMemory, NullPort, Port, SystemInfo};
use crate::platform::PlatformProvider;
use crate::roms::{DiskLoadable, RomFile};
use crate::system::System;
use crate::systems::SystemFactory;
use std::sync::{Arc, Mutex};

mod chip;
mod keyboard;
use chip::{VicChip, VicChipIO};
use keyboard::{Vic20KeyboardAdapter, KEYBOARD_MAPPING};

#[cfg(target_arch = "wasm32")]
use js_sys::Reflect;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

#[cfg(target_arch = "wasm32")]
use js_sys::Uint8Array;

use self::chip::VicChipDMA;
use self::keyboard::Vic20SymbolAdapter;

/// The set of ROM files required to run a VIC-20 system.
pub struct Vic20SystemRoms {
  /// Character ROM. Used to generate the 8x8 character bitmaps.
  pub character: RomFile,

  /// Basic ROM. Contains the BASIC interpreter.
  pub basic: RomFile,

  /// Kernal ROM. Contains the operating system and editor functions.
  pub kernal: RomFile,
}

impl Vic20SystemRoms {
  /// Load the ROM files from files.
  #[cfg(not(target_arch = "wasm32"))]
  pub fn from_disk() -> Self {
    use crate::roms::DiskLoadable;

    let character = RomFile::from_file("vic/char.bin");
    let basic = RomFile::from_file("vic/basic.bin");
    let kernal = RomFile::from_file("vic/kernal.bin");

    Self {
      character,
      basic,
      kernal,
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

    Self {
      character: RomFile::from_uint8array(&character),
      basic: RomFile::from_uint8array(&basic),
      kernal: RomFile::from_uint8array(&kernal),
    }
  }
}

/// Port B on the second VIA chip.
/// This is used to set the active columns on the keyboard matrix.
pub struct VicVia2PortB {
  keyboard_col: Arc<Mutex<u8>>,
}

impl VicVia2PortB {
  pub fn new() -> Self {
    Self {
      keyboard_col: Arc::new(Mutex::new(0)),
    }
  }

  /// Return a reference to the keyboard column's current value.
  pub fn get_keyboard_col(&self) -> Arc<Mutex<u8>> {
    self.keyboard_col.clone()
  }
}

impl Port for VicVia2PortB {
  fn read(&mut self) -> u8 {
    *self.keyboard_col.lock().unwrap()
  }

  fn write(&mut self, value: u8) {
    *self.keyboard_col.lock().unwrap() = value;
  }

  fn poll(&mut self, _info: &SystemInfo) -> bool {
    false
  }

  fn reset(&mut self) {}
}

/// Port A on the second VIA chip.
/// This is used to read the active rows on the keyboard matrix.
pub struct VicVia2PortA {
  keyboard_col: Arc<Mutex<u8>>,
  mapping_strategy: KeyMappingStrategy,
  platform: Arc<dyn PlatformProvider>,
}

impl VicVia2PortA {
  /// Create a new instance of the port, with the given keyboard column,
  /// reading the key status from the given platform.
  pub fn new(
    keyboard_col: Arc<Mutex<u8>>,
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
    let col_mask = *self.keyboard_col.lock().unwrap();

    let mut value = 0b1111_1111;

    let state = match &self.mapping_strategy {
      KeyMappingStrategy::Physical => Vic20KeyboardAdapter::map(&self.platform.get_key_state()),
      KeyMappingStrategy::Symbolic => {
        Vic20SymbolAdapter::map(&SymbolAdapter::map(&self.platform.get_key_state()))
      }
    };

    for row in 0..8 {
      for col in 0..8 {
        if (!col_mask & (1 << col)) != 0 {
          let key = KEYBOARD_MAPPING[row][col];
          if state.is_pressed(key) {
            value &= !(1 << row);
          }
        }
      }
    }

    value
  }

  fn write(&mut self, _value: u8) {}

  fn poll(&mut self, _info: &SystemInfo) -> bool {
    false
  }

  fn reset(&mut self) {}
}

/// Configuration for a VIC-20 system.
pub struct Vic20SystemConfig {
  pub mapping: KeyMappingStrategy,
}

/// The VIC-20 system by Commodore.
pub struct Vic20SystemFactory {}

impl SystemFactory<Vic20SystemRoms, Vic20SystemConfig> for Vic20SystemFactory {
  fn create(
    roms: Vic20SystemRoms,
    config: Vic20SystemConfig,
    platform: Arc<dyn PlatformProvider>,
  ) -> System {
    let low_ram = BlockMemory::ram(0x0400);
    let main_ram = BlockMemory::ram(0x0E00);

    let vic_chip = Arc::new(Mutex::new(VicChip::new(platform.clone())));
    let via1 = VIA::new(Box::new(NullPort::new()), Box::new(NullPort::new()));

    let b = VicVia2PortB::new();
    let a = VicVia2PortA::new(b.get_keyboard_col(), config.mapping, platform);

    let via2 = VIA::new(Box::new(a), Box::new(b));

    let basic_rom = BlockMemory::from_file(0x2000, roms.basic);
    let kernel_rom = BlockMemory::from_file(0x2000, roms.kernal);

    let vram = BlockMemory::ram(0x0200);
    let characters = BlockMemory::from_file(0x1000, roms.character);
    let colors = BlockMemory::ram(0x0200);
    let chip_io = VicChipIO::new(vic_chip.clone());

    let cartridge = BlockMemory::from_file(0x4000, RomFile::from_file("vic/pacman.bin"));
    // let cartridge = BlockMemory::ram(0x4000);

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

    let mut system = System::new(Box::new(memory), 1_000_000);

    system.attach_dma(Box::new(VicChipDMA::new(vic_chip)));

    system
  }
}
