use crate::memory::{
  pia::{NullPort, Port, PIA},
  ActiveInterrupt, BlockMemory, BranchMemory, Memory, NullMemory, RomFile, SystemInfo,
};
use crate::platform::{scancodes, Color, PlatformProvider, WindowConfig};
use crate::system::System;
use crate::systems::SystemFactory;
use instant::Instant;
use std::sync::{Arc, Mutex};
use std::time::Duration;

const WIDTH: u32 = 40;
const HEIGHT: u32 = 25;
const CHAR_WIDTH: u32 = 8;
const CHAR_HEIGHT: u32 = 8;
const VRAM_SIZE: usize = 1024; // 24 extra bytes to make mapping easier

// Commodore PET-style column screen memory
// (see https://www.chibiakumas.com/6502/platform4.php#LessonP38 for details)

pub struct PetVram {
  data: Vec<u8>,
  platform: Arc<dyn PlatformProvider>,
  character_rom: Vec<u8>,
  foreground: Color,
  background: Color,
}

impl PetVram {
  pub fn new(character_rom: RomFile, platform: Arc<dyn PlatformProvider>) -> Self {
    platform.request_window(WindowConfig::new(
      WIDTH * CHAR_WIDTH,
      HEIGHT * CHAR_HEIGHT,
      2.0,
    ));

    Self {
      data: vec![0; VRAM_SIZE],
      platform,
      character_rom: character_rom.get_data(),
      foreground: Color::new(0, 255, 0),
      background: Color::new(0, 0, 0),
    }
  }
}

impl Memory for PetVram {
  fn read(&mut self, address: u16) -> u8 {
    self.data[address as usize % VRAM_SIZE]
  }

  fn write(&mut self, address: u16, value: u8) {
    self.data[address as usize % VRAM_SIZE] = value;

    if address >= (HEIGHT * WIDTH) as u16 {
      return; // ignore writes to the extra bytes
    }

    let column = (address % WIDTH as u16) as u32;
    let row = (address / WIDTH as u16) as u32;

    let character_index = (value as usize) * 8;

    let mut character = self.character_rom[character_index..(character_index + 8)].to_vec();

    if value & 0x80 != 0 {
      character = character.iter().map(|&x| !x).collect();
    }

    for line in 0..CHAR_HEIGHT {
      let line_data = character[line as usize];
      for pixel in 0..CHAR_WIDTH {
        let color = if line_data & (1 << (CHAR_WIDTH - 1 - pixel)) != 0 {
          self.foreground
        } else {
          self.background
        };

        self
          .platform
          .set_pixel(column * CHAR_WIDTH + pixel, row * CHAR_HEIGHT + line, color);
      }
    }
  }

  fn reset(&mut self) {
    for i in 0..self.data.len() {
      self.data[i] = 0;
    }

    for x in 0..(WIDTH * CHAR_WIDTH) {
      for y in 0..(HEIGHT * CHAR_HEIGHT) {
        self.platform.set_pixel(x, y, self.background);
      }
    }
  }

  fn poll(&mut self, _info: &SystemInfo) -> ActiveInterrupt {
    ActiveInterrupt::None
  }
}

pub struct PetPia1PortA {
  keyboard_row: Arc<Mutex<u8>>,
  last_draw_instant: Option<Instant>,
  last_draw_cycle: u64,
}

impl PetPia1PortA {
  pub fn new() -> Self {
    Self {
      keyboard_row: Arc::new(Mutex::new(0)),
      last_draw_instant: None,
      last_draw_cycle: 0,
    }
  }

  pub fn get_keyboard_row(&self) -> Arc<Mutex<u8>> {
    self.keyboard_row.clone()
  }
}

impl Port for PetPia1PortA {
  fn read(&mut self) -> u8 {
    0b1000_0000 | *self.keyboard_row.lock().unwrap()
    //^         diagnostic mode off
    // ^        IEEE488 (not implemented)
    //  ^^      Cassette sense (not implemented)
    //     ^^^^ Keyboard row select
  }

  fn write(&mut self, value: u8) {
    *self.keyboard_row.lock().unwrap() = value & 0b1111;
  }

  fn poll(&mut self, info: &SystemInfo) -> bool {
    let min_elapsed = ((info.cycles_per_second as f64 / 60.0) * (2.0 / 3.0)) as u64;

    match self.last_draw_instant {
      Some(last_draw) => {
        if (last_draw.elapsed() > Duration::from_millis(17))
          && (info.cycle_count > self.last_draw_cycle + min_elapsed)
        {
          self.last_draw_cycle = info.cycle_count;
          self.last_draw_instant = Some(Instant::now());
          true
          // false
        } else {
          false
        }
      }
      None => {
        self.last_draw_instant = Some(Instant::now());
        false
      }
    }
  }

