use instant::Duration;

use crate::cpu::{Mos6502, Mos6502Variant};
use crate::memory::{ActiveInterrupt, Memory, SystemInfo};
use crate::memory::{BlockMemory, BranchMemory};
use crate::platform::{PlatformProvider, WindowConfig};
use crate::roms::RomFile;
use crate::systems::{System, SystemBuilder};
use std::io::Write;
use std::sync::Arc;

/// A Memory implementation that can be used to read from or write to
/// STDIN/STDOUT.
struct MappedStdIO {
  provider: Arc<dyn PlatformProvider>,
}

impl MappedStdIO {
  pub fn new(provider: Arc<dyn PlatformProvider>) -> Self {
    Self { provider }
  }
}

impl Memory for MappedStdIO {
  /// Read from STDIN. The mode is controlled by the address.
  /// 0x00: u8 as dec
  /// 0x01: char
  /// 0x02: u8 as hex
  fn read(&mut self, address: u16) -> u8 {
    let input = self.provider.input();

    match address & 0x03 {
      0x00 => input.trim().parse().expect("Invalid input for u8"),
      0x01 => {
        let char = input.chars().next().expect("String is empty");
        ((char as u32) & 0xFF) as u8
      }
      0x02 => u8::from_str_radix(input.trim(), 16).expect("Invalid input for u8"),
      0x03 => panic!("Invalid address for MappedStdIO"),
      _ => unreachable!("Invalid address"),
    }
  }

  /// Write to STDOUT. The mode is controlled by the address.
  /// 0x00: u8 as dec
  /// 0x01: char
  /// 0x02: u8 as hex
  fn write(&mut self, address: u16, value: u8) {
    match address & 0x03 {
      0x00 => self.provider.print(&format!("{value}\n")),
      0x01 => self.provider.print(&format!("{}\n", value as char)),
      0x02 => self.provider.print(&format!("{value:02X}\n")),
      0x03 => {
        print!("{}", value as char);
        std::io::stdout().flush().unwrap();
      }
      _ => unreachable!(),
    }
  }

  fn reset(&mut self) {}

  fn poll(&mut self, _cycles: u32, _info: &SystemInfo) -> ActiveInterrupt {
    ActiveInterrupt::None
  }
}

/// A factory for creating a BasicSystem.
pub struct BasicSystemBuilder;

impl SystemBuilder<BasicSystem, RomFile, ()> for BasicSystemBuilder {
  fn build(rom: RomFile, _config: (), platform: Arc<dyn PlatformProvider>) -> Box<dyn System> {
    let ram = BlockMemory::ram(0x4000);
    let io = MappedStdIO::new(platform);
    let rom = BlockMemory::from_file(0x8000, rom);

    let memory = BranchMemory::new()
      .map(0x0000, ram)
      .map(0x4000, io)
      .map(0x8000, rom);

    let cpu = Mos6502::new(memory, Mos6502Variant::NMOS);

    Box::new(BasicSystem { cpu })
  }
}

/// A system which only operates in text mode, for basic testing.
pub struct BasicSystem {
  cpu: Mos6502,
}

impl System for BasicSystem {
  fn tick(&mut self) -> Duration {
    Duration::from_secs_f64(1.0 / 20_000.0) * self.cpu.tick().into()
  }

  fn reset(&mut self) {
    self.cpu.reset();
  }

  fn render(&mut self, _framebuffer: &mut [u8], _config: WindowConfig) {}
}
