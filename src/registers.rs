pub struct Registers {
  pub accumulator: u8,
  x_index: u8,
  y_index: u8,
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
