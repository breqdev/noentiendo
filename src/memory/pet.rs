use crate::graphics::{Color, GraphicsProvider, WindowConfig};
use crate::memory::Memory;
use std::fs::File;
use std::io::Read;
use std::sync::Arc;

const WIDTH: u32 = 40;
const HEIGHT: u32 = 25;
const CHAR_WIDTH: u32 = 8;
const CHAR_HEIGHT: u32 = 8;
const VRAM_SIZE: usize = 1024; // 24 extra bytes to make mapping easier

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
  fn read(&self, address: u16) -> u8 {
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

    let character = self.character_rom[character_index..(character_index + 8)].to_vec();

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

  fn tick(&mut self) {
    self.graphics.tick();
  }

  fn reset(&mut self) {
    for i in 0..self.data.len() {
      self.data[i] = 0;
    }

    for x in 0..(WIDTH * CHAR_WIDTH) {
      for y in 0..(HEIGHT * CHAR_HEIGHT) {
        self.graphics.set_pixel(x, y, self.background);
      }
    }
  }
}

pub struct PetIO {}

impl PetIO {
  pub fn new() -> Self {
    Self {}
  }
}

impl Memory for PetIO {
  fn read(&self, address: u16) -> u8 {
    match address % 0x100 {
      0x10 => 0xFF, // cassette sense
      0x12 => 0,    // keyboard row contents

      _ => 0,
    }
  }

  fn write(&mut self, address: u16, _value: u8) {
    match address & 0x100 {
      0x10 => {} // keyboard row select
      0x11 => {} // blank screen
      0x13 => {} // cassette motor

      _ => {}
    }
  }

  fn tick(&mut self) {}

  fn reset(&mut self) {}
}
