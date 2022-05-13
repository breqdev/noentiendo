pub struct Registers {
  pub accumulator: u8,
  pub x_index: u8,
  pub y_index: u8,
  stack_pointer: u8,
  program_counter: u16,
  status_register: u8,
}

pub mod flags {
  pub const CARRY: u8 = 0b00000001;
  pub const ZERO: u8 = 0b00000010;
  pub const INTERRUPT: u8 = 0b00000100;
  pub const DECIMAL: u8 = 0b00001000;
  pub const BREAK: u8 = 0b00010000;
  pub const UNUSED: u8 = 0b00100000;
  pub const OVERFLOW: u8 = 0b01000000;
  pub const NEGATIVE: u8 = 0b10000000;
}

pub trait StackPointer {
  fn stack_push(&mut self);
  fn stack_pop(&mut self);
  fn stack_address(&self) -> u16;
}

impl StackPointer for Registers {
  fn stack_push(&mut self) {
    self.stack_pointer -= 1;
  }

  fn stack_pop(&mut self) {
    self.stack_pointer += 1;
  }

  fn stack_address(&self) -> u16 {
    0x0100 + self.stack_pointer as u16
  }
}

pub trait ProgramCounter {
  fn pc_address(&self) -> u16;
  fn pc_increment(&mut self);
  fn pc_load(&mut self, address: u16);
}

impl ProgramCounter for Registers {
  fn pc_address(&self) -> u16 {
    self.program_counter
  }

  fn pc_increment(&mut self) {
    self.program_counter += 1;
  }

  fn pc_load(&mut self, address: u16) {
    self.program_counter = address;
  }
}

pub trait StatusRegister {
  fn status_write(&mut self, flag: u8, value: bool);
  fn status_set(&mut self, flag: u8);
  fn status_clear(&mut self, flag: u8);
  fn status_read(&self, flag: u8) -> bool;

  fn status_set_nz(&mut self, value: u8);
}

impl StatusRegister for Registers {
  fn status_write(&mut self, flag: u8, value: bool) {
    if value {
      self.status_set(flag);
    } else {
      self.status_clear(flag);
    }
  }

  fn status_set(&mut self, flag: u8) {
    self.status_register |= flag;
  }

  fn status_clear(&mut self, flag: u8) {
    self.status_register &= !flag;
  }

  fn status_read(&self, flag: u8) -> bool {
    self.status_register & flag != 0
  }

  fn status_set_nz(&mut self, value: u8) {
    self.status_write(flags::NEGATIVE, value & 0x80 != 0);
    self.status_write(flags::ZERO, value != 0);
  }
}

pub trait ALU {
  fn alu_add(&mut self, value: u8);
  fn alu_subtract(&mut self, value: u8);
}

impl ALU for Registers {
  fn alu_add(&mut self, value: u8) {
    let sum = self.accumulator as u16 + value as u16 + self.status_read(flags::CARRY) as u16;

    self.status_write(flags::CARRY, sum > 0xFF);
    self.status_write(
      flags::OVERFLOW,
      !(self.accumulator ^ value) & (self.accumulator ^ sum as u8) & 0x80 != 0,
    );

    self.accumulator = sum as u8;
    self.status_set_nz(self.accumulator);
  }

  fn alu_subtract(&mut self, value: u8) {
    self.alu_add(!value);
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

  pub fn reset(&mut self) {
    self.accumulator = 0;
    self.x_index = 0;
    self.y_index = 0;
    self.stack_pointer = 0xFF;
    self.program_counter = 0x0000;
    self.status_register = 0;
  }
}