  fn reset(&mut self) {
    *self.keyboard_row.lock().unwrap() = 0;
  }
}

pub struct PetPia1PortB {
  keyboard_row: Arc<Mutex<u8>>,
  platform: Arc<dyn PlatformProvider>,
}

impl PetPia1PortB {
  pub fn new(keyboard_row: Arc<Mutex<u8>>, platform: Arc<dyn PlatformProvider>) -> Self {
    Self {
      keyboard_row,
      platform,
    }
  }
}

const KEYBOARD_MAPPING: [[char; 8]; 10] = [
  ['!', '#', '%', '&', '(', '_', '_', '_'],
  ['"', '$', '\'', '\\', ')', '_', '_', scancodes::BACKSPACE],
  ['Q', 'E', 'T', 'U', 'O', '_', '7', '9'],
  ['W', 'R', 'Y', 'I', 'P', '_', '8', '/'],
  ['A', 'D', 'G', 'J', 'L', '_', '4', '6'],
  ['S', 'F', 'H', 'K', ':', '_', '5', '*'],
  ['Z', 'C', 'B', 'M', ';', scancodes::RETURN, '1', '3'],
  ['X', 'V', 'N', ',', '?', '_', '2', '+'],
  [
    scancodes::LSUPER, // use super key instead of shift key for graphics characters
    '@',
    ']',
    '_',
    '>',
    scancodes::RSUPER,
    '0',
    '-',
  ],
  ['_', '[', ' ', '<', '_', '_', '.', '='],
];

impl Port for PetPia1PortB {
  fn read(&mut self) -> u8 {
    let row = *self.keyboard_row.lock().unwrap();
    let row = KEYBOARD_MAPPING[row as usize % 10];
    let mut value = 0b1111_1111;
    for i in 0..8 {
      if self.platform.is_pressed(row[i] as u8) {
        value &= !(1 << i);
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

pub struct PetSystemRoms {
  pub character: RomFile,
  pub basic: RomFile,
  pub editor: RomFile,
  pub kernal: RomFile,
}

impl PetSystemRoms {
  #[cfg(not(target_arch = "wasm32"))]
  pub fn from_disk() -> Self {
    let character = RomFile::from_file("pet/char.bin");
    let basic = RomFile::from_file("pet/basic.bin");
    let editor = RomFile::from_file("pet/editor.bin");
    let kernal = RomFile::from_file("pet/kernal.bin");

    Self {
      character,
      basic,
      editor,
      kernal,
    }
  }
}

pub struct PetSystemFactory {}

impl SystemFactory<PetSystemRoms> for PetSystemFactory {
  fn create(roms: PetSystemRoms, platform: Arc<dyn PlatformProvider>) -> System {
    let ram = BlockMemory::ram(0x8000);
    let vram = PetVram::new(roms.character, platform.clone());

    let expansion_rom_9 = NullMemory::new();
    let expansion_rom_a = NullMemory::new();
    let expansion_rom_b = NullMemory::new();

    let basic_rom = BlockMemory::from_file(0x2000, roms.basic);
    let editor_rom = BlockMemory::from_file(0x1000, roms.editor);

    let port_a = PetPia1PortA::new();
    let port_b = PetPia1PortB::new(port_a.get_keyboard_row(), platform);
    let pia1 = PIA::new(Box::new(port_a), Box::new(port_b));
    let pia2 = PIA::new(Box::new(NullPort::new()), Box::new(NullPort::new()));
    let via = PIA::new(Box::new(NullPort::new()), Box::new(NullPort::new()));

    let kernel_rom = BlockMemory::from_file(0x1000, roms.kernal);

    let memory = BranchMemory::new()
      .map(0x0000, Box::new(ram))
      .map(0x8000, Box::new(vram))
      .map(0x9000, Box::new(expansion_rom_9))
      .map(0xA000, Box::new(expansion_rom_a))
      .map(0xB000, Box::new(expansion_rom_b))
      .map(0xC000, Box::new(basic_rom))
      .map(0xE000, Box::new(editor_rom))
      .map(0xE810, Box::new(pia1))
      .map(0xE820, Box::new(pia2))
      .map(0xE840, Box::new(via))
      .map(0xF000, Box::new(kernel_rom));

    System::new(Box::new(memory), 1_000_000)
  }
}
