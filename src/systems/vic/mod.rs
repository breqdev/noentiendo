use crate::memory::via::VIA;
use crate::memory::{BlockMemory, BranchMemory, NullMemory, NullPort, Port, RomFile, SystemInfo};
use crate::platform::{scancodes, PlatformProvider};
use crate::system::System;
use crate::systems::SystemFactory;
use std::sync::{Arc, Mutex};

mod character;
mod chip;
mod color;
mod vram;
use character::VicCharacterRam;
use chip::{VicChip, VicChipIO};
use color::VicColorRam;
use vram::VicVram;

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
    let character = RomFile::from_file("vic/char.bin");
    let basic = RomFile::from_file("vic/basic.bin");
    let kernal = RomFile::from_file("vic/kernal.bin");

    Self {
      character,
      basic,
      kernal,
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
  platform: Arc<dyn PlatformProvider>,
}

impl VicVia2PortA {
  /// Create a new instance of the port, with the given keyboard column,
  /// reading the key status from the given platform.
  pub fn new(keyboard_col: Arc<Mutex<u8>>, platform: Arc<dyn PlatformProvider>) -> Self {
    Self {
      keyboard_col,
      platform,
    }
  }
}

/// The keyboard matrix in a VIC-20 system.
const KEYBOARD_MAPPING: [[char; 8]; 8] = [
  [
    '1',
    scancodes::LEFT_ARROW,
    scancodes::CONTROL,
    scancodes::RUN_STOP,
    ' ',
    scancodes::COMMODORE,
    'Q',
    '2',
  ],
  ['3', 'W', 'A', scancodes::LSHIFT, 'Z', 'S', 'E', '4'],
  ['5', 'R', 'D', 'X', 'C', 'F', 'T', '6'],
  ['7', 'Y', 'G', 'V', 'B', 'H', 'U', '8'],
  ['9', 'I', 'J', 'N', 'M', 'K', 'O', '0'],
  ['+', 'P', 'L', ',', '.', ':', '@', '-'],
  [
    '$',
    '*',
    ';',
    '/',
    scancodes::RSHIFT,
    '=',
    scancodes::UP_ARROW,
    scancodes::HOME,
  ],
  [
    scancodes::BACKSPACE,
    scancodes::RETURN,
    scancodes::RIGHT_ARROW,
    scancodes::DOWN_ARROW,
    scancodes::F1,
    scancodes::F3,
    scancodes::F5,
    scancodes::F7,
  ],
];

impl Port for VicVia2PortA {
  fn read(&mut self) -> u8 {
    let col_mask = *self.keyboard_col.lock().unwrap();

    let mut value = 0b1111_1111;
    for row in 0..8 {
      for col in 0..8 {
        if (!col_mask & (1 << col)) != 0 {
          let key = KEYBOARD_MAPPING[row][col];
          if self.platform.is_pressed(key as u8) {
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

/// The VIC-20 system by Commodore.
pub struct Vic20SystemFactory {}

impl SystemFactory<Vic20SystemRoms> for Vic20SystemFactory {
  fn create(roms: Vic20SystemRoms, platform: Arc<dyn PlatformProvider>) -> System {
    let low_ram = BlockMemory::ram(0x0400);
    let main_ram = BlockMemory::ram(0x0E00);

    let vic_chip = Arc::new(Mutex::new(VicChip::new(platform.clone(), roms.character)));
    let via1 = VIA::new(Box::new(NullPort::new()), Box::new(NullPort::new()));

    let b = VicVia2PortB::new();
    let a = VicVia2PortA::new(b.get_keyboard_col(), platform);

    let via2 = VIA::new(Box::new(a), Box::new(b));

    let basic_rom = BlockMemory::from_file(0x2000, roms.basic);
    let kernel_rom = BlockMemory::from_file(0x2000, roms.kernal);

    let vram = VicVram::new(vic_chip.clone());
    let characters = VicCharacterRam::new(vic_chip.clone());
    let colors = VicColorRam::new(vic_chip.clone());
    let chip_io = VicChipIO::new(vic_chip.clone());

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
      .map(0xC000, Box::new(basic_rom))
      .map(0xE000, Box::new(kernel_rom));

    System::new(Box::new(memory), 1_000_000)
  }
}
