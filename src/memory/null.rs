use crate::memory::{ActiveInterrupt, Memory, SystemInfo};

pub struct NullMemory {}

impl NullMemory {
  pub fn new() -> Self {
    Self {}
  }
}

impl Memory for NullMemory {
  fn read(&mut self, _address: u16) -> u8 {
    println!(
      "attempted to read from null memory at address {:04x}",
      _address
    );
    0
  }

  fn write(&mut self, _address: u16, _value: u8) {
    println!(
      "attempted to write to null memory at address {:04x}",
      _address
    );
  }

  fn reset(&mut self) {}

  fn poll(&mut self, _info: &SystemInfo) -> ActiveInterrupt {
    ActiveInterrupt::None
  }
}
