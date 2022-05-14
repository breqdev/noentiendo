use crate::registers::{flags, ProgramCounter, StatusRegister, ALU};
use crate::system::{Fetch, MemoryIO, Stack, System};

pub trait Execute {
  fn execute(&mut self, opcode: u8) -> Result<(), ()>;
}

impl Execute for System {
  fn execute(&mut self, opcode: u8) -> Result<(), ()> {
    match opcode {
      // === LOAD ===

      // LDA
      0xA1 => {
        // LDA (indirect,X)
        let value = self.fetch_indirect_x()?;
        self.registers.accumulator = value;
        self.registers.status_set_nz(value);
        Ok(())
      }
      0xA5 => {
        // LDA zero page
        let value = self.fetch_zero_page()?;
        self.registers.accumulator = value;
        self.registers.status_set_nz(value);
        Ok(())
      }
      0xA9 => {
        // LDA immediate
        let value = self.fetch()?;
        self.registers.accumulator = value;
        self.registers.status_set_nz(value);
        Ok(())
      }
      0xAD => {
        // LDA absolute
        let value = self.fetch_zero_page()?;
        self.registers.accumulator = value;
        self.registers.status_set_nz(value);
        Ok(())
      }
      0xB1 => {
        // LDA (indirect),Y
        let value = self.fetch_indirect_y()?;
        self.registers.accumulator = value;
        self.registers.status_set_nz(value);
        Ok(())
      }
      0xB5 => {
        // LDA zero page,X
        let value = self.fetch_zero_page_x()?;
        self.registers.accumulator = value;
        self.registers.status_set_nz(value);
        Ok(())
      }
      0xB9 => {
        // LDA absolute,Y
        let value = self.fetch_absolute_y()?;
        self.registers.accumulator = value;
        self.registers.status_set_nz(value);
        Ok(())
      }
      0xBD => {
        // LDA absolute,X
        let value = self.fetch_absolute_x()?;
        self.registers.accumulator = value;
        self.registers.status_set_nz(value);
        Ok(())
      }

      // LDX
      0xA2 => {
        // LDX immediate
        let value = self.fetch()?;
        self.registers.x_index = value;
        self.registers.status_set_nz(value);
        Ok(())
      }
      0xA6 => {
        // LDX zero page
        let value = self.fetch_zero_page()?;
        self.registers.x_index = value;
        self.registers.status_set_nz(value);
        Ok(())
      }
      0xAE => {
        // LDX absolute
        let value = self.fetch_absolute()?;
        self.registers.x_index = value;
        self.registers.status_set_nz(value);
        Ok(())
      }
      0xB6 => {
        // LDX zero page,Y
        let value = self.fetch_zero_page_y()?;
        self.registers.x_index = value;
        self.registers.status_set_nz(value);
        Ok(())
      }
      0xBE => {
        // LDX absolute,Y
        let value = self.fetch_absolute_y()?;
        self.registers.x_index = value;
        self.registers.status_set_nz(value);
        Ok(())
      }

      // LDY
      0xA0 => {
        // LDY immediate
        let value = self.fetch()?;
        self.registers.y_index = value;
        self.registers.status_set_nz(value);
        Ok(())
      }
      0xA4 => {
        // LDY zero page
        let value = self.fetch_zero_page()?;
        self.registers.y_index = value;
        self.registers.status_set_nz(value);
        Ok(())
      }
      0xAC => {
        // LDY absolute
        let value = self.fetch_absolute()?;
        self.registers.y_index = value;
        self.registers.status_set_nz(value);
        Ok(())
      }
      0xB4 => {
        // LDY zero page,X
        let value = self.fetch_zero_page_x()?;
        self.registers.y_index = value;
        self.registers.status_set_nz(value);
        Ok(())
      }
      0xBC => {
        // LDY absolute,X
        let value = self.fetch_absolute_x()?;
        self.registers.y_index = value;
        self.registers.status_set_nz(value);
        Ok(())
      }

      // === STORE ===

      // STA
      0x81 => {
        // STA (indirect,X)
        let base = self.fetch()?;
        let address = (base + self.registers.x_index) as u16;
        let pointer = self.read_word(address)?;
        self.write(pointer, self.registers.accumulator)?;
        Ok(())
      }
      0x85 => {
        // STA zero page
        let address = self.fetch()?;
        self.write(address as u16, self.registers.accumulator)?;
        Ok(())
      }
      0x8D => {
        // STA absolute
        let address = self.fetch_word()?;
        self.write(address, self.registers.accumulator)?;
        Ok(())
      }
      0x91 => {
        // STA (indirect),Y
        let base = self.fetch()?;
        let address = self.read_word(base as u16)?;
        let pointer = address + self.registers.y_index as u16;
        self.write(pointer, self.registers.accumulator)?;
        Ok(())
      }
      0x95 => {
        // STA zero page,X
        let base = self.fetch()?;
        let address = base + self.registers.x_index;
        self.write(address as u16, self.registers.accumulator)?;
        Ok(())
      }
      0x99 => {
        // STA absolute,Y
        let base = self.fetch_word()?;
        let address = base + (self.registers.y_index as u16);
        self.write(address, self.registers.accumulator)?;
        Ok(())
      }
      0x9D => {
        // STA absolute,X
        let base = self.fetch_word()?;
        let address = base + (self.registers.x_index as u16);
        self.write(address, self.registers.accumulator)?;
        Ok(())
      }

      // STX
      0x86 => {
        // STX zero page
        let address = self.fetch()?;
        self.write(address as u16, self.registers.x_index)?;
        Ok(())
      }
      0x8E => {
        // STX absolute
        let address = self.fetch_word()?;
        self.write(address, self.registers.x_index)?;
        Ok(())
      }
      0x96 => {
        // STX zero page,Y
        let base = self.fetch()?;
        let address = base + self.registers.y_index;
        self.write(address as u16, self.registers.x_index)?;
        Ok(())
      }

      // STY
      0x84 => {
        // STY zero page
        let address = self.fetch()?;
        self.write(address as u16, self.registers.y_index)?;
        Ok(())
      }
      0x8C => {
        // STY absolute
        let address = self.fetch_word()?;
        self.write(address, self.registers.y_index)?;
        Ok(())
      }
      0x94 => {
        // STY zero page,X
        let base = self.fetch()?;
        let address = base + self.registers.x_index;
        self.write(address as u16, self.registers.y_index)?;
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
        self.push(self.registers.accumulator)?;
        Ok(())
      }
      0x08 => {
        // PHP
        self.push(self.registers.status_register)?;
        Ok(())
      }
      0x68 => {
        // PLA
        self.registers.accumulator = self.pop()?;
        Ok(())
      }
      0x28 => {
        // PLP
        self.registers.status_register = self.pop()?;
        Ok(())
      }

      // === SHIFT ===

      // ASL
      0x06 => {
        // ASL zero page
        let address = self.fetch()?;
        let value = self.read(address as u16)?;
        self.registers.status_write(flags::CARRY, value & 0x80 != 0);
        let result = value << 1;
        self.registers.status_set_nz(result);
        self.write(address as u16, result)?;
        Ok(())
      }
      0x0A => {
        // ASL accumulator
        self
          .registers
          .status_write(flags::CARRY, self.registers.accumulator & 0x80 != 0);
        self.registers.accumulator = self.registers.accumulator << 1;
        self.registers.status_set_nz(self.registers.accumulator);
        Ok(())
      }
      0x0E => {
        // ASL absolute
        let address = self.fetch_word()?;
        let value = self.read(address)?;
        self.registers.status_write(flags::CARRY, value & 0x80 != 0);
        let result = value << 1;
        self.registers.status_set_nz(result);
        self.write(address, result)?;
        Ok(())
      }
      0x16 => {
        // ASL zero page,X
        let base = self.fetch()?;
        let address = base + self.registers.x_index;
        let value = self.read(address as u16)?;
        self.registers.status_write(flags::CARRY, value & 0x80 != 0);
        let result = value << 1;
        self.registers.status_set_nz(result);
        self.write(address as u16, result)?;
        Ok(())
      }
      0x1E => {
        // ASL absolute,X
        let base = self.fetch_word()?;
        let address = base + (self.registers.x_index as u16);
        let value = self.read(address)?;
        self.registers.status_write(flags::CARRY, value & 0x80 != 0);
        let result = value << 1;
        self.registers.status_set_nz(result);
        self.write(address, result)?;
        Ok(())
      }

      // LSR
      0x46 => {
        // LSR zero page
        let address = self.fetch()?;
        let value = self.read(address as u16)?;
        self.registers.status_write(flags::CARRY, value & 0x01 != 0);
        let result = value >> 1;
        self.registers.status_set_nz(result);
        self.write(address as u16, result)?;
        Ok(())
      }
      0x4A => {
        // LSR accumulator
        self
          .registers
          .status_write(flags::CARRY, self.registers.accumulator & 0x01 != 0);
        self.registers.accumulator = self.registers.accumulator >> 1;
        self.registers.status_set_nz(self.registers.accumulator);
        Ok(())
      }
      0x4E => {
        // LSR absolute
        let address = self.fetch_word()?;
        let value = self.read(address)?;
        self.registers.status_write(flags::CARRY, value & 0x01 != 0);
        let result = value >> 1;
        self.registers.status_set_nz(result);
        self.write(address, result)?;
        Ok(())
      }
      0x56 => {
        // LSR zero page,X
        let base = self.fetch()?;
        let address = base + self.registers.x_index;
        let value = self.read(address as u16)?;
        self.registers.status_write(flags::CARRY, value & 0x01 != 0);
        let result = value >> 1;
        self.registers.status_set_nz(result);
        self.write(address as u16, result)?;
        Ok(())
      }
      0x5E => {
        // LSR absolute,X
        let base = self.fetch_word()?;
        let address = base + (self.registers.x_index as u16);
        let value = self.read(address)?;
        self.registers.status_write(flags::CARRY, value & 0x01 != 0);
        let result = value >> 1;
        self.registers.status_set_nz(result);
        self.write(address, result)?;
        Ok(())
      }

      // ROL
      0x26 => {
        // ROL zero page
        let address = self.fetch()?;
        let value = self.read(address as u16)?;
        let carry = self.registers.status_read(flags::CARRY) as u8;
        self.registers.status_write(flags::CARRY, value & 0x80 != 0);
        let result = value << 1 | carry;
        self.registers.status_set_nz(result);
        self.write(address as u16, result)?;
        Ok(())
      }
      0x2A => {
        // ROL accumulator
        let carry = self.registers.status_read(flags::CARRY) as u8;
        self
          .registers
          .status_write(flags::CARRY, self.registers.accumulator & 0x80 != 0);
        self.registers.accumulator = self.registers.accumulator << 1;
        self.registers.accumulator |= carry;
        self.registers.status_set_nz(self.registers.accumulator);
        Ok(())
      }
      0x2E => {
        // ROL absolute
        let address = self.fetch_word()?;
        let value = self.read(address)?;
        let carry = self.registers.status_read(flags::CARRY) as u8;
        self.registers.status_write(flags::CARRY, value & 0x80 != 0);
        let result = value << 1 | carry;
        self.registers.status_set_nz(result);
        self.write(address, result)?;
        Ok(())
      }
      0x36 => {
        // ROL zero page,X
        let base = self.fetch()?;
        let address = base + self.registers.x_index;
        let value = self.read(address as u16)?;
        let carry = self.registers.status_read(flags::CARRY) as u8;
        self.registers.status_write(flags::CARRY, value & 0x80 != 0);
        let result = value << 1 | carry;
        self.registers.status_set_nz(result);
        self.write(address as u16, result)?;
        Ok(())
      }
      0x3E => {
        // ROL absolute,X
        let base = self.fetch_word()?;
        let address = base + (self.registers.x_index as u16);
        let value = self.read(address)?;
        let carry = self.registers.status_read(flags::CARRY) as u8;
        self.registers.status_write(flags::CARRY, value & 0x80 != 0);
        let result = value << 1 | carry;
        self.registers.status_set_nz(result);
        self.write(address, result)?;
        Ok(())
      }

      // ROR
      0x66 => {
        // ROR zero page
        let address = self.fetch()?;
        let value = self.read(address as u16)?;
        let carry = self.registers.status_read(flags::CARRY) as u8;
        self.registers.status_write(flags::CARRY, value & 0x01 != 0);
        let result = value >> 1 | carry << 7;
        self.registers.status_set_nz(result);
        self.write(address as u16, result)?;
        Ok(())
      }
      0x6A => {
        // ROR accumulator
        let carry = self.registers.status_read(flags::CARRY) as u8;
        self
          .registers
          .status_write(flags::CARRY, self.registers.accumulator & 0x01 != 0);
        self.registers.accumulator = self.registers.accumulator >> 1;
        self.registers.accumulator |= carry << 7;
        self.registers.status_set_nz(self.registers.accumulator);
        Ok(())
      }
      0x6E => {
        // ROR absolute
        let address = self.fetch_word()?;
        let value = self.read(address)?;
        let carry = self.registers.status_read(flags::CARRY) as u8;
        self.registers.status_write(flags::CARRY, value & 0x01 != 0);
        let result = value >> 1 | carry << 7;
        self.registers.status_set_nz(result);
        self.write(address, result)?;
        Ok(())
      }
      0x76 => {
        // ROR zero page,X
        let base = self.fetch()?;
        let address = base + self.registers.x_index;
        let value = self.read(address as u16)?;
        let carry = self.registers.status_read(flags::CARRY) as u8;
        self.registers.status_write(flags::CARRY, value & 0x01 != 0);
        let result = value >> 1 | carry << 7;
        self.registers.status_set_nz(result);
        self.write(address as u16, result)?;
        Ok(())
      }
      0x7E => {
        // ROR absolute,X
        let base = self.fetch_word()?;
        let address = base + (self.registers.x_index as u16);
        let value = self.read(address)?;
        let carry = self.registers.status_read(flags::CARRY) as u8;
        self.registers.status_write(flags::CARRY, value & 0x01 != 0);
        let result = value >> 1 | carry << 7;
        self.registers.status_set_nz(result);
        self.write(address, result)?;
        Ok(())
      }

      // === LOGIC ===

      // AND
      0x21 => {
        // AND (indirect,X)
        let value = self.fetch_indirect_x()?;
        self.registers.accumulator &= value;
        self.registers.status_set_nz(self.registers.accumulator);
        Ok(())
      }
      0x25 => {
        // AND zero page
        let value = self.fetch_zero_page()?;
        self.registers.accumulator &= value;
        self.registers.status_set_nz(self.registers.accumulator);
        Ok(())
      }
      0x29 => {
        // AND immediate
        let value = self.fetch()?;
        self.registers.accumulator &= value;
        self.registers.status_set_nz(self.registers.accumulator);
        Ok(())
      }
      0x2D => {
        // AND absolute
        let value = self.fetch_absolute()?;
        self.registers.accumulator &= value;
        self.registers.status_set_nz(self.registers.accumulator);
        Ok(())
      }
      0x31 => {
        // AND (indirect),Y
        let value = self.fetch_indirect_y()?;
        self.registers.accumulator &= value;
        self.registers.status_set_nz(self.registers.accumulator);
        Ok(())
      }
      0x35 => {
        // AND zero page,X
        let value = self.fetch_zero_page_x()?;
        self.registers.accumulator &= value;
        self.registers.status_set_nz(self.registers.accumulator);
        Ok(())
      }
      0x39 => {
        // AND absolute,Y
        let value = self.fetch_absolute_y()?;
        self.registers.accumulator &= value;
        self.registers.status_set_nz(self.registers.accumulator);
        Ok(())
      }
      0x3D => {
        // AND absolute,X
        let value = self.fetch_absolute_x()?;
        self.registers.accumulator &= value;
        self.registers.status_set_nz(self.registers.accumulator);
        Ok(())
      }

      // BIT
      0x24 => {
        // BIT zero page
        let value = self.fetch_zero_page()?;
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
      0x2C => {
        // BIT absolute
        let value = self.fetch_absolute()?;
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

      // EOR
      0x41 => {
        // EOR (indirect,X)
        let value = self.fetch_indirect_x()?;
        self.registers.accumulator ^= value;
        self.registers.status_set_nz(self.registers.accumulator);
        Ok(())
      }
      0x45 => {
        // EOR zero page
        let value = self.fetch_zero_page()?;
        self.registers.accumulator ^= value;
        self.registers.status_set_nz(self.registers.accumulator);
        Ok(())
      }
      0x49 => {
        // EOR immediate
        let value = self.fetch()?;
        self.registers.accumulator ^= value;
        self.registers.status_set_nz(self.registers.accumulator);
        Ok(())
      }
      0x4D => {
        // EOR absolute
        let value = self.fetch_absolute()?;
        self.registers.accumulator ^= value;
        self.registers.status_set_nz(self.registers.accumulator);
        Ok(())
      }
      0x51 => {
        // EOR (indirect),Y
        let value = self.fetch_indirect_y()?;
        self.registers.accumulator ^= value;
        self.registers.status_set_nz(self.registers.accumulator);
        Ok(())
      }
      0x55 => {
        // EOR zero page,X
        let value = self.fetch_zero_page_x()?;
        self.registers.accumulator ^= value;
        self.registers.status_set_nz(self.registers.accumulator);
        Ok(())
      }
      0x59 => {
        // EOR absolute,Y
        let value = self.fetch_absolute_y()?;
        self.registers.accumulator ^= value;
        self.registers.status_set_nz(self.registers.accumulator);
        Ok(())
      }
      0x5D => {
        // EOR absolute,X
        let value = self.fetch_absolute_x()?;
        self.registers.accumulator ^= value;
        self.registers.status_set_nz(self.registers.accumulator);
        Ok(())
      }

      // ORA
      0x01 => {
        // ORA (indirect,X)
        let value = self.fetch_indirect_x()?;
        self.registers.accumulator |= value;
        self.registers.status_set_nz(self.registers.accumulator);
        Ok(())
      }
      0x05 => {
        // ORA zero page
        let value = self.fetch_zero_page()?;
        self.registers.accumulator |= value;
        self.registers.status_set_nz(self.registers.accumulator);
        Ok(())
      }
      0x09 => {
        // ORA immediate
        let value = self.fetch()?;
        self.registers.accumulator |= value;
        self.registers.status_set_nz(self.registers.accumulator);
        Ok(())
      }
      0x0D => {
        // ORA absolute
        let value = self.fetch_absolute()?;
        self.registers.accumulator |= value;
        self.registers.status_set_nz(self.registers.accumulator);
        Ok(())
      }
      0x11 => {
        // ORA (indirect),Y
        let value = self.fetch_indirect_y()?;
        self.registers.accumulator |= value;
        self.registers.status_set_nz(self.registers.accumulator);
        Ok(())
      }
      0x15 => {
        // ORA zero page,X
        let value = self.fetch_zero_page_x()?;
        self.registers.accumulator |= value;
        self.registers.status_set_nz(self.registers.accumulator);
        Ok(())
      }
      0x19 => {
        // ORA absolute,Y
        let value = self.fetch_absolute_y()?;
        self.registers.accumulator |= value;
        self.registers.status_set_nz(self.registers.accumulator);
        Ok(())
      }
      0x1D => {
        // ORA absolute,X
        let value = self.fetch_absolute_x()?;
        self.registers.accumulator |= value;
        self.registers.status_set_nz(self.registers.accumulator);
        Ok(())
      }

      // === ARITHMETIC ===

      // ADC
      0x61 => {
        // ADC (indirect,X)
        let value = self.fetch_indirect_x()?;
        self.registers.alu_add(value);
        Ok(())
      }
      0x65 => {
        // ADC zero page
        let value = self.fetch_zero_page()?;
        self.registers.alu_add(value);
        Ok(())
      }
      0x69 => {
        // ADC immediate
        let value = self.fetch()?;
        self.registers.alu_add(value);
        Ok(())
      }
      0x6D => {
        // ADC absolute
        let value = self.fetch_absolute()?;
        self.registers.alu_add(value);
        Ok(())
      }
      0x71 => {
        // ADC (indirect),Y
        let value = self.fetch_indirect_y()?;
        self.registers.alu_add(value);
        Ok(())
      }
      0x75 => {
        // ADC zero page,X
        let value = self.fetch_zero_page_x()?;
        self.registers.alu_add(value);
        Ok(())
      }

      0x79 => {
        // ADC absolute,Y
        let value = self.fetch_absolute_y()?;
        self.registers.alu_add(value);
        Ok(())
      }
      0x7D => {
        // ADC absolute,X
        let value = self.fetch_absolute_x()?;
        self.registers.alu_add(value);
        Ok(())
      }

      // CMP
      0xC1 => {
        // CMP (indirect,X)
        let value = self.fetch_indirect_x()?;
        self
          .registers
          .alu_compare(self.registers.accumulator, value);
        Ok(())
      }
      0xC5 => {
        // CMP zero page
        let value = self.fetch_zero_page()?;
        self
          .registers
          .alu_compare(self.registers.accumulator, value);
        Ok(())
      }
      0xC9 => {
        // CMP immediate
        let value = self.fetch()?;
        self
          .registers
          .alu_compare(self.registers.accumulator, value);
        Ok(())
      }
      0xCD => {
        // CMP absolute
        let value = self.fetch_absolute()?;
        self
          .registers
          .alu_compare(self.registers.accumulator, value);
        Ok(())
      }
      0xD1 => {
        // CMP (indirect),Y
        let value = self.fetch_indirect_y()?;
        self
          .registers
          .alu_compare(self.registers.accumulator, value);
        Ok(())
      }
      0xD5 => {
        // CMP zero page,X
        let value = self.fetch_zero_page_x()?;
        self
          .registers
          .alu_compare(self.registers.accumulator, value);
        Ok(())
      }
      0xD9 => {
        // CMP absolute,Y
        let value = self.fetch_absolute_y()?;
        self
          .registers
          .alu_compare(self.registers.accumulator, value);
        Ok(())
      }
      0xDD => {
        // CMP absolute,X
        let value = self.fetch_absolute_x()?;
        self
          .registers
          .alu_compare(self.registers.accumulator, value);
        Ok(())
      }

      // CPX
      0xE0 => {
        // CPX immediate
        let value = self.fetch()?;
        self.registers.alu_compare(self.registers.x_index, value);
        Ok(())
      }
      0xE4 => {
        // CPX zero page
        let value = self.fetch_zero_page()?;
        self.registers.alu_compare(self.registers.x_index, value);
        Ok(())
      }
      0xEC => {
        // CPX absolute
        let value = self.fetch_absolute()?;
        self.registers.alu_compare(self.registers.x_index, value);
        Ok(())
      }

      // CPY
      0xC0 => {
        // CPY immediate
        let value = self.fetch()?;
        self.registers.alu_compare(self.registers.y_index, value);
        Ok(())
      }
      0xC4 => {
        // CPY zero page
        let value = self.fetch_zero_page()?;
        self.registers.alu_compare(self.registers.y_index, value);
        Ok(())
      }
      0xCC => {
        // CPY absolute
        let value = self.fetch_absolute()?;
        self.registers.alu_compare(self.registers.y_index, value);
        Ok(())
      }

      // SBC
      0xE1 => {
        // SBC (indirect,X)
        let value = self.fetch_indirect_x()?;
        self.registers.alu_subtract(value);
        Ok(())
      }
      0xE5 => {
        // SBC zero page
        let value = self.fetch_zero_page()?;
        self.registers.alu_subtract(value);
        Ok(())
      }
      0xE9 => {
        // SBC immediate
        let value = self.fetch()?;
        self.registers.alu_subtract(value);
        Ok(())
      }
      0xED => {
        // SBC absolute
        let value = self.fetch_absolute()?;
        self.registers.alu_subtract(value);
        Ok(())
      }
      0xF1 => {
        // SBC (indirect),Y
        let value = self.fetch_indirect_y()?;
        self.registers.alu_subtract(value);
        Ok(())
      }
      0xF5 => {
        // SBC zero page,X
        let value = self.fetch_zero_page_x()?;
        self.registers.alu_subtract(value);
        Ok(())
      }
      0xF9 => {
        // SBC absolute,Y
        let value = self.fetch_absolute_y()?;
        self.registers.alu_subtract(value);
        Ok(())
      }
      0xFD => {
        // SBC absolute,X
        let value = self.fetch_absolute_x()?;
        self.registers.alu_subtract(value);
        Ok(())
      }

      // === INCREMENT ===

      // DEC
      0xC6 => {
        // DEC zero page
        let address = self.fetch()?;
        let value = self.read(address as u16)?;
        self.registers.status_set_nz(value - 1);
        self.write(address as u16, value - 1)?;
        Ok(())
      }
      0xCE => {
        // DEC absolute
        let address = self.fetch_word()?;
        let value = self.read(address)?;
        self.registers.status_set_nz(value - 1);
        self.write(address as u16, value - 1)?;
        Ok(())
      }
      0xD6 => {
        // DEC zero page,X
        let base = self.fetch()?;
        let address = base + self.registers.x_index;
        let value = self.read(address as u16)?;
        self.registers.status_set_nz(value - 1);
        self.write(address as u16, value - 1)?;
        Ok(())
      }
      0xDE => {
        // DEC absolute,X
        let base = self.fetch_word()?;
        let address = base + (self.registers.x_index as u16);
        let value = self.read(address)?;
        self.registers.status_set_nz(value - 1);
        self.write(address as u16, value - 1)?;
        Ok(())
      }

      // DEX
      0xCA => {
        // DEX
        self.registers.x_index -= 1;
        self.registers.status_set_nz(self.registers.x_index);
        Ok(())
      }

      // DEY
      0x88 => {
        // DEY
        self.registers.y_index -= 1;
        self.registers.status_set_nz(self.registers.y_index);
        Ok(())
      }

      // INC
      0xE6 => {
        // INC zero page
        let address = self.fetch()?;
        let value = self.read(address as u16)?;
        self.registers.status_set_nz(value + 1);
        self.write(address as u16, value + 1)?;
        Ok(())
      }
      0xEE => {
        // INC absolute
        let address = self.fetch_word()?;
        let value = self.read(address)?;
        self.registers.status_set_nz(value + 1);
        self.write(address as u16, value + 1)?;
        Ok(())
      }
      0xF6 => {
        // INC zero page,X
        let base = self.fetch()?;
        let address = base + self.registers.x_index;
        let value = self.read(address as u16)?;
        self.registers.status_set_nz(value + 1);
        self.write(address as u16, value + 1)?;
        Ok(())
      }
      0xFE => {
        // INC absolute,X
        let base = self.fetch_word()?;
        let address = base + (self.registers.x_index as u16);
        let value = self.read(address)?;
        self.registers.status_set_nz(value + 1);
        self.write(address as u16, value + 1)?;
        Ok(())
      }

      // INX
      0xE8 => {
        // INX
        self.registers.x_index += 1;
        self.registers.status_set_nz(self.registers.x_index);
        Ok(())
      }

      // INY
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
      0x4C => {
        // JMP absolute
        let address = self.fetch_word()?;
        self.registers.pc_load(address);
        Ok(())
      }
      0x6C => {
        // JMP (indirect)
        let indirect = self.fetch_word()?;
        let address = self.read_word(indirect)?;
        self.registers.pc_load(address);
        Ok(())
      }
      0x20 => {
        // JSR absolute
        let address = self.fetch_word()?;
        let return_to = self.registers.pc_address() + 1;
        self.push((return_to & 0xFF >> 8) as u8)?;
        self.push((return_to & 0xFF) as u8)?;
        self.registers.pc_load(address);
        Ok(())
      }
      0x40 => {
        // RTI
        Err(())
      }
      0x60 => {
        // RTS
        let pc_low = self.pop()?;
        let pc_high = self.pop()?;

        self
          .registers
          .pc_load((pc_high as u16 | (pc_low as u16) << 8) + 1);
        Ok(())
      }

      // === BRANCH ===
      0x90 => {
        // BCC
        let offset = self.fetch()? as i8;
        if !self.registers.status_read(flags::CARRY) {
          self.registers.pc_offset(offset);
        }
        Ok(())
      }
      0xB0 => {
        // BCS
        let offset = self.fetch()? as i8;
        if self.registers.status_read(flags::CARRY) {
          self.registers.pc_offset(offset);
        }
        Ok(())
      }
      0xF0 => {
        // BEQ
        let offset = self.fetch()? as i8;
        if self.registers.status_read(flags::ZERO) {
          self.registers.pc_offset(offset);
        }
        Ok(())
      }
      0x30 => {
        // BMI
        let offset = self.fetch()? as i8;
        if self.registers.status_read(flags::NEGATIVE) {
          self.registers.pc_offset(offset);
        }
        Ok(())
      }
      0xD0 => {
        // BNE
        let offset = self.fetch()? as i8;
        if !self.registers.status_read(flags::ZERO) {
          self.registers.pc_offset(offset);
        }
        Ok(())
      }
      0x10 => {
        // BPL
        let offset = self.fetch()? as i8;
        if !self.registers.status_read(flags::NEGATIVE) {
          self.registers.pc_offset(offset);
        }
        Ok(())
      }
      0x50 => {
        // BVC
        let offset = self.fetch()? as i8;
        if !self.registers.status_read(flags::OVERFLOW) {
          self.registers.pc_offset(offset);
        }
        Ok(())
      }
      0x70 => {
        // BVS
        let offset = self.fetch()? as i8;
        if self.registers.status_read(flags::OVERFLOW) {
          self.registers.pc_offset(offset);
        }
        Ok(())
      }

      // === FLAGS ===
      0x18 => {
        // CLC
        self.registers.status_clear(flags::CARRY);
        Ok(())
      }
      0xD8 => {
        // CLD
        self.registers.status_clear(flags::DECIMAL);
        Ok(())
      }
      0x58 => {
        // CLI
        self.registers.status_clear(flags::INTERRUPT);
        Ok(())
      }
      0xB8 => {
        // CLV
        self.registers.status_clear(flags::OVERFLOW);
        Ok(())
      }
      0x38 => {
        // SEC
        self.registers.status_set(flags::CARRY);
        Ok(())
      }
      0xF8 => {
        // SED
        self.registers.status_set(flags::DECIMAL);
        Ok(())
      }
      0x78 => {
        // SEI
        self.registers.status_set(flags::INTERRUPT);
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
