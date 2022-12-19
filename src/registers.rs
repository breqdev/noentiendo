/// The registers inside of a MOS 6502 processor.
#[derive(Default)]
pub struct Registers {
  /// The accumulator register, used in ALU operations.
  pub a: u8,

  /// The X index register, used in indexed addressing.
  pub x: u8,

  /// The Y index register, used in indexed addressing.
  pub y: u8,

  /// The stack pointer register, used for the stack from 0x0100 to 0x01FF.
  pub sp: StackPointer,

  /// The program counter register, used to represent the current instruction.
  pub pc: ProgramCounter,

  /// The processor status register, which represents the current state of the
  /// processor with a series of flags.
  pub sr: StatusRegister,
}

/// Flags used in the MOS 6502 status register.
pub mod flags {
  /// The previous operation has resulted in a carry.
  pub const CARRY: u8 = 0b00000001;

  /// The result of the previous operation is zero.
  pub const ZERO: u8 = 0b00000010;

  /// Interrupts are disabled.
  pub const INTERRUPT: u8 = 0b00000100;

  /// ALU operations (addition, subtraction, compare) are performed assuming
  /// the arguments are stored in Binary Coded Decimal (BCD) format.
  pub const DECIMAL: u8 = 0b00001000;

  /// An interrupt has occurred as the result of a BRK instruction.
  pub const BREAK: u8 = 0b00010000;

  /// This flag is not used.
  pub const UNUSED: u8 = 0b00100000;

  /// The previous operation has resulted in an arithmetic overflow.
  pub const OVERFLOW: u8 = 0b01000000;

  /// The result of the previous operation is negative.
  pub const NEGATIVE: u8 = 0b10000000;
}

/// The stack pointer register, representing an address in the stack from 0x0100
/// to 0x01FF. The stack is used for storing the return address of a subroutine
/// call, as well as the processor status register when an interrupt occurs.
/// The stack register is usually initialized by the operating system to 0x01FF,
/// and grows upward (i.e., pushing decrements the value, while popping
/// increments it).
#[derive(Default)]
pub struct StackPointer {
  value: u8,
}

impl StackPointer {
  fn new() -> Self {
    Self { value: 0xFF }
  }

  /// Decrement the stack pointer, as a value was just pushed to the stack.
  pub fn push(&mut self) {
    self.value = self.value.wrapping_sub(1);
  }

  /// Increment the stack pointer, as a value was just popped from the stack.
  pub fn pop(&mut self) {
    self.value = self.value.wrapping_add(1);
  }

  /// Get the current value of the stack pointer register.
  pub fn get(&self) -> u8 {
    self.value
  }

  /// Set the current value of the stack pointer register.
  pub fn set(&mut self, value: u8) {
    self.value = value;
  }

  /// Get the address in memory that the stack pointer is currently pointing to.
  /// (This is the stack pointer's value plus 0x0100.)
  pub fn address(&self) -> u16 {
    0x0100 + self.value as u16
  }
}

/// The program counter register, which points to the current instruction being
/// executed.
#[derive(Default)]
pub struct ProgramCounter {
  value: u16,
}

impl ProgramCounter {
  fn new() -> Self {
    Self { value: 0x0000 }
  }

  /// Get the current address pointed to by the program counter.
  pub fn address(&self) -> u16 {
    self.value
  }

  /// Advance the program counter by 1.
  pub fn increment(&mut self) {
    self.value += 1;
  }

  /// Load the given address into the program counter.
  pub fn load(&mut self, address: u16) {
    self.value = address;
  }

  /// Offset the program counter by the given signed value.
  pub fn offset(&mut self, offset: i8) {
    self.value = self.value.wrapping_add(offset as u16);
  }
}

/// The processor status register, which contains a series of flags that
/// represent the current state of the processor.
#[derive(Default)]
pub struct StatusRegister {
  value: u8,
}

impl StatusRegister {
  fn new() -> Self {
    Self { value: 0b00110100 }
  }

  /// If the given value is true, set the given flag; otherwise, clear it.
  pub fn write(&mut self, flag: u8, value: bool) {
    if value {
      self.set(flag);
    } else {
      self.clear(flag);
    }
  }

  /// Set the given flag (i.e., set the corresponding bit to 1).
  pub fn set(&mut self, flag: u8) {
    self.value |= flag;
  }

