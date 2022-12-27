use crate::cpu::fetch::Fetch;
use crate::cpu::registers::{flags, Alu};
use crate::cpu::{InterruptHandler, MemoryIO, Stack, System};

pub trait Execute {
  /// Execute the given opcode, returning either the number of cycles used or an error.
  fn execute(&mut self, opcode: u8) -> Result<u8, ()>;
}

impl Execute for System {
  fn execute(&mut self, opcode: u8) -> Result<u8, ()> {
    match opcode {
      // === LOAD ===
      0xA1 | 0xA5 | 0xA9 | 0xAD | 0xB1 | 0xB5 | 0xB9 | 0xBD => {
        // LDA
        let (value, cycles) = self.fetch_operand_value(opcode);
        self.registers.a = value;
        self.registers.sr.set_nz(value);
        Ok(cycles)
      }

      0xA2 | 0xA6 | 0xAE | 0xB6 | 0xBE => {
        // LDX
        let (value, cycles) = self.fetch_operand_value(opcode);
        self.registers.x = value;
        self.registers.sr.set_nz(value);
        Ok(cycles)
      }

      0xA0 | 0xA4 | 0xAC | 0xB4 | 0xBC => {
        // LDY
        let (value, cycles) = self.fetch_operand_value(opcode);
        self.registers.y = value;
        self.registers.sr.set_nz(value);
        Ok(cycles)
      }

      // === STORE ===
      0x81 | 0x85 | 0x8D | 0x91 | 0x95 | 0x99 | 0x9D => {
        // STA
        let (address, cycles) = self.fetch_operand_address(opcode);
        self.write(address, self.registers.a);
        Ok(cycles)
      }

      // STX
      0x86 | 0x8E | 0x96 => {
        let (address, cycles) = self.fetch_operand_address(opcode);
        self.write(address, self.registers.x);
        Ok(cycles)
      }

      // STY
      0x84 | 0x8C | 0x94 => {
        let (address, cycles) = self.fetch_operand_address(opcode);
        self.write(address, self.registers.y);
        Ok(cycles)
      }

      // === TRANSFER ===
      0xAA => {
        // TAX
        self.registers.x = self.registers.a;
        self.registers.sr.set_nz(self.registers.a);
        Ok(2)
      }
      0xA8 => {
        // TAY
        self.registers.y = self.registers.a;
        self.registers.sr.set_nz(self.registers.a);
        Ok(2)
      }
      0xBA => {
        // TSX
        self.registers.x = self.registers.sp.get();
        self.registers.sr.set_nz(self.registers.sp.get());
        Ok(2)
      }
      0x8A => {
        // TXA
        self.registers.a = self.registers.x;
        self.registers.sr.set_nz(self.registers.x);
        Ok(2)
      }
      0x9A => {
        // TXS
        self.registers.sp.set(self.registers.x);
        Ok(2)
      }
      0x98 => {
        // TYA
        self.registers.a = self.registers.y;
        self.registers.sr.set_nz(self.registers.y);
        Ok(2)
      }

      // === STACK ===
      0x48 => {
        // PHA
        self.push(self.registers.a);
        Ok(3)
      }
      0x08 => {
        // PHP
        self.push(self.registers.sr.get() | flags::BREAK);
        Ok(3)
      }
      0x68 => {
        // PLA
        let value = self.pop();
        self.registers.a = value;
        self.registers.sr.set_nz(value);
        Ok(4)
      }
      0x28 => {
        // PLP
        let status = self.pop();
        self.registers.sr.load(status);
        Ok(4)
      }

      // === SHIFT ===
      0x0A => {
        // ASL a
        let value = self.registers.a;
        self.registers.a = value << 1;

        self.registers.sr.write(flags::CARRY, value & 0x80 != 0);
        self.registers.sr.set_nz(self.registers.a);
        Ok(2)
      }
      0x06 | 0x0E | 0x16 | 0x1E => {
        // ASL
        let (address, cycles) = self.fetch_operand_address(opcode);
        let value = self.read(address);
        let result = value << 1;

        self.registers.sr.write(flags::CARRY, value & 0x80 != 0);
        self.registers.sr.set_nz(result);
        self.write(address, result);
        Ok(cycles + 2)
      }

      0x4A => {
        // LSR a
        let value = self.registers.a;
        self.registers.a = value >> 1;

        self.registers.sr.write(flags::CARRY, value & 0x01 != 0);
        self.registers.sr.set_nz(self.registers.a);
        Ok(2)
      }
      0x46 | 0x4E | 0x56 | 0x5E => {
        // LSR
        let (address, cycles) = self.fetch_operand_address(opcode);
        let value = self.read(address);
        let result = value >> 1;

        self.registers.sr.write(flags::CARRY, value & 0x01 != 0);
        self.registers.sr.set_nz(result);
        self.write(address, result);
        Ok(cycles + 2)
      }

      0x2A => {
        // ROL a
        let value = self.registers.a;
        let result = (value << 1) | (self.registers.sr.read(flags::CARRY) as u8);

        self.registers.sr.write(flags::CARRY, value & 0x80 != 0);
        self.registers.sr.set_nz(result);
        self.registers.a = result;
        Ok(2)
      }
      0x26 | 0x2E | 0x36 | 0x3E => {
        // ROL
        let (address, cycles) = self.fetch_operand_address(opcode);
        let value = self.read(address);
        let result = (value << 1) | (self.registers.sr.read(flags::CARRY) as u8);

        self.registers.sr.write(flags::CARRY, value & 0x80 != 0);
        self.registers.sr.set_nz(result);
        self.write(address, result);
        Ok(cycles + 2)
      }

      0x6A => {
        // ROR a
        let value = self.registers.a;
        let result = (value >> 1) | (self.registers.sr.read(flags::CARRY) as u8) << 7;

        self.registers.sr.write(flags::CARRY, value & 0x01 != 0);
        self.registers.sr.set_nz(result);
        self.registers.a = result;
        Ok(2)
      }
      0x66 | 0x6E | 0x76 | 0x7E => {
        // ROR
        let (address, cycles) = self.fetch_operand_address(opcode);
        let value = self.read(address);
        let result = value >> 1 | (self.registers.sr.read(flags::CARRY) as u8) << 7;

        self.registers.sr.write(flags::CARRY, value & 0x01 != 0);
        self.registers.sr.set_nz(result);
        self.write(address, result);
        Ok(cycles)
      }

      // === LOGIC ===
      0x21 | 0x25 | 0x29 | 0x2D | 0x31 | 0x35 | 0x39 | 0x3D => {
        // AND
        let (value, cycles) = self.fetch_operand_value(opcode);
        self.registers.a &= value;
        self.registers.sr.set_nz(self.registers.a);
        Ok(cycles)
      }

      0x24 | 0x2C => {
        // BIT
        let (value, cycles) = self.fetch_operand_value(opcode);
        self.registers.sr.write(flags::NEGATIVE, value & 0x80 != 0);
        self.registers.sr.write(flags::OVERFLOW, value & 0x40 != 0);
        self
          .registers
          .sr
          .write(flags::ZERO, value & self.registers.a == 0);
        Ok(cycles)
      }

      0x41 | 0x45 | 0x49 | 0x4D | 0x51 | 0x55 | 0x59 | 0x5D => {
        // EOR
        let (value, cycles) = self.fetch_operand_value(opcode);
        self.registers.a ^= value;
        self.registers.sr.set_nz(self.registers.a);
        Ok(cycles)
      }

      0x01 | 0x05 | 0x09 | 0x0D | 0x11 | 0x15 | 0x19 | 0x1D => {
        // ORA
        let (value, cycles) = self.fetch_operand_value(opcode);
        self.registers.a |= value;
        self.registers.sr.set_nz(self.registers.a);
        Ok(cycles)
      }

      // === ARITHMETIC ===
      0x61 | 0x65 | 0x69 | 0x6D | 0x71 | 0x75 | 0x79 | 0x7D => {
        // ADC
        let (value, cycles) = self.fetch_operand_value(opcode);
        self.registers.alu_add(value);
        Ok(cycles)
      }

      0xC1 | 0xC5 | 0xC9 | 0xCD | 0xD1 | 0xD5 | 0xD9 | 0xDD => {
        // CMP
        let (value, cycles) = self.fetch_operand_value(opcode);
        self.registers.alu_compare(self.registers.a, value);
        Ok(cycles)
      }

      0xE0 | 0xE4 | 0xEC => {
        // CPX
        let (value, cycles) = self.fetch_operand_value(opcode);
        self.registers.alu_compare(self.registers.x, value);
        Ok(cycles)
      }

      0xC0 | 0xC4 | 0xCC => {
        // CPY
        let (value, cycles) = self.fetch_operand_value(opcode);
        self.registers.alu_compare(self.registers.y, value);
        Ok(cycles)
      }

      0xE1 | 0xE5 | 0xE9 | 0xED | 0xF1 | 0xF5 | 0xF9 | 0xFD => {
        // SBC
        let (value, cycles) = self.fetch_operand_value(opcode);
        self.registers.alu_subtract(value);
        Ok(cycles)
      }

      // === INCREMENT ===
      0xC6 | 0xCE | 0xD6 | 0xDE => {
        // DEC
        let (address, cycles) = self.fetch_operand_address(opcode);
        let value = self.read(address);
        let result = value.wrapping_sub(1);
        self.registers.sr.set_nz(result);
        self.write(address, result);
        Ok(cycles + 2)
      }

      0xCA => {
        // DEX
        self.registers.x = self.registers.x.wrapping_sub(1);
        self.registers.sr.set_nz(self.registers.x);
        Ok(2)
      }

      0x88 => {
        // DEY
        self.registers.y = self.registers.y.wrapping_sub(1);
        self.registers.sr.set_nz(self.registers.y);
        Ok(2)
      }

      0xE6 | 0xEE | 0xF6 | 0xFE => {
        // INC
        let (address, cycles) = self.fetch_operand_address(opcode);
        let value = self.read(address);
        let result = value.wrapping_add(1);
        self.registers.sr.set_nz(result);
        self.write(address, result);
        Ok(cycles + 2)
      }

      0xE8 => {
        // INX
        self.registers.x = self.registers.x.wrapping_add(1);
        self.registers.sr.set_nz(self.registers.x);
        Ok(2)
      }

      0xC8 => {
        // INY
        self.registers.y = self.registers.y.wrapping_add(1);
        self.registers.sr.set_nz(self.registers.y);
        Ok(2)
      }

      // === CONTROL ===
      0x00 => {
        // BRK
        self.registers.pc.increment();
        self.interrupt(true, true);
        Ok(7)
      }
      0x4C | 0x6C => {
        // JMP
        let (address, cycles) = match opcode {
          0x4C => (self.fetch_word(), 3),
          0x6C => {
            let indirect = self.fetch_word();
            (self.read_word(indirect), 5)
          }
          _ => unreachable!(),
        };

        self.registers.pc.load(address);
        Ok(cycles)
      }
      0x20 => {
        // JSR absolute
        let address = self.fetch_word();
        self.push_word(self.registers.pc.address().wrapping_sub(1));

        self.registers.pc.load(address);
        Ok(6)
      }
      0x40 => {
        // RTI
        let status = self.pop();
        self.registers.sr.load(status);
        let dest = self.pop_word();
        self.registers.pc.load(dest);
        Ok(6)
      }
      0x60 => {
        // RTS
        let dest = self.pop_word().wrapping_add(1);
        self.registers.pc.load(dest);
        Ok(6)
      }

      // === BRANCH ===
      0x90 | 0xB0 | 0xF0 | 0x30 | 0xD0 | 0x10 | 0x50 | 0x70 => {
        let offset = self.fetch() as i8;

        let condition = match opcode {
          0x90 => !self.registers.sr.read(flags::CARRY),   // BCC
          0xB0 => self.registers.sr.read(flags::CARRY),    // BCS
          0xF0 => self.registers.sr.read(flags::ZERO),     // BEQ
          0x30 => self.registers.sr.read(flags::NEGATIVE), // BMI
          0xD0 => !self.registers.sr.read(flags::ZERO),    // BNE
          0x10 => !self.registers.sr.read(flags::NEGATIVE), // BPL
          0x50 => !self.registers.sr.read(flags::OVERFLOW), // BVC
          0x70 => self.registers.sr.read(flags::OVERFLOW), // BVS
          _ => unreachable!(),
        };

        if condition {
          self.registers.pc.offset(offset);
          Ok(3)
        } else {
          Ok(2)
        }
      }

      // === FLAGS ===
      0x18 | 0xD8 | 0x58 | 0xB8 => {
        self.registers.sr.clear(match opcode {
          0x18 => flags::CARRY,     // CLC
          0xD8 => flags::DECIMAL,   // CLD
          0x58 => flags::INTERRUPT, // CLI
          0xB8 => flags::OVERFLOW,  // CLV
          _ => unreachable!(),
        });

        Ok(2)
      }

      0x38 | 0xF8 | 0x78 => {
        self.registers.sr.set(match opcode {
          0x38 => flags::CARRY,     // SEC
          0xF8 => flags::DECIMAL,   // SED
          0x78 => flags::INTERRUPT, // SEI
          _ => unreachable!(),
        });

        Ok(2)
      }

      // === NOP ===
      0xEA => {
        // NOP
        Ok(2)
      }

      _ => {
        println!("Unimplemented opcode: {:02X}", opcode);
        Err(())
      }
    }
  }
}
