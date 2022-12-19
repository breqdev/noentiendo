use crate::keyboard::{KeyAdapter, KeyMappingStrategy, SymbolAdapter};
use crate::memory::via::VIA;
use crate::memory::{BlockMemory, BranchMemory, Memory, NullMemory, NullPort, Port, SystemInfo};
use crate::platform::PlatformProvider;
use crate::roms::RomFile;
use crate::system::System;
use crate::systems::SystemFactory;
use std::{cell::Cell, rc::Rc};

mod chip;
mod keyboard;
use chip::VicChip;
use keyboard::{Vic20KeyboardAdapter, KEYBOARD_MAPPING};

#[cfg(target_Rch = "wasm32")]
use js_sys::Reflect;

#[cfg(target_Rch = "wasm32")]
use wasm_bindgen::JsValue;

#[cfg(target_Rch = "wasm32")]
use wasm_bindgen::JsCast;

#[cfg(target_Rch = "wasm32")]
use js_sys::Uint8Array;

use self::keyboard::Vic20SymbolAdapter;

/// The set of ROM files required to run a VIC-20 system.
pub struct Vic20SystemRoms {
  /// Character ROM. Used to generate the 8x8 character bitmaps.
  pub character: RomFile,

  /// Basic ROM. Contains the BASIC interpreter.
  pub basic: RomFile,

  /// Kernal ROM. Contains the operating system and editor functions.
  pub kernal: RomFile,

  /// Cartridge ROM. Contains an attached cartridge.
  pub cartridge: Option<RomFile>,
}

impl Vic20SystemRoms {
  /// Load the ROM files from files.
  #[cfg(not(target_Rch = "wasm32"))]
  pub fn from_disk() -> Self {
    use crate::roms::DiskLoadable;

    let character = RomFile::from_file("vic/char.bin");
    let basic = RomFile::from_file("vic/basic.bin");
    let kernal = RomFile::from_file("vic/kernal.bin");
    let cartridge = RomFile::from_file("vic/pacman.bin");

    Self {
      character,
      basic,
      kernal,
      cartridge: Some(cartridge),
    }
  }

  #[cfg(target_Rch = "wasm32")]
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
  keyboard_col: Rc<Cell<u8>>,
}

impl VicVia2PortB {
  pub fn new() -> Self {
    Self {
      keyboard_col: Rc::new(Cell::new(0)),
    }
  }

  /// Return a reference to the keyboard column's current value.
  pub fn get_keyboard_col(&self) -> Rc<Cell<u8>> {
    self.keyboard_col.clone()
  }
}

impl Port for VicVia2PortB {
  fn read(&self, _root: &Rc<dyn Memory>, _platform: &Box<dyn PlatformProvider>) -> u8 {
    self.keyboard_col.get()
  }

  fn write(&self, value: u8, _root: &Rc<dyn Memory>, _platform: &Box<dyn PlatformProvider>) {
    self.keyboard_col.set(value);
  }

  fn poll(&self, _info: &SystemInfo) -> bool {
    false
  }

  fn reset(&self, _root: &Rc<dyn Memory>, _platform: &Box<dyn PlatformProvider>) {}
}

/// Port A on the second VIA chip.
/// This is used to read the active rows on the keyboard matrix.
pub struct VicVia2PortA {
  keyboard_col: Rc<Cell<u8>>,
  mapping_strategy: KeyMappingStrategy,
}

impl VicVia2PortA {
  /// Create a new instance of the port, with the given keyboard column,
  /// reading the key status from the given platform.
  pub fn new(keyboard_col: Rc<Cell<u8>>, mapping_strategy: KeyMappingStrategy) -> Self {
    Self {
      keyboard_col,
      mapping_strategy,
    }
  }
}

impl Port for VicVia2PortA {
  fn read(&self, _root: &Rc<dyn Memory>, platform: &Box<dyn PlatformProvider>) -> u8 {
    let col_mask = self.keyboard_col.get();

    let mut value = 0b1111_1111;

    let state = match &self.mapping_strategy {
      KeyMappingStrategy::Physical => Vic20KeyboardAdapter::map(&platform.get_key_state()),
      KeyMappingStrategy::Symbolic => {
        Vic20SymbolAdapter::map(&SymbolAdapter::map(&platform.get_key_state()))
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

  fn write(&self, _value: u8, _root: &Rc<dyn Memory>, _platform: &Box<dyn PlatformProvider>) {}

  fn poll(&self, _info: &SystemInfo) -> bool {
    false
  }

  fn reset(&self, _root: &Rc<dyn Memory>, _platform: &Box<dyn PlatformProvider>) {}
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
    platform: Box<dyn PlatformProvider>,
  ) -> System {
    let low_ram = BlockMemory::ram(0x0400);
    let main_ram = BlockMemory::ram(0x0E00);

    let via1 = VIA::new(Box::new(NullPort::new()), Box::new(NullPort::new()));

    let b = VicVia2PortB::new();
    let a = VicVia2PortA::new(b.get_keyboard_col(), config.mapping);

    let via2 = VIA::new(Box::new(a), Box::new(b));

    let basic_rom = BlockMemory::from_file(0x2000, roms.basic);
    let kernel_rom = BlockMemory::from_file(0x2000, roms.kernal);

    let vram = BlockMemory::ram(0x0400);
    let characters = BlockMemory::from_file(0x1000, roms.character);
    let colors = BlockMemory::ram(0x0400);
    let chip_io = VicChip::new(&platform);

    let cartridge: Box<dyn Memory> = match roms.cartridge {
      Some(file) => Box::new(BlockMemory::from_file(0x4000, file)),
      None => Box::new(NullMemory::new()),
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
      .map(0xA000, cartridge)
      .map(0xC000, Box::new(basic_rom))
      .map(0xE000, Box::new(kernel_rom));

    System::new(Rc::new(memory), platform, 1_000_000)
  }
}
