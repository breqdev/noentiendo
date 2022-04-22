use crate::memory::Memory;
use crate::registers::{ProgramCounter, Registers, StackPointer};

mod vectors {
  pub const RESET: u16 = 0xFFFC;
  pub const NMI: u16 = 0xFFFA;
  pub const IRQ: u16 = 0xFFFE;
}

pub struct System {
  registers: Registers,
  memory: Box<dyn Memory>,
}

trait Stack {
  fn push(&mut self, value: u8) -> Result<(), ()>;
  fn pop(&mut self) -> Result<u8, ()>;
}

impl Stack for System {
  fn push(&mut self, value: u8) -> Result<(), ()> {
    self.registers.stack_push();
    self.memory.write(self.registers.stack_address(), value)
  }

  fn pop(&mut self) -> Result<u8, ()> {
    let value = self.memory.read(self.registers.stack_address());
    self.registers.stack_pop();
    value
  }
}

trait Fetch {
  fn fetch(&mut self) -> Result<u8, ()>;
  fn fetch_word(&mut self) -> Result<u16, ()>;
}

impl Fetch for System {
  fn fetch(&mut self) -> Result<u8, ()> {
    let result = self.memory.read(self.registers.pc_address());
    self.registers.pc_increment();
    result
  }

  fn fetch_word(&mut self) -> Result<u16, ()> {
    let lo = self.fetch()?;
    let hi = self.fetch()?;
    Ok((hi as u16) << 8 | lo as u16)
  }
}

trait Execute {
  fn execute(&mut self, opcode: u8) -> Result<(), ()>;
}

impl Execute for System {
  fn execute(&mut self, opcode: u8) -> Result<(), ()> {
    match opcode {
      0xA9 => {
        let value = self.fetch()?;
        self.registers.accumulator = value;
        Ok(())
      }
      0x8D => {
        let address = self.fetch_word()?;
        self.memory.write(address, self.registers.accumulator)?;
        Ok(())
      }
      0x4C => {
        let address = self.fetch_word()?;
        self.registers.pc_load(address);
        Ok(())
      }
      _ => {
        println!("Unimplemented opcode: {:02X}", opcode);
        Err(())
      }
    }
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

    let pc_low = self.memory.read(vectors::RESET).unwrap();
    let pc_high = self.memory.read(vectors::RESET + 1).unwrap();

    self
      .registers
      .pc_load((pc_high as u16) << 8 | pc_low as u16);
  }

  pub fn tick(&mut self) {
    let opcode = self.fetch().expect("Failed to fetch instruction opcode");

    self.execute(opcode).expect("Failed to execute instruction");
  }

  pub fn read(&self, address: u16) -> Result<u8, ()> {
    self.memory.read(address)
  }

  pub fn write(&mut self, address: u16, value: u8) -> Result<(), ()> {
    self.memory.write(address, value)
  }
}
