use crate::registers::ProgramCounter;
use crate::system::{MemoryIO, System};

pub trait Fetch {
  // Fetch immediate values
  fn fetch(&mut self) -> u8;
  fn fetch_word(&mut self) -> u16;

  // Fetch an 8-bit pointer and return the value at that address
  fn fetch_zero_page(&mut self) -> u8;
  fn fetch_zero_page_x(&mut self) -> u8;
  fn fetch_zero_page_y(&mut self) -> u8;

  // Fetch a 16-bit pointer and return the value at that address
  fn fetch_absolute(&mut self) -> u8;
  fn fetch_absolute_x(&mut self) -> u8;
  fn fetch_absolute_y(&mut self) -> u8;

  // Fetch a 16-bit pointer by adding the X register to the instruction argument
  // and return the value at that address
  fn fetch_indirect_x(&mut self) -> u8;

  // Fetch a 16-bit pointer, add the Y register to it, and return the value at
  // that address
  fn fetch_indirect_y(&mut self) -> u8;

  // Fetch operand value based on the opcode
  fn fetch_operand_value(&mut self, opcode: u8) -> u8;

  // Fetch operand address based on the opcode
  fn fetch_operand_address(&mut self, opcode: u8) -> u16;
}

impl Fetch for System {
  fn fetch(&mut self) -> u8 {
    let result = self.read(self.registers.pc_address());
    self.registers.pc_increment();
    result
  }

  fn fetch_word(&mut self) -> u16 {
    let lo = self.fetch();
    let hi = self.fetch();
    (hi as u16) << 8 | lo as u16
  }

  fn fetch_zero_page(&mut self) -> u8 {
    let address = self.fetch() as u16;
    let result = self.read(address);
    result
  }

  fn fetch_zero_page_x(&mut self) -> u8 {
    let address = self.fetch();
    let result = self.read((address + self.registers.x_index) as u16);
    result
  }

  fn fetch_zero_page_y(&mut self) -> u8 {
    let address = self.fetch();
    let result = self.read((address + self.registers.y_index) as u16);
    result
  }

  fn fetch_absolute(&mut self) -> u8 {
    let address = self.fetch_word();
    let result = self.read(address);
    result
  }

  fn fetch_absolute_x(&mut self) -> u8 {
    let address = self.fetch_word();
    let result = self.read(address + self.registers.x_index as u16);
    result
  }

  fn fetch_absolute_y(&mut self) -> u8 {
    let address = self.fetch_word();
    let result = self.read(address + self.registers.y_index as u16);
    result
  }

  fn fetch_indirect_x(&mut self) -> u8 {
    let base = self.fetch();
    let address = (base + self.registers.x_index) as u16;
    let pointer = self.read_word(address);
    let result = self.read(pointer);
    result
  }

  fn fetch_indirect_y(&mut self) -> u8 {
    let base = self.fetch();
    let address = self.read_word(base as u16);
    let result = self.read(address + self.registers.y_index as u16);
    result
  }

  fn fetch_operand_value(&mut self, opcode: u8) -> u8 {
    match opcode & 0x1F {
      0x00 | 0x02 | 0x09 => self.fetch(),
      0x01 => self.fetch_indirect_x(),
      0x04 | 0x05 | 0x06 => self.fetch_zero_page(),
      0x0C | 0x0D | 0x0E => self.fetch_absolute(),
      0x11 => self.fetch_indirect_y(),
      0x14 | 0x15 => self.fetch_zero_page_x(),
      0x16 => self.fetch_zero_page_y(),
      0x19 | 0x1E => self.fetch_absolute_y(),
      0x1C | 0x1D => self.fetch_absolute_x(),
      _ => unreachable!(),
    }
  }

  fn fetch_operand_address(&mut self, opcode: u8) -> u16 {
    match opcode & 0x1F {
      0x01 => {
        let base = self.fetch();
        let pointer = (base + self.registers.x_index) as u16;
        self.read_word(pointer)
      }
      0x04 => self.fetch() as u16,
      0x05 => self.fetch() as u16,
      0x06 => self.fetch() as u16,
      0x0C => self.fetch_word(),
      0x0D => self.fetch_word(),
      0x0E => self.fetch_word(),
      0x11 => {
        let base = self.fetch();
        let pointer = self.read_word(base as u16);
        pointer + self.registers.y_index as u16
      }
      0x14 => {
        let base = self.fetch();
        (base + self.registers.x_index) as u16
      }
      0x15 => {
        let base = self.fetch();
        (base + self.registers.x_index) as u16
      }
      0x16 => {
        let base = self.fetch();
        (base + self.registers.x_index) as u16
      }
      0x19 => {
        let base = self.fetch_word();
        base + self.registers.y_index as u16
      }
      0x1D => {
        let base = self.fetch_word();
        base + self.registers.x_index as u16
      }
      0x1E => {
        let base = self.fetch_word();
        base + self.registers.x_index as u16
      }
      _ => unreachable!(),
    }
  }
}
