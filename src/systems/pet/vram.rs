use std::sync::Arc;

use crate::{
  memory::{ActiveInterrupt, Memory, SystemInfo},
  platform::{Color, PlatformProvider, WindowConfig},
  roms::RomFile,
};

const WIDTH: u32 = 40;
const HEIGHT: u32 = 25;
const CHAR_WIDTH: u32 = 8;
const CHAR_HEIGHT: u32 = 8;
const VRAM_SIZE: usize = 1024; // 24 extra bytes to make mapping easier

/// Commodore PET-style column screen memory.
/// This is a 40x25 character display with no color support.
/// Writing a character code to the screen memory will display that character
/// at the position corresponding to the address.
/// The characters are defined in a separate character ROM not accessible to
/// the rest of the system.
/// (see <https://www.chibiakumas.com/6502/platform4.php#LessonP38> for details)
/// Note that this emulates a 40-column pet, not an 80-column "Business" pet.
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
