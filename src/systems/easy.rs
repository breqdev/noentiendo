use instant::Duration;

use crate::cpu::{MemoryIO, Mos6502};
use crate::keyboard::KeyPosition;
use crate::memory::{ActiveInterrupt, BlockMemory, BranchMemory, Memory, SystemInfo};
use crate::platform::{Color, PlatformProvider, WindowConfig};
use crate::roms::RomFile;
use crate::systems::{System, SystemBuilder};
use std::sync::Arc;

const WIDTH: u32 = 32;

/// VRAM based around the Easy6502 display system from
/// <https://skilldrick.github.io/easy6502/>.
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
/// <https://skilldrick.github.io/easy6502/>.
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

/// A factory for the Easy6502 system.
pub struct Easy6502SystemBuilder;

impl SystemBuilder<Easy6502System, RomFile, ()> for Easy6502SystemBuilder {
  fn build(rom: RomFile, _config: (), platform: Arc<dyn PlatformProvider>) -> Box<dyn System> {
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

    let cpu = Mos6502::new(Box::new(memory));

    Box::new(Easy6502System { cpu })
  }
}

/// A port of the "Easy6502" system from
/// <https://skilldrick.github.io/easy6502/>
pub struct Easy6502System {
  cpu: Mos6502,
}

impl System for Easy6502System {
  fn tick(&mut self) -> Duration {
    Duration::from_secs_f64(1.0 / 20_000.0) * self.cpu.tick().into()
  }

  fn reset(&mut self) {
    self.cpu.reset();
  }

  fn render(&mut self, framebuffer: &mut [u8], config: WindowConfig) {
    for y in 0..WIDTH {
      for x in 0..WIDTH {
        let index = (y * WIDTH + x) as u16;
        let color = self.cpu.read(0x0200 + index);

        let color = match color & 0x0F {
          0 => Color::new(0x00, 0x00, 0x00),
          1 => Color::new(0xFF, 0xFF, 0xFF),
          2 => Color::new(0x88, 0x00, 0x00),
          3 => Color::new(0xAA, 0xFF, 0xEE),
          4 => Color::new(0xCC, 0x44, 0xCC),
          5 => Color::new(0x00, 0xCC, 0x55),
          6 => Color::new(0x00, 0x00, 0xAA),
          7 => Color::new(0xEE, 0xEE, 0x77),
          8 => Color::new(0xDD, 0x88, 0x55),
          9 => Color::new(0x66, 0x44, 0x00),
          10 => Color::new(0xFF, 0x77, 0x77),
          11 => Color::new(0x33, 0x33, 0x33),
          12 => Color::new(0x77, 0x77, 0x77),
          13 => Color::new(0xAA, 0xFF, 0x66),
          14 => Color::new(0x00, 0x88, 0xFF),
          15 => Color::new(0xBB, 0xBB, 0xBB),
          _ => Color::new(0x00, 0x00, 0x00),
        };

        let index = ((y * config.width + x) * 4) as usize;
        let pixel = &mut framebuffer[index..(index + 4)];
        pixel.copy_from_slice(&color.to_rgba());
      }
    }
  }
}
