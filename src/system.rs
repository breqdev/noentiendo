use crate::execute::Execute;
use crate::fetch::Fetch;
use crate::memory::Memory;
use crate::registers::{flags, Registers};

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
    self.memory.write(address, value);
  }

  fn write_word(&mut self, address: u16, value: u16) {
    self.memory.write(address, value as u8);
    self.memory.write(address + 1, (value >> 8) as u8);
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
    self.registers.sp.push();
    self.write(self.registers.sp.address(), value);
  }

  fn pop(&mut self) -> u8 {
    let value = self.read(self.registers.sp.address());
    self.registers.sp.pop();
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

pub trait InterruptHandler {
  fn interrupt(&mut self, maskable: bool);
}

impl InterruptHandler for System {
  fn interrupt(&mut self, maskable: bool) {
    self.registers.sr.set(flags::INTERRUPT);
    self.push_word(self.registers.pc.address());
    self.push(self.registers.sr.get());

    let dest = match maskable {
      false => self.read_word(0xFFFA),
      true => self.read_word(0xFFFE),
    };

    self.registers.pc.load(dest);
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
    self.registers.pc.load(self.read_word(0xFFFC));
  }

  pub fn tick(&mut self) {
    self.memory.tick();

    let opcode = self.fetch();
    self.execute(opcode).expect("Failed to execute instruction");
  }
}
