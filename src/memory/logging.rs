use super::{ActiveInterrupt, Memory, SystemInfo};

pub struct LoggingMemory {
  pub memory: Box<dyn Memory>,
}

impl LoggingMemory {
  pub fn new(memory: Box<dyn Memory>) -> Self {
    Self { memory }
  }
}

impl Memory for LoggingMemory {
  fn read(&mut self, address: u16) -> u8 {
    let value = self.memory.read(address);
    println!("READ  {:04X} -> {:02X}", address, value);
    value
  }

  fn write(&mut self, address: u16, value: u8) {
    println!("WRITE {:04X} <- {:02X}", address, value);
    self.memory.write(address, value);
  }

  fn reset(&mut self) {
    self.memory.reset();
  }

  fn poll(&mut self, cycles: u32, info: &SystemInfo) -> ActiveInterrupt {
    self.memory.poll(cycles, &info)
  }
}
