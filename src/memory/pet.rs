use crate::graphics::{Color, GraphicsProvider, WindowConfig};
use crate::memory::{pia::Port, ActiveInterrupt, Memory};
use std::fs::File;
use std::io::Read;
use std::sync::Arc;

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

  fn poll(&mut self) -> ActiveInterrupt {
    ActiveInterrupt::None
  }
}

pub struct PetPia1PortA {
  keyboard_row: u8,
}

impl PetPia1PortA {
  pub fn new() -> Self {
    Self { keyboard_row: 0 }
  }
}

impl Port for PetPia1PortA {
  fn read(&mut self) -> u8 {
    0b1000_0000
    //^         diagnostic mode off
    // ^        IEEE488 (not implemented)
    //  ^^      Cassette sense (not implemented)
    //     ^^^^ Keyboard row select (not readable)
  }

  fn write(&mut self, value: u8) {
    self.keyboard_row = value & 0b1111;
  }

  fn reset(&mut self) {
    self.keyboard_row = 0;
  }
}

pub struct PetPia1PortB {}

impl PetPia1PortB {
  pub fn new() -> Self {
    Self {}
  }
}

impl Port for PetPia1PortB {
  fn read(&mut self) -> u8 {
    0b0000_0000 // contents of keyboard row
  }

  fn write(&mut self, value: u8) {}

  fn reset(&mut self) {}
}
