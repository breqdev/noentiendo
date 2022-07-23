pub struct Registers {
  pub a: u8,
  pub x: u8,
  pub y: u8,
  pub sp: StackPointer,
  pub pc: ProgramCounter,
  pub sr: StatusRegister,
}

pub mod flags {
  pub const CARRY: u8 = 0b00000001;
  pub const ZERO: u8 = 0b00000010;
  pub const INTERRUPT: u8 = 0b00000100;
  pub const DECIMAL: u8 = 0b00001000;
  pub const _BREAK: u8 = 0b00010000;
  pub const _UNUSED: u8 = 0b00100000;
  pub const OVERFLOW: u8 = 0b01000000;
  pub const NEGATIVE: u8 = 0b10000000;
}

pub struct StackPointer {
  value: u8,
}

impl StackPointer {
  fn new() -> Self {
    Self { value: 0xFF }
  }

  pub fn push(&mut self) {
    self.value = self.value.wrapping_sub(1);
  }

  pub fn pop(&mut self) {
    self.value = self.value.wrapping_add(1);
  }

  pub fn get(&self) -> u8 {
    self.value
  }

  pub fn set(&mut self, value: u8) {
    self.value = value;
  }

  pub fn address(&self) -> u16 {
    0x0100 + self.value as u16
  }
}

pub struct ProgramCounter {
  value: u16,
}

impl ProgramCounter {
  fn new() -> Self {
    Self { value: 0x0000 }
  }

  pub fn address(&self) -> u16 {
    self.value
  }

  pub fn increment(&mut self) {
    self.value += 1;
  }

  pub fn load(&mut self, address: u16) {
    self.value = address;
  }

  pub fn offset(&mut self, offset: i8) {
    self.value = self.value.wrapping_add(offset as u16);
  }
}

pub struct StatusRegister {
  value: u8,
}

impl StatusRegister {
  fn new() -> Self {
    Self { value: 0b00110000 }
  }

  pub fn write(&mut self, flag: u8, value: bool) {
    if value {
      self.set(flag);
    } else {
      self.clear(flag);
    }
  }

  pub fn set(&mut self, flag: u8) {
    self.value |= flag;
  }

  pub fn clear(&mut self, flag: u8) {
    self.value &= !flag;
  }

  pub fn read(&self, flag: u8) -> bool {
    self.value & flag != 0
  }

  pub fn load(&mut self, value: u8) {
    self.value = value;
  }

  pub fn get(&self) -> u8 {
    self.value
  }

  pub fn set_nz(&mut self, value: u8) {
    self.write(flags::NEGATIVE, value & 0x80 != 0);
    self.write(flags::ZERO, value == 0);
  }
}

pub trait ALU {
  fn alu_add(&mut self, value: u8);
  fn alu_subtract(&mut self, value: u8);
  fn alu_compare(&mut self, register: u8, value: u8);
}

impl ALU for Registers {
  fn alu_add(&mut self, value: u8) {
    let sum = (self.a as u16)
      .wrapping_add(value as u16)
      .wrapping_add(self.sr.read(flags::CARRY) as u16);

    self.sr.write(flags::CARRY, sum > 0xFF);
    self.sr.write(
      flags::OVERFLOW,
      !(self.a ^ value) & (self.a ^ sum as u8) & 0x80 != 0,
    );

    self.a = sum as u8;
    self.sr.set_nz(self.a);
  }

  fn alu_subtract(&mut self, value: u8) {
    self.alu_add(!value);
  }

  fn alu_compare(&mut self, register: u8, value: u8) {
    self.sr.write(flags::CARRY, register >= value);
    self.sr.write(flags::ZERO, register == value);
    let negative = register.wrapping_sub(value) & 0x80 != 0;
    self.sr.write(flags::NEGATIVE, negative);
  }
}

impl Registers {
  pub fn new() -> Registers {
    Registers {
      a: 0,
      x: 0,
      y: 0,
      sp: StackPointer::new(),
      pc: ProgramCounter::new(),
      sr: StatusRegister::new(),
    }
  }

  pub fn reset(&mut self) {
    self.a = 0;
    self.x = 0;
    self.y = 0;
    self.sp = StackPointer::new();
    self.pc = ProgramCounter::new();
    self.sr = StatusRegister::new();
  }
}
