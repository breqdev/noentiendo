use crate::memory::{ActiveInterrupt, Memory, SystemInfo};

// MOS 6520

pub trait Port: Send {
  fn read(&mut self) -> u8;
  fn write(&mut self, value: u8);
  fn poll(&mut self, info: &SystemInfo) -> bool;
  fn reset(&mut self);
}

pub struct NullPort {
  warn: Option<&'static str>,
}

impl NullPort {
  pub fn new() -> Self {
    Self {
      warn: None,
    }
  }

  pub fn with_warnings(message: &'static str) -> Self {
    Self {
      warn: Some(message),
    }
  }
}

impl Port for NullPort {
  fn read(&mut self) -> u8 {
    if let Some(message) = self.warn {
      println!("attempted to read from {} at address {:04x}", message, 0);
    }
    0
  }

  fn write(&mut self, _value: u8) {
    if let Some(message) = self.warn {
      println!("attempted to write to {} at address {:04x}", message, 0);
    }
  }

  fn poll(&mut self, _info: &SystemInfo) -> bool {
    false
  }

  fn reset(&mut self) {}
}

struct PortRegisters {
  port: Box<dyn Port>,
  writes: u8,  // if the DDR is write, allow reading the current written value
  ddr: u8, // data direction register, each bit controls whether the line is an input (0) or output (1)
  control: u8, // control register
}

impl PortRegisters {
  fn new(port: Box<dyn Port>) -> Self {
    Self {
      port,
      writes: 0,
      ddr: 0,
      control: 0,
    }
  }

  fn poll(&mut self, info: &SystemInfo) -> bool {
    self.port.poll(info)
  }

  fn reset(&mut self) {
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
    let port = if address & 0b10 == 0 {
      &mut self.a
    } else {
      &mut self.b
    };

    if address & 0b01 == 0 {
      if port.control & control_bits::DDR_SELECT != 0 {
        (port.port.read() & !port.ddr) | (port.writes & port.ddr)
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
      if port.control & control_bits::DDR_SELECT != 0 {
        port.writes = value & port.ddr;
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
