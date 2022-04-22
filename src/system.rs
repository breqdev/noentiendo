use crate::memory::Memory;
use crate::registers::{Execute, Fetch, Registers};

pub struct System {
  registers: Registers,
  memory: Box<dyn Memory>,
}

impl System {
  pub fn new(memory: Box<dyn Memory>) -> System {
    System {
      registers: Registers::new(),
      memory,
    }
  }

  pub fn reset(&mut self) {
    self.registers.reset(&mut *self.memory);
  }

  pub fn tick(&mut self) {
    let opcode = self
      .registers
      .fetch(&mut *self.memory)
      .expect("Failed to fetch instruction opcode");

    self
      .registers
      .execute(opcode, &mut *self.memory)
      .expect("Failed to execute instruction");
  }

  pub fn read(&self, address: u16) -> Result<u8, ()> {
    self.memory.read(address)
  }

  pub fn write(&mut self, address: u16, value: u8) -> Result<(), ()> {
    self.memory.write(address, value)
  }
}
