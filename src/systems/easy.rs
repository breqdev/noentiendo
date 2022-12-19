use crate::keyboard::KeyPosition;
use crate::memory::{ActiveInterrupt, BlockMemory, BranchMemory, Memory, SystemInfo};
use crate::platform::{Color, PlatformProvider, WindowConfig};
use crate::roms::RomFile;
use crate::system::System;
use crate::systems::SystemFactory;
use std::sync::Arc;

/// VRAM based around the Easy6502 display system from
/// https://skilldrick.github.io/easy6502/
/// This is a 32x32 pixel display with 16 colors. Writing a byte to an
/// address in the VRAM will set the color of the pixel at that address
/// to the color in the palette at the index of the byte.
struct EasyVram {
  width: u32,
  height: u32,
  data: Vec<u8>,
  platform: Arc<dyn PlatformProvider>,
  palette: Vec<Color>,
}

const SCALE: u32 = 8;

impl EasyVram {
  pub fn new(width: u32, height: u32, platform: Arc<dyn PlatformProvider>) -> Self {
    platform.request_window(WindowConfig::new(width, height, SCALE as f64));

    let palette = [
      0x000000, 0xffffff, 0x880000, 0xaaffee, 0xcc44cc, 0x00cc55, 0x0000aa, 0xeeee77, 0xdd8855,
      0x664400, 0xff7777, 0x333333, 0x777777, 0xaaff66, 0x0088ff, 0xbbbbbb,
    ];

    let palette = palette
      .iter()
      .map(|&c| Color::new((c >> 16) as u8, (c >> 8) as u8, c as u8))
      .collect();

    Self {
      width,
      height,
      data: vec![0; (width * height) as usize],
      platform,
      palette,
    }
  }
}

impl Memory for EasyVram {
  fn read(&mut self, address: u16) -> u8 {
    self.data[((address as u32) % (self.width * self.height)) as usize]
  }

  fn write(&mut self, address: u16, value: u8) {
    let index = ((address as u32) % (self.width * self.height)) as usize;
    self.data[index] = value;

    let x_base = (index % self.width as usize) as u32;
    let y_base = (index / self.width as usize) as u32;
    let color = self.palette[(self.data[index] as usize) % self.palette.len()];

    self.platform.set_pixel(x_base, y_base, color);
  }

  fn reset(&mut self) {
    for i in 0..self.data.len() {
      self.data[i] = 0;
    }
  }

  fn poll(&mut self, _info: &SystemInfo) -> ActiveInterrupt {
    ActiveInterrupt::None
  }
}

/// Memory-mapped I/O for the Easy6502 system.
/// https://skilldrick.github.io/easy6502/
/// Reading from address 0 returns a random number between 0 and 255,
/// and reading from address 1 returns the ASCII code of the key most recently
/// pressed. Writing to this memory does nothing.
struct EasyIO {
  platform: Arc<dyn PlatformProvider>,
}

impl EasyIO {
  pub fn new(platform: Arc<dyn PlatformProvider>) -> Self {
    Self { platform }
  }
}

impl Memory for EasyIO {
  fn read(&mut self, address: u16) -> u8 {
    match address % 2 {
      0 => self.platform.random(),
      _ => {
        let state = self.platform.get_key_state();

        if state.is_pressed(KeyPosition::W) {
          b'W'
        } else if state.is_pressed(KeyPosition::A) {
          b'A'
        } else if state.is_pressed(KeyPosition::S) {
          b'S'
        } else if state.is_pressed(KeyPosition::D) {
          b'D'
        } else {
          0
        }
      }
    }
  }

  fn write(&mut self, _address: u16, _value: u8) {}

  fn reset(&mut self) {}

  fn poll(&mut self, _info: &SystemInfo) -> ActiveInterrupt {
    ActiveInterrupt::None
  }
}

/// A port of the "Easy6502" system from
/// <https://skilldrick.github.io/easy6502/>
pub struct EasySystemFactory;

impl SystemFactory<RomFile, ()> for EasySystemFactory {
  fn create(rom: RomFile, _config: (), platform: Arc<dyn PlatformProvider>) -> System {
    let zero_page = BlockMemory::ram(0x0100);
    let io = EasyIO::new(platform.clone());
    let stack_ram = BlockMemory::ram(0x0100);
    let vram = EasyVram::new(32, 32, platform);
    let high_ram = BlockMemory::ram(0x7A00);
    let rom = BlockMemory::from_file(0x8000, rom);

    let memory = BranchMemory::new()
      .map(0x0000, Box::new(zero_page))
      .map(0x00fe, Box::new(io))
      .map(0x0100, Box::new(stack_ram))
      .map(0x0200, Box::new(vram))
      .map(0x0600, Box::new(high_ram))
      .map(0x8000, Box::new(rom));

    System::new(Box::new(memory), 20_000)
  }
}
