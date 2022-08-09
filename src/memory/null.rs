use crate::memory::{ActiveInterrupt, Memory};

pub struct NullMemory {}

impl NullMemory {
  pub fn new() -> Self {
    Self {}
  }
}

impl Memory for NullMemory {
  fn read(&mut self, _address: u16) -> u8 {
    0
  }

  fn write(&mut self, _address: u16, _value: u8) {}

  fn reset(&mut self) {}

  fn poll(&mut self) -> ActiveInterrupt {
    ActiveInterrupt::None
  }
}
