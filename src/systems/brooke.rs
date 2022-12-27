use crate::cpu::Mos6502;
use crate::memory::{ActiveInterrupt, Memory, SystemInfo};
use crate::memory::{BlockMemory, BranchMemory};
use crate::platform::PlatformProvider;
use crate::roms::RomFile;
use crate::systems::System;
use std::io::Write;
use std::sync::Arc;

use super::SystemBuilder;

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
      0x00 => self.provider.print(&format!("{}\n", value)),
      0x01 => self.provider.print(&format!("{}\n", value as char)),
      0x02 => self.provider.print(&format!("{:02X}\n", value)),
      0x03 => {
        print!("{}", value as char);
        std::io::stdout().flush().unwrap();
      }
      _ => unreachable!(),
    }
  }

  fn reset(&mut self) {}

  fn poll(&mut self, _info: &SystemInfo) -> ActiveInterrupt {
    ActiveInterrupt::None
  }
}

/// A factory for creating a BrookeSystem.
pub struct BrookeSystemBuilder;

impl SystemBuilder<BrookeSystem, RomFile, ()> for BrookeSystemBuilder {
  fn build(rom: RomFile, _config: (), platform: Arc<dyn PlatformProvider>) -> Box<dyn System> {
    let ram = BlockMemory::ram(0x4000);
    let io = MappedStdIO::new(platform);
    let rom = BlockMemory::from_file(0x8000, rom);

    let memory = BranchMemory::new()
      .map(0x0000, Box::new(ram))
      .map(0x4000, Box::new(io))
      .map(0x8000, Box::new(rom));

    Mos6502::new(Box::new(memory));

    Box::new(BrookeSystem {})
  }
}

/// A system which only operates in text mode, for basic testing.
pub struct BrookeSystem;

impl System for BrookeSystem {
  fn tick(&mut self) -> instant::Duration {
    todo!()
  }

  fn reset(&mut self) {
    todo!()
  }

  fn render(&mut self, framebuffer: &mut [u8]) {
    todo!()
  }
}
