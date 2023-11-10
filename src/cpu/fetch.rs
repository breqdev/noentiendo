use crate::cpu::{MemoryIO, Mos6502};

/// Fetch values or addresses from memory, optionally dependent on the current
/// opcode.
pub trait Fetch {
  /// Fetch an immediate 1-byte value at the current program counter, and
  /// increment the program counter.
  fn fetch(&mut self) -> u8;

  /// Fetch an immediate 2-byte value (little-endian).
  fn fetch_word(&mut self) -> u16;

  /// Fetch the next operand value, based on the current opcode.
  /// Panics if the opcode is not valid for this operation.
  fn fetch_operand_value(&mut self, opcode: u8) -> (u8, u8);

  /// Fetch the next operand address, based on the current opcode.
  /// Panics if the opcode is not valid for this operation.
  fn fetch_operand_address(&mut self, opcode: u8) -> (u16, u8);
}

impl Fetch for Mos6502 {
  fn fetch(&mut self) -> u8 {
    let result = self.read(self.registers.pc.address());
    self.registers.pc.increment();
    result
  }

  fn fetch_word(&mut self) -> u16 {
    let lo = self.fetch();
    let hi = self.fetch();
    (hi as u16) << 8 | lo as u16
  }

  fn fetch_operand_value(&mut self, opcode: u8) -> (u8, u8) {
    match opcode & 0x1F {
      0x00 | 0x02 | 0x09 | 0x0B => (self.fetch(), 2), // Immediate
      0x08 | 0x18 | 0x1A => panic!("Implied operand has no value"),
      0x12 => panic!("Invalid opcode"),
      0x0A => (self.registers.a, 0),
      _ => {
        let (address, cycles) = self.fetch_operand_address(opcode);
        (self.read(address), cycles)
      }
    }
  }

  #[allow(clippy::manual_range_patterns)]
  fn fetch_operand_address(&mut self, opcode: u8) -> (u16, u8) {
    match opcode & 0x1F {
      0x00 | 0x02 | 0x09 | 0x0B => panic!("Immediate operand has no address"),
      0x01 | 0x03 => {
        // (Indirect,X)
        let base = self.fetch();
        let pointer = (base as u16 + self.registers.x as u16) & 0xFF;
        (self.read_word(pointer), 6)
      }
      0x04 | 0x05 | 0x06 | 0x07 => (self.fetch() as u16, 3), // Zero page
      0x08 | 0x0A | 0x18 | 0x1A => panic!("Implied operand has no address"),
      0x0C | 0x0D | 0x0E | 0x0F => (self.fetch_word(), 4), // Absolute
      0x10 => (self.fetch() as i8 as u16, 2),              // Relative
      0x11 | 0x13 => {
        // (Indirect),Y
        let base = self.fetch();
        let pointer = self.read_word(base as u16);
        (pointer + self.registers.y as u16, 5)
      }
      0x12 => panic!("Invalid opcode"),
      0x14 | 0x15 => {
        // Zero page,X
        let base = self.fetch();
        ((base as u16 + self.registers.x as u16) & 0xFF, 4)
      }
      0x16 | 0x17 => {
        // Zero page,X or Zero page,Y
        let base = self.fetch();
        if opcode & 0xC0 == 0x80 {
          ((base as u16 + self.registers.y as u16) & 0xFF, 5)
        } else {
          ((base as u16 + self.registers.x as u16) & 0xFF, 5)
        }
      }
      0x19 | 0x1B => {
        // Absolute,Y
        let base = self.fetch_word();
        (base + self.registers.y as u16, 4)
      }
      0x1C | 0x1D => {
        // Absolute,X
        let base = self.fetch_word();
        (base + self.registers.x as u16, 4)
      }
      0x1E | 0x1F => {
        // Absolute,X or Absolute,Y
        let base = self.fetch_word();
        if opcode & 0xC0 == 0x80 {
          (base + self.registers.y as u16, 4)
        } else {
          (base + self.registers.x as u16, 4)
        }
      }
      _ => unreachable!(),
    }
  }
}
