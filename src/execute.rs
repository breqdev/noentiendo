use crate::registers::{flags, ProgramCounter, StatusRegister, ALU};
use crate::system::{Fetch, System};

pub trait Execute {
  fn execute(&mut self, opcode: u8) -> Result<(), ()>;
}

impl Execute for System {
  fn execute(&mut self, opcode: u8) -> Result<(), ()> {
    match opcode {
      0x00 => {
        // BRK
        Err(())
      }
      0x01 => {
        // ORA (indirect,X)
        Err(())
      }
      0x05 => {
        // ORA zero page
        Err(())
      }
      0x06 => {
        // ASL zero page
        Err(())
      }
      0x08 => {
        // PHP
        Err(())
      }
      0x09 => {
        // ORA immediate
        let value = self.fetch()?;
        self.registers.accumulator |= value;
        self.registers.status_set_nz(self.registers.accumulator);
        Ok(())
      }
      0x0A => {
        // ASL accumulator
        Err(())
      }
      0x0D => {
        // ORA absolute
        Err(())
      }
      0x0E => {
        // ASL absolute
        Err(())
      }

      // ===
      0x10 => {
        // BPL
        Err(())
      }
      0x11 => {
        // ORA (indirect),Y
        Err(())
      }
      0x15 => {
        // ORA zero page,X
        Err(())
      }
      0x16 => {
        // ASL zero page,X
        Err(())
      }
      0x18 => {
        // CLC
        self.registers.status_clear(flags::CARRY);
        Ok(())
      }
      0x19 => {
        // ORA absolute,Y
        Err(())
      }
      0x1D => {
        // ORA absolute,X
        Err(())
      }
      0x1E => {
        // ASL absolute,X
        Err(())
      }

      // ===
      0x20 => {
        // JSR absolute
        Err(())
      }
      0x21 => {
        // AND (indirect,X)
        Err(())
      }
      0x24 => {
        // BIT zero page
        Err(())
      }
      0x25 => {
        // AND zero page
        Err(())
      }
      0x26 => {
        // ROL zero page
        Err(())
      }
      0x28 => {
        // PLP
        Err(())
      }
      0x29 => {
        // AND immediate
        let value = self.fetch()?;
        self.registers.accumulator &= value;
        self.registers.status_set_nz(self.registers.accumulator);
        Ok(())
      }
      0x2A => {
        // ROL accumulator
        Err(())
      }
      0x2C => {
        // BIT absolute
        Err(())
      }
      0x2D => {
        // AND absolute
        Err(())
      }
      0x2E => {
        // ROL absolute
        Err(())
      }

      // ===
      0x30 => {
        // BMI
        Err(())
      }
      0x31 => {
        // AND (indirect),Y
        Err(())
      }
      0x35 => {
        // AND zero page,X
        Err(())
      }
      0x36 => {
        // ROL zero page,X
        Err(())
      }
      0x38 => {
        // SEC
        self.registers.status_set(flags::CARRY);
        Ok(())
      }
      0x39 => {
        // AND absolute,Y
        Err(())
      }
      0x3D => {
        // AND absolute,X
        Err(())
      }
      0x3E => {
        // ROL absolute,X
        Err(())
      }

      // ===
      0x40 => {
        // RTI
        Err(())
      }
      0x41 => {
        // EOR (indirect,X)
        Err(())
      }
      0x45 => {
        // EOR zero page
        Err(())
      }
      0x46 => {
        // LSR zero page
        Err(())
      }
      0x48 => {
        // PHA
        Err(())
      }
      0x49 => {
        // EOR immediate
        let value = self.fetch()?;
        self.registers.accumulator ^= value;
        self.registers.status_set_nz(self.registers.accumulator);
        Ok(())
      }
      0x4A => {
        // LSR accumulator
        Err(())
      }
      0x4C => {
        // JMP absolute
        let address = self.fetch_word()?;
        self.registers.pc_load(address);
        Ok(())
      }
      0x4D => {
        // EOR absolute
        Err(())
      }
      0x4E => {
        // LSR absolute
        Err(())
      }

      // ===
      0x50 => {
        // BVC
        Err(())
      }
      0x51 => {
        // EOR (indirect),Y
        Err(())
      }
      0x55 => {
        // EOR zero page,X
        Err(())
      }
      0x56 => {
        // LSR zero page,X
        Err(())
      }
      0x58 => {
        // CLI
        self.registers.status_clear(flags::INTERRUPT);
        Ok(())
      }
      0x59 => {
        // EOR absolute,Y
        Err(())
      }
      0x5D => {
        // EOR absolute,X
        Err(())
      }
      0x5E => {
        // LSR absolute,X
        Err(())
      }

      // ===
      0x60 => {
        // RTS
        Err(())
      }
      0x61 => {
        // ADC (indirect,X)
        Err(())
      }
      0x65 => {
        // ADC zero page
        Err(())
      }
      0x66 => {
        // ROR zero page
        Err(())
      }
      0x68 => {
        // PLA
        Err(())
      }
      0x69 => {
        // ADC immediate
        let value = self.fetch()?;
        self.registers.alu_add(value);
        Ok(())
      }
      0x6A => {
        // ROR accumulator
        Err(())
      }
      0x6C => {
        // JMP (indirect)
        Err(())
      }
      0x6D => {
        // ADC absolute
        Err(())
      }
      0x6E => {
        // ROR absolute
        Err(())
      }

      // ===
      0x70 => {
        // BVS
        Err(())
      }
      0x71 => {
        // ADC (indirect),Y
        Err(())
      }
      0x75 => {
        // ADC zero page,X
        Err(())
      }
      0x76 => {
        // ROR zero page,X
        Err(())
      }
      0x78 => {
        // SEI
        self.registers.status_set(flags::INTERRUPT);
        Ok(())
      }
      0x79 => {
        // ADC absolute,Y
        Err(())
      }
      0x7D => {
        // ADC absolute,X
        Err(())
      }
      0x7E => {
        // ROR absolute,X
        Err(())
      }

      // ===
      0x81 => {
        // STA (indirect,X)
        Err(())
      }
      0x84 => {
        // STY zero page
        Err(())
      }
      0x85 => {
        // STA zero page
        Err(())
      }
      0x86 => {
        // STX zero page
        Err(())
      }
      0x88 => {
        // DEY
        Err(())
      }
      0x8A => {
        // TXA
        Err(())
      }
      0x8C => {
        // STY absolute
        Err(())
      }
      0x8D => {
        // STA absolute
        let address = self.fetch_word()?;
        self.memory.write(address, self.registers.accumulator)?;
        Ok(())
      }
      0x8E => {
        // STX absolute
        Err(())
      }

      // ===
      0x90 => {
        // BCC
        Err(())
      }
      0x91 => {
        // STA (indirect),Y
        Err(())
      }
      0x94 => {
        // STY zero page,X
        Err(())
      }
      0x95 => {
        // STA zero page,X
        Err(())
      }
      0x96 => {
        // STX zero page,Y
        Err(())
      }
      0x98 => {
        // TYA
        Err(())
      }
      0x99 => {
        // STA absolute,Y
        Err(())
      }
      0x9A => {
        // TXS
        Err(())
      }
      0x9D => {
        // STA absolute,X
        Err(())
      }

      // ===
      0xA0 => {
        // LDY immediate
        let value = self.fetch()?;
        self.registers.y_index = value;
        self.registers.status_set_nz(value);
        Ok(())
      }
      0xA1 => {
        // LDA (indirect,X)
        Err(())
      }
      0xA2 => {
        // LDX immediate
        let value = self.fetch()?;
        self.registers.x_index = value;
        self.registers.status_set_nz(value);
        Err(())
      }
      0xA4 => {
        // LDY zero page
        Err(())
      }
      0xA5 => {
        // LDA zero page
        Err(())
      }
      0xA6 => {
        // LDX zero page
        Err(())
      }
      0xA8 => {
        // TAY
        Err(())
      }
      0xA9 => {
        // LDA immediate
        let value = self.fetch()?;
        self.registers.accumulator = value;
        self.registers.status_set_nz(value);
        Ok(())
      }
      0xAA => {
        // TAX
        Err(())
      }
      0xAC => {
        // LDY absolute
        Err(())
      }
      0xAD => {
        // LDA absolute
        Err(())
      }
      0xAE => {
        // LDX absolute
        Err(())
      }

      // ===
      0xB0 => {
        // BCS
        Err(())
      }
      0xB1 => {
        // LDA (indirect),Y
        Err(())
      }
      0xB4 => {
        // LDY zero page,X
        Err(())
      }
      0xB5 => {
        // LDA zero page,X
        Err(())
      }
      0xB6 => {
        // LDX zero page,Y
        Err(())
      }
      0xB8 => {
        // CLV
        self.registers.status_clear(flags::OVERFLOW);
        Ok(())
      }
      0xB9 => {
        // LDA absolute,Y
        Err(())
      }
      0xBA => {
        // TSX
        Err(())
      }
      0xBC => {
        // LDY absolute,X
        Err(())
      }
      0xBD => {
        // LDA absolute,X
        Err(())
      }
      0xBE => {
        // LDX absolute,Y
        Err(())
      }

      // ===
      0xC0 => {
        // CPY immediate
        Err(())
      }
      0xC1 => {
        // CMP (indirect,X)
        Err(())
      }
      0xC4 => {
        // CPY zero page
        Err(())
      }
      0xC5 => {
        // CMP zero page
        Err(())
      }
      0xC6 => {
        // DEC zero page
        Err(())
      }
      0xC8 => {
        // INY
        Err(())
      }
      0xC9 => {
        // CMP immediate
        Err(())
      }
      0xCA => {
        // DEX
        Err(())
      }
      0xCC => {
        // CPY absolute
        Err(())
      }
      0xCD => {
        // CMP absolute
        Err(())
      }
      0xCE => {
        // DEC absolute
        Err(())
      }

      // ===
      0xD0 => {
        // BNE
        Err(())
      }
      0xD1 => {
        // CMP (indirect),Y
        Err(())
      }
      0xD5 => {
        // CMP zero page,X
        Err(())
      }
      0xD6 => {
        // DEC zero page,X
        Err(())
      }
      0xD8 => {
        // CLD
        self.registers.status_clear(flags::DECIMAL);
        Ok(())
      }
      0xD9 => {
        // CMP absolute,Y
        Err(())
      }
      0xDD => {
        // CMP absolute,X
        Err(())
      }
      0xDE => {
        // DEC absolute,X
        Err(())
      }

      // ===
      0xE0 => {
        // CPX immediate
        Err(())
      }
      0xE1 => {
        // SBC (indirect,X)
        Err(())
      }
      0xE4 => {
        // CPX zero page
        Err(())
      }
      0xE5 => {
        // SBC zero page
        Err(())
      }
      0xE6 => {
        // INC zero page
        Err(())
      }
      0xE8 => {
        // INX
        Err(())
      }
      0xE9 => {
        // SBC immediate
        let value = self.fetch()?;
        self.registers.alu_subtract(value);
        Ok(())
      }
      0xEA => {
        // NOP
        Err(())
      }
      0xEC => {
        // CPX absolute
        Err(())
      }
      0xED => {
        // SBC absolute
        Err(())
      }
      0xEE => {
        // INC absolute
        Err(())
      }

      // ===
      0xF0 => {
        // BEQ
        Err(())
      }
      0xF1 => {
        // SBC (indirect),Y
        Err(())
      }
      0xF5 => {
        // SBC zero page,X
        Err(())
      }
      0xF6 => {
        // INC zero page,X
        Err(())
      }
      0xF8 => {
        // SED
        self.registers.status_set(flags::DECIMAL);
        Ok(())
      }
      0xF9 => {
        // SBC absolute,Y
        Err(())
      }
      0xFD => {
        // SBC absolute,X
        Err(())
      }
      0xFE => {
        // INC absolute,X
        Err(())
      }

      _ => {
        println!("Unimplemented opcode: {:02X}", opcode);
        Err(())
      }
    }
  }
}
