use crate::execute::Execute;
use crate::fetch::Fetch;
use crate::memory::Memory;
use crate::registers::{ProgramCounter, Registers, StackPointer};

pub mod vectors {
  pub const RESET: u16 = 0xFFFC;
  pub const NMI: u16 = 0xFFFA;
  pub const IRQ: u16 = 0xFFFE;
}

pub struct System {
  pub registers: Registers,
  memory: Box<dyn Memory>,
}

pub trait MemoryIO {
  fn read(&self, address: u16) -> u8;
  fn write(&mut self, address: u16, value: u8);
  fn read_word(&self, address: u16) -> u16;
  fn write_word(&mut self, address: u16, value: u16);
}

impl MemoryIO for System {
  fn read(&self, address: u16) -> u8 {
    self.memory.read(address)
  }

  fn read_word(&self, address: u16) -> u16 {
    let lo = self.memory.read(address);
    let hi = self.memory.read(address + 1);
    (hi as u16) << 8 | lo as u16
  }

  fn write(&mut self, address: u16, value: u8) {
    self.memory.write(address, value)
  }

  fn write_word(&mut self, address: u16, value: u16) {
    self.memory.write(address, value as u8);
    self.memory.write(address + 1, (value >> 8) as u8)
  }
}

pub trait Stack {
  fn push(&mut self, value: u8);
  fn pop(&mut self) -> u8;

  fn push_word(&mut self, value: u16);
  fn pop_word(&mut self) -> u16;
}

impl Stack for System {
  fn push(&mut self, value: u8) {
    self.registers.stack_push();
    self.write(self.registers.stack_address(), value)
  }

  fn pop(&mut self) -> u8 {
    let value = self.read(self.registers.stack_address());
    self.registers.stack_pop();
    value
  }

  fn push_word(&mut self, value: u16) {
    self.push((value & 0xFF) as u8);
    self.push((value >> 8) as u8);
  }

  fn pop_word(&mut self) -> u16 {
    let hi = self.pop();
    let lo = self.pop();
    (hi as u16) << 8 | lo as u16
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
    let dest = self.read_word(vectors::RESET);
    self.registers.pc_load(dest);
  }

  pub fn tick(&mut self) {
    let opcode = self.fetch();

    self.execute(opcode).expect("Failed to execute instruction");
  }
}
