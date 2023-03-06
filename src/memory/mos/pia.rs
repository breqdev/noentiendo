use crate::memory::{mos::Port, ActiveInterrupt, Memory, SystemInfo};

// MOS 6520
// http://archive.6502.org/datasheets/mos_6520.pdf

/// The registers associated with a single port in a MOS 6520 PIA.
struct PiaPortRegisters {
  /// The port itself.
  port: Box<dyn Port>,

  /// If the DDR is write, the current written value.
  writes: u8,

  /// Data direction register. Each bit controls whether the line is an input (0) or output (1)
  ddr: u8,

  // Control register. Each bit has a specific function.
  pub control: u8,
}

impl PiaPortRegisters {
  /// Create a new PortRegisters with the given port.
  pub fn new(port: Box<dyn Port>) -> Self {
    Self {
      port,
      writes: 0,
      ddr: 0,
      control: 0,
    }
  }

  /// Read from either the port or the DDR, depending on the DDR_SELECT bit in
  /// the control register.
  /// When reading the port, bor each bit, if the DDR is set to read, this
  /// reads directly from the port. If the DDR is set to write, this reads from
  /// the written value.
  pub fn read(&mut self) -> u8 {
    if self.control & pia_control_bits::DDR_SELECT != 0 {
      (self.port.read() & !self.ddr) | (self.writes & self.ddr)
    } else {
      self.ddr
    }
  }

  /// Write to either the port or the DDR, depending on the DDR_SELECT bit in
  /// the control register.
  /// Respects the DDR, so if a bit in the DDR is set to read, then that bit
  /// will not be written.
  pub fn write(&mut self, value: u8) {
    if self.control & pia_control_bits::DDR_SELECT != 0 {
      self.writes = value;
      self.port.write(value & self.ddr);
    } else {
      self.ddr = value;
    }
  }

  /// Poll the underlying port for interrupts.
  pub fn poll(&mut self, cycles: u32, info: &SystemInfo) -> bool {
    let interrupts = self.port.poll(cycles, info);

    // TODO: handle C1 and C2

    false
  }

  /// Reset the DDR, control register, and underlying port.
  pub fn reset(&mut self) {
    self.ddr = 0;
    self.control = 0;

    self.port.reset();
  }
}

#[allow(dead_code)]
/// The meanings of each bit in the control register.
pub mod pia_control_bits {
  pub const C1_ACTIVE_TRANSITION_FLAG: u8 = 0b10000000; // 1 = 0->1, 0 = 1->0
  pub const C2_ACTIVE_TRANSITION_FLAG: u8 = 0b01000000;
  pub const C2_DIRECTION: u8 = 0b00100000; // 1 = output, 0 = input
  pub const C2_CONTROL: u8 = 0b00011000; // ???
  pub const DDR_SELECT: u8 = 0b00000100; // enable accessing DDR
  pub const C1_CONTROL: u8 = 0b00000011; // interrupt status control
}

/// The MOS 6520 Peripheral Interface Adapter (PIA), containing two ports and
/// some control lines.
pub struct Pia {
  a: PiaPortRegisters,
  b: PiaPortRegisters,
}

impl Pia {
  /// Create a new PIA with the two given port implementations.
  pub fn new(a: Box<dyn Port>, b: Box<dyn Port>) -> Self {
    Self {
      a: PiaPortRegisters::new(a),
      b: PiaPortRegisters::new(b),
    }
  }
}

impl Memory for Pia {
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

  fn poll(&mut self, cycles: u32, info: &SystemInfo) -> ActiveInterrupt {
    let a = self.a.poll(cycles, info);
    let b = self.b.poll(cycles, info);

    if a || b {
      ActiveInterrupt::IRQ
    } else {
      ActiveInterrupt::None
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::memory::mos::NullPort;

  use super::*;

  #[test]
  fn test_read() {
    let mut pia = Pia::new(Box::new(NullPort::new()), Box::new(NullPort::new()));

    // deselect the DDR
    pia.write(0x01, pia_control_bits::DDR_SELECT);

    assert_eq!(0, pia.read(0x00));
    assert_eq!(pia_control_bits::DDR_SELECT, pia.read(0x01));
    assert_eq!(0, pia.read(0x02));
    assert_eq!(0, pia.read(0x03));

    // wraps around
    assert_eq!(0, pia.read(0x04));

    // select the DDR
    pia.write(0x01, 0);

    assert_eq!(0, pia.read(0x00));
    assert_eq!(0, pia.read(0x01));
    assert_eq!(0, pia.read(0x02));
    assert_eq!(0, pia.read(0x03));
  }

  #[test]
  fn test_write() {
    let mut pia = Pia::new(Box::new(NullPort::new()), Box::new(NullPort::new()));

    // deselect the DDR
    pia.write(0x01, pia_control_bits::DDR_SELECT);

    // writes without DDR shouldn't be reflected in reads
    pia.write(0x00, 0b10101010);
    assert_eq!(0, pia.read(0x00));

    // write to the DDR
    pia.write(0x01, 0);
    pia.write(0x00, 0b11110000);

    // now, our past writes should be reflected in reads
    // (masked by the DDR)
    pia.write(0x01, pia_control_bits::DDR_SELECT);
    assert_eq!(0b10100000, pia.read(0x00));
    assert_eq!(pia_control_bits::DDR_SELECT, pia.read(0x01));

    // and future writes should be reflected in reads
    pia.write(0x00, 0b01010101);
    assert_eq!(0b01010000, pia.read(0x00));
  }
}
