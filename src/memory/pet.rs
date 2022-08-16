use crate::graphics::{scancodes, Color, GraphicsProvider, WindowConfig};
use crate::memory::{pia::Port, ActiveInterrupt, Memory, SystemInfo};
use std::fs::File;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

const WIDTH: u32 = 40;
const HEIGHT: u32 = 25;
const CHAR_WIDTH: u32 = 8;
const CHAR_HEIGHT: u32 = 8;
const VRAM_SIZE: usize = 1024; // 24 extra bytes to make mapping easier

// Commodore PET-style column screen memory
// (see https://www.chibiakumas.com/6502/platform4.php#LessonP38 for details)

pub struct PetVram {
  data: Vec<u8>,
  graphics: Arc<dyn GraphicsProvider>,
  character_rom: Vec<u8>,
  foreground: Color,
  background: Color,
}

impl PetVram {
  pub fn new(rom_path: &str, graphics: Arc<dyn GraphicsProvider>) -> Self {
    let mut file = File::open(rom_path).unwrap();
    let mut character_rom = Vec::new();
    file.read_to_end(&mut character_rom).unwrap();

    graphics.configure_window(WindowConfig::new(
      WIDTH * CHAR_WIDTH,
      HEIGHT * CHAR_HEIGHT,
      2.0,
    ));

    Self {
      data: vec![0; VRAM_SIZE],
      graphics,
      character_rom,
      foreground: Color::new(255, 255, 255),
      background: Color::new(0, 0, 255),
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
          .graphics
          .set_pixel(column * CHAR_WIDTH + pixel, row * CHAR_HEIGHT + line, color);
      }
    }
  }

  fn reset(&mut self) {
    for i in 0..self.data.len() {
      self.data[i] = 0;
    }

    self.graphics.wait_for_pixels();

    for x in 0..(WIDTH * CHAR_WIDTH) {
      for y in 0..(HEIGHT * CHAR_HEIGHT) {
        self.graphics.set_pixel(x, y, self.background);
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
    match self.last_draw_instant {
      Some(last_draw) => {
        if last_draw.elapsed() > Duration::from_millis(17)
          && info.cycle_count > self.last_draw_cycle + (info.cycles_per_second / 16)
        {
          self.last_draw_cycle = info.cycle_count;
          self.last_draw_instant = None;
          true
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
  graphics: Arc<dyn GraphicsProvider>,
}

impl PetPia1PortB {
  pub fn new(keyboard_row: Arc<Mutex<u8>>, graphics: Arc<dyn GraphicsProvider>) -> Self {
    Self {
      keyboard_row,
      graphics,
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
    scancodes::LSHIFT,
    '@',
    ']',
    '_',
    '>',
    scancodes::RSHIFT,
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
      if self.graphics.is_pressed(row[i] as u8) {
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
