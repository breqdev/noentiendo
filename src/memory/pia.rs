use crate::memory::{ActiveInterrupt, Memory};

// MOS 6520

// PORT: 8 individual lines
// DDR (Data Direction Register): each bit controls whether the line is an input (0) or output (1)
// Control Register:
//  bit 7: IRQ 1
//  bit 6: IRQ 2
//  bits 5, 4, 3: CA2 (interrupt status control)
//  bit 2: enable accessing DDR
//  bits 1, 0: CA1 (interrupt status control)

pub enum PortName {
  A,
  B,
}

pub trait Port: Send {
  fn read(&mut self) -> u8;
  fn write(&mut self, value: u8);

  fn reset(&mut self);
}

pub struct NullPort {}

impl NullPort {
  pub fn new() -> Self {
    Self {}
  }
}

impl Port for NullPort {
  fn read(&mut self) -> u8 {
    0
  }

  fn write(&mut self, _value: u8) {}

  fn reset(&mut self) {}
}

struct PortRegisters {
  port: Box<dyn Port>,
  ddr: u8, // data direction register, each bit controls whether the line is an input (0) or output (1)
  control: u8, // control register
}

impl PortRegisters {
  fn new(port: Box<dyn Port>) -> Self {
    Self {
      port,
      ddr: 0,
      control: 0,
    }
  }

  fn reset(&mut self) {
    self.ddr = 0;
    self.control = 0;

    self.port.reset();
  }
}

pub mod ControlBits {
  pub const IRQ1: u8 = 0b10000000;
  pub const IRQ2: u8 = 0b01000000;
  pub const CA2: u8 = 0b00110000;
  pub const DDR: u8 = 0b00000100;
  pub const CA1: u8 = 0b00000011;
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
    let port = if address & 0b10 == 0 {
      &mut self.a
    } else {
      &mut self.b
    };

    if address & 0b01 == 0 {
      if port.control & ControlBits::DDR != 0 {
        port.port.read() & !port.ddr
      } else {
        port.ddr
      }
    } else {
      port.control
    }
  }

  fn write(&mut self, address: u16, value: u8) {
    let port = if address & 0b10 == 0 {
      &mut self.a
    } else {
      &mut self.b
    };

    if address & 0b01 == 0 {
      if port.control & ControlBits::DDR != 0 {
        port.port.write(value & port.ddr);
      } else {
        port.ddr = value;
      }
    } else {
      port.control = value;
    }
  }

  fn reset(&mut self) {
    self.a.reset();
    self.b.reset();
  }

  fn poll(&mut self) -> ActiveInterrupt {
    ActiveInterrupt::None
  }
}
