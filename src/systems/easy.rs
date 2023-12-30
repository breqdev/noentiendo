use instant::Duration;

use crate::cpu::{
  mos6502::{MemoryIO, Mos6502, Mos6502Variant},
  Cpu,
};
use crate::keyboard::KeyPosition;
use crate::memory::{ActiveInterrupt, BlockMemory, BranchMemory, Memory};
use crate::platform::{Color, PlatformProvider, WindowConfig};
use crate::roms::RomFile;
use crate::systems::{BuildableSystem, System};
use std::sync::Arc;

const WIDTH: u32 = 32;
const SCALE: u32 = 8;

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

  fn poll(&mut self, _cycles_since_poll: u64, _total_cycle_count: u64) -> ActiveInterrupt {
    ActiveInterrupt::None
  }
}

/// A factory for the Easy6502 system.
impl BuildableSystem<RomFile, ()> for Easy6502System {
  fn build(rom: RomFile, _config: (), platform: Arc<dyn PlatformProvider>) -> Box<dyn System> {
    platform.request_window(WindowConfig::new(WIDTH, WIDTH, SCALE as f64));

    let zero_page = BlockMemory::ram(0x0100);
    let io = EasyIO::new(platform.clone());
    let stack_ram = BlockMemory::ram(0x0100);
    let vram = BlockMemory::ram(0x0400);
    let high_ram = BlockMemory::ram(0x7A00);
    let rom = BlockMemory::from_file(0x8000, rom);

    let memory = BranchMemory::new()
      .map(0x0000, zero_page)
      .map(0x00fe, io)
      .map(0x0100, stack_ram)
      .map(0x0200, vram)
      .map(0x0600, high_ram)
      .map(0x8000, rom);

    let cpu = Mos6502::new(memory, Mos6502Variant::NMOS);

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
