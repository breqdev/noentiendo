use crate::memory::{ActiveInterrupt, Memory, Port, SystemInfo};

// MOS 6522

struct PortRegisters {
  port: Box<dyn Port>,
  writes: u8, // if the DDR is write, allow reading the current written value
  ddr: u8, // data direction register, each bit controls whether the line is an input (0) or output (1)
}

impl PortRegisters {
  pub fn new(port: Box<dyn Port>) -> Self {
    Self {
      port,
      writes: 0,
      ddr: 0,
    }
  }

  pub fn read(&mut self) -> u8 {
    (self.port.read() & !self.ddr) | (self.writes & self.ddr)
  }

  pub fn write(&mut self, value: u8) {
    self.writes = value;
    self.port.write(value & self.ddr);
  }

  pub fn poll(&mut self, info: &SystemInfo) -> bool {
    self.port.poll(info)
  }

  pub fn reset(&mut self) {
    self.ddr = 0;

    self.port.reset();
  }
}

struct Timer {
  latch: u16,
  counter: u16,
  control: u8,
  interrupt: bool,
}

impl Timer {
  pub fn new() -> Self {
    Self {
      latch: 0,
      counter: 0,
      control: 0,
      interrupt: false,
    }
  }
}

pub struct VIA {
  a: PortRegisters,
  b: PortRegisters,
  t1: Timer,
  t2: Timer,
  sr: u8,  // shift register
  acr: u8, // auxiliary control register
  pcr: u8, // peripheral control register
  ifr: u8, // interrupt flag register
  ier: u8, // interrupt enable register
}

impl VIA {
  pub fn new(a: Box<dyn Port>, b: Box<dyn Port>) -> Self {
    Self {
      a: PortRegisters::new(a),
      b: PortRegisters::new(b),
      t1: Timer::new(),
      t2: Timer::new(),
      sr: 0,
      acr: 0,
      pcr: 0,
      ifr: 0,
      ier: 0,
    }
  }
}

impl Memory for VIA {
  fn read(&mut self, address: u16) -> u8 {
    match address % 0x10 {
      0x00 => self.b.read(),
      0x01 => self.a.read(), // TODO: controls handshake?
      0x02 => self.b.ddr,
      0x03 => self.a.ddr,
      0x04 => {
        self.t1.interrupt = false;
        (self.t1.counter & 0xff) as u8
      }
      0x05 => ((self.t1.counter >> 8) & 0xff) as u8,
      0x06 => (self.t1.latch & 0xff) as u8,
      0x07 => ((self.t1.latch >> 8) & 0xff) as u8,
      0x08 => {
        self.t2.interrupt = false;
        (self.t2.counter & 0xff) as u8
      }
      0x09 => ((self.t2.counter >> 8) & 0xff) as u8,
      0x0a => self.sr,
      0x0b => self.acr,
      0x0c => self.pcr,
      0x0d => self.ifr,
      0x0e => self.ier,
      0x0f => self.a.read(),
      _ => unreachable!(),
    }
  }

  fn write(&mut self, address: u16, value: u8) {
    match address % 0x10 {
      0x00 => self.b.write(value),
      0x01 => self.a.write(value), // TODO: controls handshake?
      0x02 => self.b.ddr = value,
      0x03 => self.a.ddr = value,
      0x04 => self.t1.latch = (self.t1.latch & 0xff00) | (value as u16),
      0x05 => {
        self.t1.latch = (self.t1.latch & 0x00ff) | ((value as u16) << 8);
        self.t1.counter = self.t1.latch;
        self.t1.interrupt = false;
      }
      0x06 => self.t1.latch = (self.t1.latch & 0xff00) | (value as u16),
      0x07 => {
        self.t1.latch = (self.t1.latch & 0x00ff) | ((value as u16) << 8);
        self.t1.interrupt = false;
      }
      0x08 => self.t2.latch = (self.t2.latch & 0xff00) | (value as u16),
      0x09 => {
        self.t2.counter = (self.t2.latch & 0x00ff) | ((value as u16) << 8);
        self.t2.interrupt = false;
      }
      0x0a => self.sr = value,
      0x0b => self.acr = value,
      0x0c => self.pcr = value,
      0x0d => self.ifr &= !value,
      0x0e => self.ier = value,
      0x0f => self.a.write(value),
      _ => unreachable!(),
    }
  }

  fn reset(&mut self) {
    self.a.reset();
    self.b.reset();
  }

  fn poll(&mut self, info: &super::SystemInfo) -> super::ActiveInterrupt {
    let a = self.a.poll(info);
    let b = self.b.poll(info);

    if a || b {
      ActiveInterrupt::IRQ
    } else {
      ActiveInterrupt::None
    }
  }
}
