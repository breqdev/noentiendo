use std::sync::Arc;

use crate::{
  keyboard::KeyMappingStrategy,
  memory::{ActiveInterrupt, BlockMemory, BranchMemory, Memory, NullMemory, SystemInfo},
  platform::{Color, PlatformProvider, WindowConfig},
  roms::RomFile,
  system::System,
  systems::SystemFactory,
};

mod roms;

/// TODO: This is bad because the vram can be remapped!!
struct C64Vram {
  data: Vec<u8>,
  platform: Arc<dyn PlatformProvider>,
  character_rom: Vec<u8>,
  foreground: Color,
  background: Color,
}

const WIDTH: u32 = 40;
const HEIGHT: u32 = 25;
const CHAR_WIDTH: u32 = 8;
const CHAR_HEIGHT: u32 = 8;
const VRAM_SIZE: usize = 1024; // 24 extra bytes to make mapping easier

impl C64Vram {
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

impl Memory for C64Vram {
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

pub use roms::C64SystemRoms;

/// Configuration for a Commodore 64 system.
pub struct C64SystemConfig {
  pub mapping: KeyMappingStrategy,
}

/// The Commodore 64 system.
pub struct C64SystemFactory;

impl SystemFactory<C64SystemRoms, C64SystemConfig> for C64SystemFactory {
  fn create(
    roms: C64SystemRoms,
    config: C64SystemConfig,
    platform: Arc<dyn PlatformProvider>,
  ) -> System {
    let ram = BlockMemory::ram(0x0400);
    let vram = C64Vram::new(roms.character, platform.clone());
    let basic_ram = BlockMemory::ram(0x9800);
    let cartridge_low = NullMemory::new();
    let basic_rom = BlockMemory::from_file(0x2000, roms.basic);
    let high_ram = BlockMemory::ram(0x1000);
    let character_rom = NullMemory::new(); // BlockMemory::from_file(0x1000, roms.character);
    let kernal_rom = BlockMemory::from_file(0x2000, roms.kernal);

    let memory = BranchMemory::new()
      .map(0x0000, Box::new(ram))
      .map(0x0400, Box::new(vram))
      .map(0x0800, Box::new(basic_ram))
      .map(0x8000, Box::new(cartridge_low))
      .map(0xA000, Box::new(basic_rom))
      .map(0xC000, Box::new(high_ram))
      .map(0xD000, Box::new(character_rom))
      .map(0xE000, Box::new(kernal_rom));

    System::new(Box::new(memory), 1_000_000)
  }
}
