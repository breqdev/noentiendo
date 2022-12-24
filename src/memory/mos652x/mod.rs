pub mod cia;
pub mod pia;
pub mod via;

pub use cia::Cia;
pub use pia::Pia;
pub use via::Via;

use crate::memory::{Port, SystemInfo};

/// A port and its associated registers on the MOS 6522 VIA or MOS 6526 CIA.
struct PortRegisters {
  /// The Port implementation that this instance delegates to.
  port: Box<dyn Port>,

  /// Stores the current value written to the port.
  writes: u8,

  /// Data Direction Register. Each bit controls whether the line is an input (0) or output (1).
  ddr: u8,

  /// Latch enable: Present on the MOS 6522 VIA.
  latch_enabled: bool,
}

impl PortRegisters {
  pub fn new(port: Box<dyn Port>) -> Self {
    Self {
      port,
      writes: 0,
      ddr: 0,
      latch_enabled: false,
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