  /// Clear the given flag (i.e., set the corresponding bit to 0).
  pub fn clear(&mut self, flag: u8) {
    self.value &= !flag;
  }

  /// Get the value of the given flag.
  pub fn read(&self, flag: u8) -> bool {
    self.value & flag != 0
  }

  /// Load an 8-bit value into the status register.
  pub fn load(&mut self, value: u8) {
    self.value = value | flags::UNUSED | flags::BREAK;
  }

  /// Get the current 8-bit value of the status register.
  pub fn get(&self) -> u8 {
    self.value
  }

  /// Set the N and Z flags based on the given value,
  /// assuming this value is the result of a processor operation.
  pub fn set_nz(&mut self, value: u8) {
    self.write(flags::NEGATIVE, value & 0x80 != 0);
    self.write(flags::ZERO, value == 0);
  }
}

/// The Arithmetic Logic Unit (ALU) of the MOS 6502 processor.
pub trait ALU {
  /// Add the given value to the accumulator, setting the N and Z flags, and
  /// storing the result in the accumulator. If the carry flag is set, add 1
  /// to the result. If the result causes a carry, set the carry flag.
  fn alu_add(&mut self, value: u8);

  /// Subtract the given value from the accumulator, setting the N and Z flags,
  /// and storing the result in the accumulator. If the carry flag is *cleared*,
  /// *subtract* 1 from the result. If the result causes a borrow, clear the
  /// carry flag. If you want to perform a subtraction without borrowing,
  /// set the carry flag before calling this function.
  fn alu_subtract(&mut self, value: u8);

  /// Compare the given value with the accumulator. This is done by performing
  /// a subtraction, setting the N and Z flags, and discarding the result.
  fn alu_compare(&mut self, register: u8, value: u8);
}

impl ALU for Registers {
  fn alu_add(&mut self, value: u8) {
    if !self.sr.read(flags::DECIMAL) {
      let sum = (self.a as u16)
        .wrapping_add(value as u16)
        .wrapping_add(self.sr.read(flags::CARRY) as u16);

      self.sr.write(flags::CARRY, sum > 0xFF);
      self.sr.write(
        flags::OVERFLOW,
        !(self.a ^ value) & (self.a ^ sum as u8) & 0x80 != 0,
      );

      let sum = sum as u8;
      self.sr.set_nz(sum);
      self.a = sum;
    } else {
      let mut lsd = (self.a & 0x0F) + (value & 0x0F) + self.sr.read(flags::CARRY) as u8;
      let mut msd = ((self.a & 0xF0) as u16) + ((value & 0xF0) as u16);

      if lsd > 0x09 {
        msd += 0x10;
        lsd += 0x06;
        lsd &= 0x0F;
      }

      if msd > 0x90 {
        msd += 0x60;
      }

      self.sr.write(flags::CARRY, (msd & 0xFF00) != 0);

      let sum = (msd as u8) | (lsd as u8);
      self.sr.set_nz(sum);
      self.a = sum;
    }
  }

  fn alu_subtract(&mut self, value: u8) {
    if !self.sr.read(flags::DECIMAL) {
      let sum = (self.a as u16)
        .wrapping_add(!value as u16)
        .wrapping_add(self.sr.read(flags::CARRY) as u16);

      self.sr.write(flags::CARRY, sum > 0xFF);
      self.sr.write(
        flags::OVERFLOW,
        (self.a ^ value) & (self.a ^ sum as u8) & 0x80 != 0,
      );

      let sum = sum as u8;
      self.sr.set_nz(sum);
      self.a = sum;
    } else {
      let mut lsd = (self.a & 0x0F) + (0x09 - (value & 0x0F)) + self.sr.read(flags::CARRY) as u8;
      let mut msd = ((self.a & 0xF0) as u16) + ((0x90 - (value & 0xF0)) as u16);

      if lsd > 0x09 {
        msd += 0x10;
        lsd += 0x06;
        lsd &= 0x0F;
      }

      if msd > 0x90 {
        msd += 0x60;
      }

      self.sr.write(flags::CARRY, (msd & 0xFF00) != 0);

      let sum = (msd as u8) | (lsd as u8);
      self.sr.set_nz(sum);
      self.a = sum;
    }
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
