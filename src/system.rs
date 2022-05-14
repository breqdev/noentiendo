use crate::execute::Execute;
use crate::fetch::Fetch;
use crate::memory::Memory;
use crate::registers::{ProgramCounter, Registers, StackPointer};

mod vectors {
  pub const RESET: u16 = 0xFFFC;
  pub const NMI: u16 = 0xFFFA;
  pub const IRQ: u16 = 0xFFFE;
}

pub struct System {
  pub registers: Registers,
  memory: Box<dyn Memory>,
}

pub trait MemoryIO {
  fn read(&self, address: u16) -> Result<u8, ()>;
  fn write(&mut self, address: u16, value: u8) -> Result<(), ()>;
  fn read_word(&self, address: u16) -> Result<u16, ()>;
  fn write_word(&mut self, address: u16, value: u16) -> Result<(), ()>;
}

impl MemoryIO for System {
  fn read(&self, address: u16) -> Result<u8, ()> {
    self.memory.read(address)
  }

  fn read_word(&self, address: u16) -> Result<u16, ()> {
    let lo = self.memory.read(address)?;
    let hi = self.memory.read(address + 1)?;
    Ok((hi as u16) << 8 | lo as u16)
  }

  fn write(&mut self, address: u16, value: u8) -> Result<(), ()> {
    self.memory.write(address, value)
  }

  fn write_word(&mut self, address: u16, value: u16) -> Result<(), ()> {
    self.memory.write(address, value as u8)?;
    self.memory.write(address + 1, (value >> 8) as u8)
  }
}

pub trait Stack {
  fn push(&mut self, value: u8) -> Result<(), ()>;
  fn pop(&mut self) -> Result<u8, ()>;
}

impl Stack for System {
  fn push(&mut self, value: u8) -> Result<(), ()> {
    self.registers.stack_push();
    self.write(self.registers.stack_address(), value)
  }

  fn pop(&mut self) -> Result<u8, ()> {
    let value = self.read(self.registers.stack_address());
    self.registers.stack_pop();
    value
  }
}

impl System {
  pub fn new(memory: Box<dyn Memory>) -> System {
    System {
      registers: Registers::new(),
      memory,
    }
  }

  pub fn reset(&mut self) {
    self.registers.reset();

    let pc_low = self.read(vectors::RESET).unwrap();
    let pc_high = self.read(vectors::RESET + 1).unwrap();

    self
      .registers
      .pc_load((pc_high as u16) << 8 | pc_low as u16);
  }

  pub fn tick(&mut self) {
    let opcode = self.fetch().expect("Failed to fetch instruction opcode");

    self.execute(opcode).expect("Failed to execute instruction");
  }
}
