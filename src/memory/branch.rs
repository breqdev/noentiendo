use crate::memory::Memory;

pub struct BranchMemory {
  low: Box<dyn Memory>,
  high: Box<dyn Memory>,
  bits: u8,
}

impl Memory for BranchMemory {
  fn read(&self, address: u16) -> u8 {
    if address < (1 << (self.bits - 1)) {
      self.low.read(address)
    } else {
      self.high.read(address - (1 << (self.bits - 1)))
    }
  }

  fn write(&mut self, address: u16, value: u8) {
    if address < (1 << (self.bits - 1)) {
      self.low.write(address, value)
    } else {
      self.high.write(address - (1 << (self.bits - 1)), value)
    }
  }

  fn tick(&mut self) {
    self.low.tick();
    self.high.tick();
  }

  fn reset(&mut self) {
    self.low.reset();
    self.high.reset();
  }
}

impl BranchMemory {
  pub fn new(low: Box<dyn Memory>, high: Box<dyn Memory>, bits: u8) -> Self {
    Self { low, high, bits }
  }
}
