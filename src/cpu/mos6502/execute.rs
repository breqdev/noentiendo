use crate::cpu::mos6502::{
  fetch::Fetch,
  registers::{flags, Alu},
  InterruptHandler, MemoryIO, Mos6502, Stack,
};

use super::Mos6502Variant;

pub trait Execute {
  /// Execute the given opcode, returning either the number of cycles used or an error.
  fn execute(&mut self, opcode: u8) -> Result<u8, ()>;
}

impl Execute for Mos6502 {
  fn execute(&mut self, opcode: u8) -> Result<u8, ()> {
    match opcode {
      // === LOAD ===
      0xA1 | 0xA5 | 0xA9 | 0xAD | 0xB1 | 0xB2 | 0xB5 | 0xB9 | 0xBD => {
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
      0x81 | 0x85 | 0x8D | 0x91 | 0x92 | 0x95 | 0x99 | 0x9D => {
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

        if let Mos6502Variant::NMOS = self.variant {
          self.write(address, value);
        }
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

        if let Mos6502Variant::NMOS = self.variant {
          self.write(address, value);
        }

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

        if let Mos6502Variant::NMOS = self.variant {
          self.write(address, value);
        }

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

        if let Mos6502Variant::NMOS = self.variant {
          self.write(address, value);
        }

        let result = value >> 1 | (self.registers.sr.read(flags::CARRY) as u8) << 7;

        self.registers.sr.write(flags::CARRY, value & 0x01 != 0);
        self.registers.sr.set_nz(result);
        self.write(address, result);
        Ok(cycles)
      }

      // === LOGIC ===
      0x21 | 0x25 | 0x29 | 0x2D | 0x31 | 0x32 | 0x35 | 0x39 | 0x3D => {
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

      0x41 | 0x45 | 0x49 | 0x4D | 0x51 | 0x52 | 0x55 | 0x59 | 0x5D => {
        // EOR
        let (value, cycles) = self.fetch_operand_value(opcode);
        self.registers.a ^= value;
        self.registers.sr.set_nz(self.registers.a);
        Ok(cycles)
      }

      0x01 | 0x05 | 0x09 | 0x0D | 0x11 | 0x12 | 0x15 | 0x19 | 0x1D => {
        // ORA
        let (value, cycles) = self.fetch_operand_value(opcode);
        self.registers.a |= value;
        self.registers.sr.set_nz(self.registers.a);
        Ok(cycles)
      }

      // === ARITHMETIC ===
      0x61 | 0x65 | 0x69 | 0x6D | 0x71 | 0x72 | 0x75 | 0x79 | 0x7D => {
        // ADC
        let (value, cycles) = self.fetch_operand_value(opcode);
        self.registers.alu_add(value);
        Ok(cycles)
      }

      0xC1 | 0xC5 | 0xC9 | 0xCD | 0xD1 | 0xD2 | 0xD5 | 0xD9 | 0xDD => {
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

      0xE1 | 0xE5 | 0xE9 | 0xED | 0xF1 | 0xF2 | 0xF5 | 0xF9 | 0xFD => {
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

            if self.variant == Mos6502Variant::NMOS && indirect & 0xFF == 0xFF {
              let lo = self.read(indirect);
              let hi = self.read(indirect & 0xFF00);
              ((hi as u16) << 8 | lo as u16, 5)
            } else {
              // normal behavior
              (self.read_word(indirect), 5)
            }
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

      _ => match self.variant {
        Mos6502Variant::NMOS => self.execute_nmos_extensions(opcode),
        Mos6502Variant::CMOS => self.execute_cmos_extensions(opcode),
      },
    }
  }
}

impl Mos6502 {
  fn execute_nmos_extensions(&mut self, opcode: u8) -> Result<u8, ()> {
    match opcode {
      // === ILLEGAL OPCODES ===
      0x02 | 0x22 | 0x42 | 0x62 => {
        // STP or KIL or JAM or HLT depending on who you ask
        println!("Execution stopped");
        Err(())
      }

      0x03 | 0x07 | 0x0F | 0x13 | 0x17 | 0x1B | 0x1F => {
        // SLO: ASL -> ORA
        let (address, cycles) = self.fetch_operand_address(opcode);
        let value = self.read(address);
        self.registers.sr.write(flags::CARRY, value & 0x80 != 0);

        let result = value << 1;
        self.write(address, result);

        self.registers.a |= result;
        self.registers.sr.set_nz(self.registers.a);

        Ok(cycles + 2)
      }

      0x23 | 0x27 | 0x2F | 0x33 | 0x37 | 0x3B | 0x3F => {
        // RLA: ROL -> AND
        let (address, cycles) = self.fetch_operand_address(opcode);
        let value = self.read(address);

        let result = (value << 1) | (self.registers.sr.read(flags::CARRY) as u8);
        self.registers.sr.write(flags::CARRY, result & 0x80 != 0);
        self.write(address, result);

        self.registers.a &= result;
        self.registers.sr.set_nz(self.registers.a);

        Ok(cycles + 2)
      }

      0x43 | 0x47 | 0x4F | 0x53 | 0x57 | 0x5B | 0x5F => {
        // SRE: LSR -> EOR
        let (address, cycles) = self.fetch_operand_address(opcode);
        let value = self.read(address);
        let result = value >> 1;

        self.registers.sr.write(flags::CARRY, value & 0x01 != 0);
        self.write(address, result);

        self.registers.a ^= result;
        self.registers.sr.set_nz(self.registers.a);

        Ok(cycles + 2)
      }

      0x63 | 0x67 | 0x6F | 0x73 | 0x77 | 0x7B | 0x7F => {
        // RRA: ROR -> ADC
        let (address, cycles) = self.fetch_operand_address(opcode);
        let value = self.read(address);
        let result = value >> 1 | (self.registers.sr.read(flags::CARRY) as u8) << 7;

        self.registers.sr.write(flags::CARRY, value & 0x01 != 0);
        self.registers.sr.set_nz(result);
        self.write(address, result);

        self.registers.alu_add(result);

        Ok(cycles)
      }

      0x83 | 0x87 | 0x8F | 0x97 => {
        // SAX: AND -> STA
        let (address, cycles) = self.fetch_operand_address(opcode);
        let value = self.registers.x & self.registers.a;
        self.registers.sr.set_nz(value);
        self.write(address, value);

        Ok(cycles)
      }

      0xA3 | 0xA7 | 0xAF | 0xB3 | 0xB7 | 0xBF => {
        // LAX: LDA & LDX
        let (value, cycles) = self.fetch_operand_value(opcode);
        self.registers.a = value;
        self.registers.x = value;
        self.registers.sr.set_nz(value);

        Ok(cycles)
      }

      0xC3 | 0xC7 | 0xCF | 0xD3 | 0xD7 | 0xDB | 0xDF => {
        // DCP: DEC + SEC
        self.registers.sr.set(flags::CARRY);

        let (address, cycles) = self.fetch_operand_address(opcode);
        let value = self.read(address);
        let result = value.wrapping_sub(1);
        self.registers.sr.set_nz(result);
        self.write(address, result);

        Ok(cycles + 2)
      }

      0xE3 | 0xE7 | 0xEF | 0xF3 | 0xF7 | 0xFB | 0xFF => {
        // ISC: INC => SBC
        let (address, cycles) = self.fetch_operand_address(opcode);
        let value = self.read(address);
        let result = value.wrapping_add(1);
        self.registers.alu_subtract(value);
        self.registers.sr.set_nz(result);
        self.write(address, result);

        Ok(cycles + 2)
      }

      0x0B | 0x2B => {
        // ANC: AND byte with accumulator. If result is negative then carry is set.
        let (value, cycles) = self.fetch_operand_value(opcode);
        let new_val = self.registers.a & value;
        self.registers.sr.write(flags::CARRY, new_val & 0x80 != 0);

        Ok(cycles)
      }

      0x4B => {
        // ALR: AND + LSR
        let (value, cycles) = self.fetch_operand_value(opcode);
        let new_val = (self.registers.a & value) >> 1;

        self.registers.sr.write(flags::CARRY, new_val & 0x01 != 0);
        self.registers.sr.set_nz(new_val);

        Ok(cycles)
      }

      0x6B => {
        // ARR: AND + ROR
        let (value, cycles) = self.fetch_operand_value(opcode);
        let new_val = self.registers.a & value;

        let new_val = (new_val >> 1) | (self.registers.sr.read(flags::CARRY) as u8) << 7;

        self.registers.sr.write(flags::CARRY, new_val & 0x40 != 0);
        self
          .registers
          .sr
          .write(flags::OVERFLOW, new_val & 0x20 != 0);
        self.registers.sr.set_nz(new_val);

        Ok(cycles)
      }

      0x8B => {
        // XAA: AND X + AND immediate
        // Oooo she's highly unstable xx "Do not use" or whatever

        let (value, cycles) = self.fetch_operand_value(opcode);
        let magic: u8;
        #[cfg(not(target_arch = "wasm32"))]
        {
          magic = rand::random::<u8>();
        }
        #[cfg(target_arch = "wasm32")]
        {
          magic = 0xFF;
        }
        self.registers.a |= magic;
        self.registers.a &= self.registers.x & value;
        self.registers.sr.set_nz(self.registers.a);

        Ok(cycles)
      }

      0xCB => {
        // AXS: AND -> DEX -> STX
        let (value, cycles) = self.fetch_operand_value(opcode);
        self.registers.x &= self.registers.a;

        self.registers.alu_compare(self.registers.x, value);
        self.registers.x = self.registers.x.wrapping_sub(value);

        Ok(cycles)
      }

      0xEB => {
        // SBC (same as official sbc)
        let (value, cycles) = self.fetch_operand_value(opcode);
        self.registers.alu_subtract(value);
        Ok(cycles)
      }

      0x9C => {
        // SHY: (Y & (high(addr) + 1)) -> addr
        let (address, cycles) = self.fetch_operand_address(opcode);
        let value = (address >> 8) as u8;
        let result = self.registers.y & (value.wrapping_add(1));
        self.registers.sr.set_nz(result);
        self.write(address, result);

        Ok(cycles + 2)
      }

      0x9E => {
        // SHX: (X & (high(addr) + 1)) -> addr
        let (address, cycles) = self.fetch_operand_address(opcode);
        let value = (address >> 8) as u8;
        let result = self.registers.x & (value.wrapping_add(1));
        self.registers.sr.set_nz(result);
        self.write(address, result);

        Ok(cycles + 2)
      }

      0x93 | 0x9F => {
        // AHX: (A & X & (high(addr) + 1)) -> addr
        let (address, cycles) = self.fetch_operand_address(opcode);
        let value = (address >> 8) as u8;
        let result = self.registers.a & self.registers.x & (value.wrapping_add(1));
        self.registers.sr.set_nz(result);
        self.write(address, result);

        Ok(cycles + 2)
      }

      0x9B => {
        // TAS: TSX with accumulator and AHX
        // A AND X -> SP
        // A AND X AND (H+1) -> M
        self.registers.sp.set(self.registers.a & self.registers.x);

        let (address, cycles) = self.fetch_operand_address(opcode);
        let value = (address >> 8) as u8;
        let result = self.registers.a & self.registers.x & (value.wrapping_add(1));
        self.write(address, result);

        Ok(cycles + 2)
      }

      0xBB => {
        // LAS: LDA + TSX unholy matrimony
        // M AND SP -> A, X, SP
        let (value, cycles) = self.fetch_operand_value(opcode);
        let result = value & self.registers.sp.get();

        self.registers.a = result;
        self.registers.x = result;
        self.registers.sp.set(result);
        self.registers.sr.set_nz(result);

        Ok(cycles)
      }

      0xAB => {
        // ATX or LXA: XAA but instead of and X we store in X
        let (value, cycles) = self.fetch_operand_value(opcode);
        let magic: u8;
        #[cfg(not(target_arch = "wasm32"))]
        {
          magic = rand::random::<u8>();
        }
        #[cfg(target_arch = "wasm32")]
        {
          magic = 0xFF;
        }
        self.registers.a |= magic;
        self.registers.a &= value;
        self.registers.x = self.registers.a;
        self.registers.sr.set_nz(self.registers.a);

        Ok(cycles)
      }

      // TODO: Verify cycle counts
      _ => {
        // NOP
        match opcode {
          0x1A | 0x3A | 0x5A | 0x7A | 0xDA | 0xFA => {
            // No address
            Ok(2)
          }
          _ => {
            // Address
            let (_value, cycles) = self.fetch_operand_value(opcode);
            Ok(cycles)
          }
        }
      }
    }
  }

  fn execute_cmos_extensions(&mut self, opcode: u8) -> Result<u8, ()> {
    match opcode {
      0x89 | 0x34 | 0x3C => {
        // BIT (3 extra addressing modes)
        let (value, cycles) = self.fetch_operand_value(opcode);

        if opcode != 0x89 {
          // N, V flags not set for immediate
          self.registers.sr.write(flags::NEGATIVE, value & 0x80 != 0);
          self.registers.sr.write(flags::OVERFLOW, value & 0x40 != 0);
        }

        self
          .registers
          .sr
          .write(flags::ZERO, value & self.registers.a == 0);
        Ok(cycles)
      }

      0x3A => {
        // DEC (like DEX/DEY but for accumulator)
        self.registers.a = self.registers.a.wrapping_sub(1);
        self.registers.sr.set_nz(self.registers.a);
        Ok(2)
      }

      0x1A => {
        // INC (like INX/INY but for accumulator)
        self.registers.a = self.registers.a.wrapping_add(1);
        self.registers.sr.set_nz(self.registers.a);
        Ok(2)
      }

      0x7C => {
        // JMP (abs,X)
        let address = self.fetch_word();
        let pointer = address + self.registers.x as u16;
        let address = self.read_word(pointer);
        self.registers.pc.load(address);
        Ok(6)
      }

      0x80 => {
        // BRA (branch Always)
        let offset = self.fetch() as i8;
        self.registers.pc.offset(offset);
        Ok(3)
      }

      // New Stack Instructions
      0xDA => {
        // PHX (push X onto stack)
        self.push(self.registers.x);
        Ok(3)
      }
      0x5A => {
        // PHY (push Y onto stack)
        self.push(self.registers.y);
        Ok(3)
      }
      0xFA => {
        // PLX (pull X from stack)
        let value = self.pop();
        self.registers.x = value;
        self.registers.sr.set_nz(value);
        Ok(4)
      }
      0x7A => {
        // PLY (pull Y from stack)
        let value = self.pop();
        self.registers.y = value;
        self.registers.sr.set_nz(value);
        Ok(4)
      }

      0x64 | 0x74 | 0x9C | 0x9E => {
        // STZ (store zero)
        // Note: 0x9C breaks the typical addressing mode pattern
        let (address, cycles) = match opcode {
          0x9C => (self.fetch_word(), 4),
          0x9E => {
            let base = self.fetch_word();
            let indexed = base + self.registers.x as u16;
            (indexed, 4)
          }
          _ => self.fetch_operand_address(opcode),
        };

        self.write(address, 0);
        Ok(cycles)
      }

      0x14 | 0x1C => {
        // TRB (test and reset bits)
        let (address, cycles) = match opcode {
          0x14 => (self.fetch() as u16, 3),
          0x1C => (self.fetch_word(), 4),
          _ => unreachable!(),
        };
        let value = self.read(address);

        self
          .registers
          .sr
          .write(flags::ZERO, value & self.registers.a == 0);

        self.write(address, value & !self.registers.a);
        Ok(cycles)
      }

      0x04 | 0x0C => {
        // TSB (test and set bits)
        let (address, cycles) = match opcode {
          0x04 => (self.fetch() as u16, 3),
          0x0C => (self.fetch_word(), 4),
          _ => unreachable!(),
        };
        let value = self.read(address);

        self
          .registers
          .sr
          .write(flags::ZERO, value & self.registers.a == 0);

        self.write(address, value | self.registers.a);
        Ok(cycles)
      }

      0x0F | 0x1F | 0x2F | 0x3F | 0x4F | 0x5F | 0x6F | 0x7F | 0x8F | 0x9F | 0xAF | 0xBF | 0xCF
      | 0xDF | 0xEF | 0xFF => {
        // BBS and BBR
        let address = self.fetch() as u16;
        let value = self.read(address);
        let offset = self.fetch() as i8;

        let bit = (opcode >> 4) & 0b111;
        let bit_value = ((1 << bit) & value) != 0;
        let target_value = opcode & 0x80 != 0;

        if target_value == bit_value {
          self.registers.pc.offset(offset);
          Ok(3)
        } else {
          Ok(2)
        }
      }

      0x07 | 0x17 | 0x27 | 0x37 | 0x47 | 0x57 | 0x67 | 0x77 | 0x87 | 0x97 | 0xA7 | 0xB7 | 0xC7
      | 0xD7 | 0xE7 | 0xF7 => {
        // RMB and SMB
        let address = self.fetch() as u16;
        let value = self.read(address);

        let bit = (opcode >> 4) & 0b111;

        let value = if opcode & 0x80 == 0 {
          value & !(1 << bit)
        } else {
          value | (1 << bit)
        };
        self.write(address, value);

        Ok(2)
      }

      0x02 | 0x22 | 0x42 | 0x62 | 0x82 | 0xA2 | 0xC2 | 0xE2 => {
        // NOP (2-byte)
        self.fetch();
        Ok(2)
      }
      0x44 => {
        self.fetch();
        Ok(3)
      }
      0x54 | 0xD4 | 0xF4 => {
        self.fetch();
        Ok(4)
      }
      0x5C => {
        self.fetch_word();
        Ok(8)
      }
      0xDC | 0xFC => {
        self.fetch_word();
        Ok(4)
      }

      _ => {
        // NOP
        Ok(1)
      }
    }
  }
}
