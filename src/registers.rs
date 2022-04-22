use crate::memory::Memory;

pub struct Registers {
  accumulator: u8,
  x_index: u8,
  y_index: u8,
  stack_pointer: u8,
  program_counter: u16,
  status_register: u8,
}

mod flags {
  pub const CARRY: u8 = 0b00000001;
  pub const ZERO: u8 = 0b00000010;
  pub const INTERRUPT: u8 = 0b00000100;
  pub const DECIMAL: u8 = 0b00001000;
  pub const BREAK: u8 = 0b00010000;
  pub const UNUSED: u8 = 0b00100000;
  pub const OVERFLOW: u8 = 0b01000000;
  pub const NEGATIVE: u8 = 0b10000000;
}

mod vectors {
  pub const RESET: u16 = 0xFFFC;
  pub const NMI: u16 = 0xFFFA;
  pub const IRQ: u16 = 0xFFFE;
}

trait Stack {
  fn push(&mut self, value: u8, memory: &mut dyn Memory);
  fn pop(&mut self, memory: &mut dyn Memory) -> u8;
}

impl Stack for Registers {
  fn push(&mut self, value: u8, memory: &mut dyn Memory) {
    self.stack_pointer -= 1;
    memory
      .write(0x0100 + self.stack_pointer as u16, value)
      .unwrap();
  }

  fn pop(&mut self, memory: &mut dyn Memory) -> u8 {
    let value = memory.read(0x0100 + self.stack_pointer as u16).unwrap();
    self.stack_pointer += 1;
    value
  }
}

pub trait Fetch {
  fn fetch(&mut self, memory: &mut dyn Memory) -> Result<u8, ()>;
  fn fetch_word(&mut self, memory: &mut dyn Memory) -> Result<u16, ()>;
}

impl Fetch for Registers {
  fn fetch(&mut self, memory: &mut dyn Memory) -> Result<u8, ()> {
    let result = memory.read(self.program_counter);
    self.program_counter += 1;
    result
  }

  fn fetch_word(&mut self, memory: &mut dyn Memory) -> Result<u16, ()> {
    let lo = self.fetch(memory)?;
    let hi = self.fetch(memory)?;
    Ok((hi as u16) << 8 | lo as u16)
  }
}

pub trait Execute {
  fn execute(&mut self, opcode: u8, memory: &mut dyn Memory) -> Result<(), ()>;
}

impl Execute for Registers {
  fn execute(&mut self, opcode: u8, memory: &mut dyn Memory) -> Result<(), ()> {
    match opcode {
      0xA9 => {
        let value = self.fetch(memory)?;
        self.accumulator = value;
        Ok(())
      }
      0x8D => {
        let address = self.fetch_word(memory)?;
        memory.write(address, self.accumulator)?;
        Ok(())
      }
      _ => {
        println!("Unimplemented opcode: {:02X}", opcode);
        Err(())
      }
    }
  }
}

impl Registers {
  pub fn new() -> Registers {
    Registers {
      accumulator: 0,
      x_index: 0,
      y_index: 0,
      stack_pointer: 0xFF,
      program_counter: 0x0000,
      status_register: 0,
    }
  }

  pub fn reset(&mut self, memory: &mut dyn Memory) {
    self.accumulator = 0;
    self.x_index = 0;
    self.y_index = 0;
    self.stack_pointer = 0xFF;
    self.program_counter = 0x0000;
    self.status_register = 0;

    let pc_low = memory.read(vectors::RESET).unwrap();
    let pc_high = memory.read(vectors::RESET + 1).unwrap();

    self.program_counter = (pc_high as u16) << 8 | pc_low as u16;

    println!("pc = {:04X}", self.program_counter);
  }
}
