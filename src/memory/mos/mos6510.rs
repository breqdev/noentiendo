use crate::memory::{mos::Port, ActiveInterrupt, Memory, SystemInfo};

/// Represents the port built into a MOS 6510 processor, mapped to memory addresses 0x0000 (for the DDR) and 0x0001 (for the port itself).
pub struct Mos6510Port {
  /// The port itself.
  port: Box<dyn Port>,

  /// If the DDR is write, the current written value.
  writes: u8,

  /// Data direction register. Each bit controls whether the line is an input (0) or output (1)
  ddr: u8,
}

impl Mos6510Port {
  /// Create a new MOS 6510 port with the given port.
  pub fn new(port: Box<dyn Port>) -> Self {
    Self {
      port,
      writes: 0,
      ddr: 0xFF,
    }
  }
}

impl Memory for Mos6510Port {
  fn read(&mut self, address: u16) -> u8 {
    match address % 2 {
      0 => self.ddr,
      1 => (self.port.read() & !self.ddr) | (self.writes & self.ddr),
      _ => unreachable!(),
    }
  }

  fn write(&mut self, address: u16, value: u8) {
    match address % 2 {
      0 => {
        self.ddr = value;
        self.port.write(self.writes & self.ddr);
      }
      1 => {
        self.writes = value;
        self.port.write(value & self.ddr);
      }
      _ => unreachable!(),
    }
  }

  fn reset(&mut self) {
    self.port.reset();
  }

  fn poll(&mut self, cycles: u32, info: &SystemInfo) -> ActiveInterrupt {
    match self.port.poll(cycles, info) {
      true => ActiveInterrupt::IRQ,
      false => ActiveInterrupt::None,
    }
  }
}
