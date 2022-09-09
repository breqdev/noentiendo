use crate::isomorphic::{readline, writeline};
use crate::memory::{ActiveInterrupt, Memory, SystemInfo};
use crate::memory::{BlockMemory, BranchMemory, RomFile};
use crate::platform::PlatformProvider;
use crate::system::System;
use crate::systems::SystemFactory;
use std::sync::Arc;

struct MappedStdIO {}

impl MappedStdIO {
  pub fn new() -> Self {
    Self {}
  }
}

impl Memory for MappedStdIO {
  // 0x00: u8 as dec
  // 0x01: char
  // 0x02: u8 as hex
  fn read(&mut self, address: u16) -> u8 {
    let input = readline();

    match address & 0x03 {
      0x00 => input.trim().parse().expect("Invalid input for u8"),
      0x01 => {
        let char = input.chars().next().expect("String is empty");
        ((char as u32) & 0xFF) as u8
      }
      0x02 => u8::from_str_radix(&input.trim(), 16).expect("Invalid input for u8"),
      0x03 => panic!("Invalid address for MappedStdIO"),
      _ => unreachable!("Invalid address"),
    }
  }

  fn write(&mut self, address: u16, value: u8) {
    match address & 0x03 {
      0x00 => writeline(&format!("{}", value)),
      0x01 => writeline(&format!("{}", value as char)),
      0x02 => writeline(&format!("{:02X}", value)),
      0x03 => {
        // print!("{}", value as char);
        // std::io::stdout().flush().unwrap();
      }
      _ => unreachable!(),
    }
  }

  fn reset(&mut self) {}

  fn poll(&mut self, _info: &SystemInfo) -> ActiveInterrupt {
    ActiveInterrupt::None
  }
}

pub struct BrookeSystemFactory {}

impl SystemFactory<RomFile> for BrookeSystemFactory {
  fn create(rom: RomFile, _graphics: Arc<dyn PlatformProvider>) -> System {
    let ram = BlockMemory::ram(0x4000);
    let io = MappedStdIO::new();
    let rom = BlockMemory::from_file(0x8000, rom);

    let memory = BranchMemory::new()
      .map(0x0000, Box::new(ram))
      .map(0x4000, Box::new(io))
      .map(0x8000, Box::new(rom));

    System::new(Box::new(memory), 0)
  }
}
