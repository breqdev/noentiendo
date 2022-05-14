use crate::fetch::Fetch;
use crate::registers::{flags, ProgramCounter, StatusRegister, ALU};
use crate::system::{MemoryIO, Stack, System};

pub trait Execute {
  fn execute(&mut self, opcode: u8) -> Result<(), ()>;
}

impl Execute for System {
  fn execute(&mut self, opcode: u8) -> Result<(), ()> {
    match opcode {
      // === LOAD ===
      0xA1 | 0xA5 | 0xA9 | 0xAD | 0xB1 | 0xB5 | 0xB9 | 0xBD => {
        // LDA
        let value = match opcode {
          0xA1 => self.fetch_indirect_x(),
          0xA5 => self.fetch_zero_page(),
          0xA9 => self.fetch(),
          0xAD => self.fetch_absolute(),
          0xB1 => self.fetch_indirect_y(),
          0xB5 => self.fetch_zero_page_x(),
          0xB9 => self.fetch_absolute_y(),
          0xBD => self.fetch_absolute_x(),
          _ => unreachable!(),
        };

        self.registers.accumulator = value;
        self.registers.status_set_nz(value);
        Ok(())
      }

      0xA2 | 0xA6 | 0xAE | 0xB6 | 0xBE => {
        // LDX
        let value = match opcode {
          0xA2 => self.fetch(),
          0xA6 => self.fetch_zero_page(),
          0xAE => self.fetch_absolute(),
          0xB6 => self.fetch_zero_page_y(),
          0xBE => self.fetch_absolute_y(),
          _ => unreachable!(),
        };

        self.registers.x_index = value;
        self.registers.status_set_nz(value);
        Ok(())
      }

      0xA0 | 0xA4 | 0xAC | 0xB4 | 0xBC => {
        // LDY
        let value = match opcode {
          0xA0 => self.fetch(),
          0xA4 => self.fetch_zero_page(),
          0xAC => self.fetch_absolute(),
          0xB4 => self.fetch_zero_page_x(),
          0xBC => self.fetch_absolute_x(),
          _ => unreachable!(),
        };

        self.registers.y_index = value;
        self.registers.status_set_nz(value);
        Ok(())
      }

      // === STORE ===
      0x81 | 0x85 | 0x8D | 0x91 | 0x95 | 0x99 | 0x9D => {
        // STA
        let address = match opcode {
          0x81 => {
            // STA (indirect, x)
            let base = self.fetch();
            let pointer = (base + self.registers.x_index) as u16;
            self.read_word(pointer)
          }
          0x85 => {
            // STA zero page
            self.fetch() as u16
          }
          0x8D => {
            // STA absolute
            self.fetch_word()
          }
          0x91 => {
            // STA (indirect), y
            let base = self.fetch();
            let pointer = self.read_word(base as u16);
            pointer + self.registers.y_index as u16
          }
          0x95 => {
            // STA zero page, x
            let base = self.fetch();
            (base + self.registers.x_index) as u16
          }
          0x99 => {
            // STA absolute, y
            let base = self.fetch_word();
            base + self.registers.y_index as u16
          }
          0x9D => {
            // STA absolute, x
            let base = self.fetch_word();
            base + self.registers.x_index as u16
          }
          _ => unreachable!(),
        };

        self.write(address, self.registers.accumulator);
        Ok(())
      }

      // STX
      0x86 | 0x8E | 0x96 => {
        let address = match opcode {
          0x86 => {
            // STX zero page
            self.fetch() as u16
          }
          0x8E => {
            // STX absolute
            self.fetch_word()
          }
          0x96 => {
            // STX zero page, y
            let base = self.fetch();
            (base + self.registers.y_index) as u16
          }
          _ => unreachable!(),
        };

        self.write(address, self.registers.x_index);
        Ok(())
      }

      // STY
      0x84 | 0x8C | 0x94 => {
        let address = match opcode {
          0x84 => {
            // STY zero page
            self.fetch() as u16
          }
          0x8C => {
            // STY absolute
            self.fetch_word()
          }
          0x94 => {
            // STY zero page, x
            let base = self.fetch();
            (base + self.registers.x_index) as u16
          }
          _ => unreachable!(),
        };

        self.write(address, self.registers.y_index);
        Ok(())
      }

      // === TRANSFER ===
      0xAA => {
        // TAX
        self.registers.x_index = self.registers.accumulator;
        self.registers.status_set_nz(self.registers.accumulator);
        Ok(())
      }
      0xA8 => {
        // TAY
        self.registers.y_index = self.registers.accumulator;
        self.registers.status_set_nz(self.registers.accumulator);
        Ok(())
      }
      0xBA => {
        // TSX
        self.registers.x_index = self.registers.stack_pointer;
        self.registers.status_set_nz(self.registers.stack_pointer);
        Ok(())
      }
      0x8A => {
        // TXA
        self.registers.accumulator = self.registers.x_index;
        self.registers.status_set_nz(self.registers.x_index);
        Ok(())
      }
      0x9A => {
        // TXS
        self.registers.stack_pointer = self.registers.x_index;
        Ok(())
      }
      0x98 => {
        // TYA
        self.registers.accumulator = self.registers.y_index;
        self.registers.status_set_nz(self.registers.y_index);
        Ok(())
      }

      // === STACK ===
      0x48 => {
        // PHA
        self.push(self.registers.accumulator);
        Ok(())
      }
      0x08 => {
        // PHP
        self.push(self.registers.status_register);
        Ok(())
      }
      0x68 => {
        // PLA
        self.registers.accumulator = self.pop();
        Ok(())
      }
      0x28 => {
        // PLP
        self.registers.status_register = self.pop();
        Ok(())
      }

      // === SHIFT ===
      0x06 | 0x0A | 0x0E | 0x16 | 0x1E => {
        // ASL
        let address = match opcode {
          0x06 => {
            // ASL zero page
            Some(self.fetch() as u16)
          }
          0x0A => {
            // ASL accumulator
            None
          }
          0x0E => {
            // ASL absolute
            Some(self.fetch_word())
          }
          0x16 => {
            // ASL zero page, x
            let base = self.fetch();
            Some((base + self.registers.x_index) as u16)
          }
          0x1E => {
            // ASL absolute, x
            let base = self.fetch_word();
            Some(base + self.registers.x_index as u16)
          }
          _ => unreachable!(),
        };

        let value = match address {
          Some(address) => self.read(address),
          None => self.registers.accumulator,
        };

        let result = value << 1;

        self.registers.status_write(flags::CARRY, value & 0x80 != 0);
        self.registers.status_set_nz(result);
        match address {
          Some(address) => self.write(address, result),
          None => self.registers.accumulator = result,
        };
        Ok(())
      }

      0x46 | 0x4A | 0x4E | 0x56 | 0x5E => {
        // LSR
        let address = match opcode {
          0x46 => {
            // LSR zero page
            Some(self.fetch() as u16)
          }
          0x4A => {
            // LSR accumulator
            None
          }
          0x4E => {
            // LSR absolute
            Some(self.fetch_word())
          }
          0x56 => {
            // LSR zero page, x
            let base = self.fetch();
            Some((base + self.registers.x_index) as u16)
          }
          0x5E => {
            // LSR absolute, x
            let base = self.fetch_word();
            Some(base + self.registers.x_index as u16)
          }
          _ => unreachable!(),
        };

        let value = match address {
          Some(address) => self.read(address),
          None => self.registers.accumulator,
        };

        let result = value >> 1;

        self.registers.status_write(flags::CARRY, value & 0x01 != 0);
        self.registers.status_set_nz(result);
        match address {
          Some(address) => self.write(address, result),
          None => self.registers.accumulator = result,
        };
        Ok(())
      }

      0x26 | 0x2A | 0x2E | 0x36 | 0x3E => {
        // ROL
        let address = match opcode {
          0x26 => {
            // ROL zero page
            Some(self.fetch() as u16)
          }
          0x2A => {
            // ROL accumulator
            None
          }
          0x2E => {
            // ROL absolute
            Some(self.fetch_word())
          }
          0x36 => {
            // ROL zero page, x
            let base = self.fetch();
            Some((base + self.registers.x_index) as u16)
          }
          0x3E => {
            // ROL absolute, x
            let base = self.fetch_word();
            Some(base + self.registers.x_index as u16)
          }
          _ => unreachable!(),
        };

        let value = match address {
          Some(address) => self.read(address),
          None => self.registers.accumulator,
        };

        let result = (value << 1) | (self.registers.status_read(flags::CARRY) as u8);

        self.registers.status_write(flags::CARRY, value & 0x80 != 0);
        self.registers.status_set_nz(result);
        match address {
          Some(address) => self.write(address, result),
          None => self.registers.accumulator = result,
        };
        Ok(())
      }

      0x66 | 0x6A | 0x6E | 0x76 | 0x7E => {
        // ROR
        let address = match opcode {
          0x66 => {
            // ROR zero page
            Some(self.fetch() as u16)
          }
          0x6A => {
            // ROR accumulator
            None
          }
          0x6E => {
            // ROR absolute
            Some(self.fetch_word())
          }
          0x76 => {
            // ROR zero page, x
            let base = self.fetch();
            Some((base + self.registers.x_index) as u16)
          }
          0x7E => {
            // ROR absolute, x
            let base = self.fetch_word();
            Some(base + self.registers.x_index as u16)
          }
          _ => unreachable!(),
        };

        let value = match address {
          Some(address) => self.read(address),
          None => self.registers.accumulator,
        };

        let carry = self.registers.status_read(flags::CARRY) as u8;
        let result = value >> 1 | carry << 7;

        self.registers.status_write(flags::CARRY, value & 0x01 != 0);
        self.registers.status_set_nz(result);

        match address {
          Some(address) => self.write(address, result),
          None => self.registers.accumulator = result,
        };

        Ok(())
      }

      // === LOGIC ===
      0x21 | 0x25 | 0x29 | 0x2D | 0x31 | 0x35 | 0x39 | 0x3D => {
        // AND
        let value = match opcode {
          0x21 => self.fetch_indirect_x(),
          0x25 => self.fetch_zero_page(),
          0x29 => self.fetch(),
          0x2D => self.fetch_absolute(),
          0x31 => self.fetch_indirect_y(),
          0x35 => self.fetch_zero_page_x(),
          0x39 => self.fetch_absolute_y(),
          0x3D => self.fetch_absolute_x(),
          _ => unreachable!(),
        };

        self.registers.accumulator &= value;
        self.registers.status_set_nz(self.registers.accumulator);
        Ok(())
      }

      0x24 | 0x2C => {
        // BIT
        let value = match opcode {
          0x24 => self.fetch_zero_page(),
          0x2C => self.fetch_absolute(),
          _ => unreachable!(),
        };

        self
          .registers
          .status_write(flags::NEGATIVE, value & 0x80 != 0);
        self
          .registers
          .status_write(flags::OVERFLOW, value & 0x40 != 0);
        self
          .registers
          .status_write(flags::ZERO, value & self.registers.accumulator == 0);
        Ok(())
      }

      0x41 | 0x45 | 0x49 | 0x4D | 0x51 | 0x55 | 0x59 | 0x5D => {
        // EOR
        let value = match opcode {
          0x41 => self.fetch_indirect_x(),
          0x45 => self.fetch_zero_page(),
          0x49 => self.fetch(),
          0x4D => self.fetch_absolute(),
          0x51 => self.fetch_indirect_y(),
          0x55 => self.fetch_zero_page_x(),
          0x59 => self.fetch_absolute_y(),
          0x5D => self.fetch_absolute_x(),
          _ => unreachable!(),
        };

        self.registers.accumulator ^= value;
        self.registers.status_set_nz(self.registers.accumulator);
        Ok(())
      }

      0x01 | 0x05 | 0x09 | 0x0D | 0x11 | 0x15 | 0x19 | 0x1D => {
        // ORA
        let value = match opcode {
          0x01 => self.fetch_indirect_x(),
          0x05 => self.fetch_zero_page(),
          0x09 => self.fetch(),
          0x0D => self.fetch_absolute(),
          0x11 => self.fetch_indirect_y(),
          0x15 => self.fetch_zero_page_x(),
          0x19 => self.fetch_absolute_y(),
          0x1D => self.fetch_absolute_x(),
          _ => unreachable!(),
        };

        self.registers.accumulator |= value;
        self.registers.status_set_nz(self.registers.accumulator);
        Ok(())
      }

      // === ARITHMETIC ===
      0x61 | 0x65 | 0x69 | 0x6D | 0x71 | 0x75 | 0x79 | 0x7D => {
        // ADC
        let value = match opcode {
          0x61 => self.fetch_indirect_x(),
          0x65 => self.fetch_zero_page(),
          0x69 => self.fetch(),
          0x6D => self.fetch_absolute(),
          0x71 => self.fetch_indirect_y(),
          0x75 => self.fetch_zero_page_x(),
          0x79 => self.fetch_absolute_y(),
          0x7D => self.fetch_absolute_x(),
          _ => unreachable!(),
        };

        self.registers.alu_add(value);
        Ok(())
      }

      0xC1 | 0xC5 | 0xC9 | 0xCD | 0xD1 | 0xD5 | 0xD9 | 0xDD => {
        // CMP
        let value = match opcode {
          0xC1 => self.fetch_indirect_x(),
          0xC5 => self.fetch_zero_page(),
          0xC9 => self.fetch(),
          0xCD => self.fetch_absolute(),
          0xD1 => self.fetch_indirect_y(),
          0xD5 => self.fetch_zero_page_x(),
          0xD9 => self.fetch_absolute_y(),
          0xDD => self.fetch_absolute_x(),
          _ => unreachable!(),
        };

        self
          .registers
          .alu_compare(self.registers.accumulator, value);
        Ok(())
      }

      0xE0 | 0xE4 | 0xEC => {
        // CPX
        let value = match opcode {
          0xE0 => self.fetch(),
          0xE4 => self.fetch_zero_page(),
          0xEC => self.fetch_absolute(),
          _ => unreachable!(),
        };

        self.registers.alu_compare(self.registers.x_index, value);
        Ok(())
      }

      0xC0 | 0xC4 | 0xCC => {
        // CPY
        let value = match opcode {
          0xC0 => self.fetch(),
          0xC4 => self.fetch_zero_page(),
          0xCC => self.fetch_absolute(),
          _ => unreachable!(),
        };

        self.registers.alu_compare(self.registers.y_index, value);
        Ok(())
      }

      0xE1 | 0xE5 | 0xE9 | 0xED | 0xF1 | 0xF5 | 0xF9 | 0xFD => {
        // SBC
        let value = match opcode {
          0xE1 => self.fetch_indirect_x(),
          0xE5 => self.fetch_zero_page(),
          0xE9 => self.fetch(),
          0xED => self.fetch_absolute(),
          0xF1 => self.fetch_indirect_y(),
          0xF5 => self.fetch_zero_page_x(),
          0xF9 => self.fetch_absolute_y(),
          0xFD => self.fetch_absolute_x(),
          _ => unreachable!(),
        };

        self.registers.alu_subtract(value);
        Ok(())
      }

      // === INCREMENT ===
      0xC6 | 0xCE | 0xD6 | 0xDE => {
        // DEC
        let address = match opcode {
          0xC6 => {
            // DEC zero page
            self.fetch() as u16
          }
          0xCE => {
            // DEC absolute
            self.fetch_word()
          }
          0xD6 => {
            // DEC zero page, X
            let base = self.fetch();
            (base + self.registers.x_index) as u16
          }
          0xDE => {
            // DEC absolute, X
            let base = self.fetch_word();
            base + self.registers.x_index as u16
          }
          _ => unreachable!(),
        };

        let value = self.read(address);
        self.registers.status_set_nz(value - 1);
        self.write(address, value - 1);
        Ok(())
      }

      0xCA => {
        // DEX
        self.registers.x_index -= 1;
        self.registers.status_set_nz(self.registers.x_index);
        Ok(())
      }

      0x88 => {
        // DEY
        self.registers.y_index -= 1;
        self.registers.status_set_nz(self.registers.y_index);
        Ok(())
      }

      0xE6 | 0xEE | 0xF6 | 0xFE => {
        // INC
        let address = match opcode {
          0xE6 => {
            // INC zero page
            self.fetch() as u16
          }
          0xEE => {
            // INC absolute
            self.fetch_word()
          }
          0xF6 => {
            // INC zero page, X
            let base = self.fetch();
            (base + self.registers.x_index) as u16
          }
          0xFE => {
            // INC absolute, X
            let base = self.fetch_word();
            base + self.registers.x_index as u16
          }
          _ => unreachable!(),
        };

        let value = self.read(address);
        self.registers.status_set_nz(value + 1);
        self.write(address, value + 1);
        Ok(())
      }

      0xE8 => {
        // INX
        self.registers.x_index += 1;
        self.registers.status_set_nz(self.registers.x_index);
        Ok(())
      }

      0xC8 => {
        // INY
        self.registers.y_index += 1;
        self.registers.status_set_nz(self.registers.y_index);
        Ok(())
      }

      // === CONTROL ===
      0x00 => {
        // BRK
        Err(())
      }
      0x4C | 0x6C => {
        // JMP
        let address = match opcode {
          0x4C => self.fetch_word(),
          0x6C => {
            let indirect = self.fetch_word();
            self.read_word(indirect)
          }
          _ => unreachable!(),
        };

        self.registers.pc_load(address);
        Ok(())
      }
      0x20 => {
        // JSR absolute
        let address = self.fetch_word();
        let return_to = self.registers.pc_address() + 1;
        self.push((return_to & 0xFF >> 8) as u8);
        self.push((return_to & 0xFF) as u8);
        self.registers.pc_load(address);
        Ok(())
      }
      0x40 => {
        // RTI
        Err(())
      }
      0x60 => {
        // RTS
        let pc_low = self.pop();
        let pc_high = self.pop();

        self
          .registers
          .pc_load((pc_high as u16 | (pc_low as u16) << 8) + 1);
        Ok(())
      }

      // === BRANCH ===
      0x90 | 0xB0 | 0xF0 | 0x30 | 0xD0 | 0x10 | 0x50 | 0x70 => {
        let offset = self.fetch() as i8;

        let condition = match opcode {
          0x90 => !self.registers.status_read(flags::CARRY), // BCC
          0xB0 => self.registers.status_read(flags::CARRY),  // BCS
          0xF0 => self.registers.status_read(flags::ZERO),   // BEQ
          0x30 => self.registers.status_read(flags::NEGATIVE), // BMI
          0xD0 => !self.registers.status_read(flags::ZERO),  // BNE
          0x10 => !self.registers.status_read(flags::NEGATIVE), // BPL
          0x50 => !self.registers.status_read(flags::OVERFLOW), // BVC
          0x70 => self.registers.status_read(flags::OVERFLOW), // BVS
          _ => unreachable!(),
        };

        if condition {
          self.registers.pc_offset(offset);
        }

        Ok(())
      }

      // === FLAGS ===
      0x18 | 0xD8 | 0x58 | 0xB8 => {
        self.registers.status_clear(match opcode {
          0x18 => flags::CARRY,     // CLC
          0xD8 => flags::DECIMAL,   // CLD
          0x58 => flags::INTERRUPT, // CLI
          0xB8 => flags::OVERFLOW,  // CLV
          _ => unreachable!(),
        });

        Ok(())
      }

      0x38 | 0xF8 | 0x78 => {
        self.registers.status_set(match opcode {
          0x38 => flags::CARRY,     // SEC
          0xF8 => flags::DECIMAL,   // SED
          0x78 => flags::INTERRUPT, // SEI
          _ => unreachable!(),
        });

        Ok(())
      }

      // === NOP ===
      0xEA => {
        // NOP
        Ok(())
      }

      _ => {
        println!("Unimplemented opcode: {:02X}", opcode);
        Err(())
      }
    }
  }
}
