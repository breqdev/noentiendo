use crate::memory::{ActiveInterrupt, Memory, Port, SystemInfo};

// MOS 6520

struct PortRegisters {
  port: Box<dyn Port>,
  writes: u8,      // if the DDR is write, allow reading the current written value
  ddr: u8, // data direction register, each bit controls whether the line is an input (0) or output (1)
  pub control: u8, // control register
}

impl PortRegisters {
  pub fn new(port: Box<dyn Port>) -> Self {
    Self {
      port,
      writes: 0,
      ddr: 0,
      control: 0,
    }
  }

  pub fn read(&mut self) -> u8 {
    if self.control & control_bits::DDR_SELECT != 0 {
      (self.port.read() & !self.ddr) | (self.writes & self.ddr)
    } else {
      self.ddr
    }
  }

  pub fn write(&mut self, value: u8) {
    if self.control & control_bits::DDR_SELECT != 0 {
      self.writes = value;
      self.port.write(value & self.ddr);
    } else {
      self.ddr = value;
    }
  }

  pub fn poll(&mut self, info: &SystemInfo) -> bool {
    self.port.poll(info)
  }

  pub fn reset(&mut self) {
    self.ddr = 0;
    self.control = 0;

    self.port.reset();
  }
}

pub mod control_bits {
  pub const C1_ACTIVE_TRANSITION_FLAG: u8 = 0b10000000; // 1 = 0->1, 0 = 1->0
  pub const C2_ACTIVE_TRANSITION_FLAG: u8 = 0b01000000;
  pub const C2_DIRECTION: u8 = 0b00100000; // 1 = output, 0 = input
  pub const C2_CONTROL: u8 = 0b00011000; // ???
  pub const DDR_SELECT: u8 = 0b00000100; // enable accessing DDR
  pub const C1_CONTROL: u8 = 0b00000011; // interrupt status control
}

pub struct PIA {
  a: PortRegisters,
  b: PortRegisters,
}

impl PIA {
  pub fn new(a: Box<dyn Port>, b: Box<dyn Port>) -> Self {
    Self {
      a: PortRegisters::new(a),
      b: PortRegisters::new(b),
    }
  }
}

impl Memory for PIA {
  fn read(&mut self, address: u16) -> u8 {
    match address % 0x04 {
      0x00 => self.a.read(),
      0x01 => self.a.control,
      0x02 => self.b.read(),
      0x03 => self.b.control,
      _ => unreachable!(),
    }
  }

  fn write(&mut self, address: u16, value: u8) {
    match address % 0x04 {
      0x00 => self.a.write(value),
      0x01 => self.a.control = value,
      0x02 => self.b.write(value),
      0x03 => self.b.control = value,
      _ => unreachable!(),
    }
  }

  fn reset(&mut self) {
    self.a.reset();
    self.b.reset();
  }

  fn poll(&mut self, info: &SystemInfo) -> ActiveInterrupt {
    let a = self.a.poll(info);
    let b = self.b.poll(info);

    if a || b {
      ActiveInterrupt::IRQ
    } else {
      ActiveInterrupt::None
    }
  }
}
