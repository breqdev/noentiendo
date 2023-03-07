use crate::memory::{ActiveInterrupt, Memory, SystemInfo};

use super::{ControlLines, ControlLinesPort};

// MOS 6520
// http://archive.6502.org/datasheets/mos_6520.pdf

/// The registers associated with a single port in a MOS 6520 PIA.
struct PiaPortRegisters {
  /// The port itself.
  port: Box<dyn ControlLinesPort>,

  /// If the DDR is write, the current written value.
  writes: u8,

  /// Data direction register. Each bit controls whether the line is an input (0) or output (1)
  ddr: u8,

  // Control register. Each bit has a specific function.
  pub control: u8,

  /// Previous state of the control lines. Used to detect rising or falling edges.
  control_lines: ControlLines,
}

impl PiaPortRegisters {
  /// Create a new PortRegisters with the given port.
  pub fn new(port: Box<dyn ControlLinesPort>) -> Self {
    Self {
      port,
      writes: 0,
      ddr: 0,
      control: 0,
      control_lines: ControlLines::new(),
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
      self.control &= !pia_control_bits::C1_ACTIVE_TRANSITION_FLAG;
      self.control &= !pia_control_bits::C2_ACTIVE_TRANSITION_FLAG;
      self.ddr
    }
  }

  /// Write to either the port or the DDR, depending on the DDR_SELECT bit in
  /// the control register.
  /// Respects the DDR, so if a bit in the DDR is set to read, then that bit
  /// will not be written.
  pub fn write(&mut self, value: u8) {
    // TODO: support writes to C2
    if self.control & pia_control_bits::DDR_SELECT != 0 {
      self.writes = value;
      self.port.write(value & self.ddr);
    } else {
      self.ddr = value;
    }
  }

  /// Poll the underlying port for interrupts.
  pub fn poll(&mut self, cycles: u32, info: &SystemInfo) -> bool {
    let control_lines = self.port.poll(cycles, info);
    let mut interrupt = false;

    if control_lines.c1 != self.control_lines.c1 {
      if control_lines.c1 {
        // Rising edge (positive transition)
        if self.control & pia_control_bits::C1_ACTIVE_TRANSITION != 0 {
          self.control |= pia_control_bits::C1_ACTIVE_TRANSITION_FLAG;

          if self.control & pia_control_bits::C1_ENABLE_INTERRUPT != 0 {
            interrupt = true;
          }
        }
      } else {
        // Falling edge (negative transition)
        if self.control & pia_control_bits::C1_ACTIVE_TRANSITION == 0 {
          self.control |= pia_control_bits::C1_ACTIVE_TRANSITION_FLAG;

          if self.control & pia_control_bits::C1_ENABLE_INTERRUPT != 0 {
            interrupt = true;
          }
        }
      }
    }

    if self.control & pia_control_bits::C2_DIRECTION == 0 {
      // C2 is an input
      if control_lines.c2 != self.control_lines.c2 {
        if control_lines.c2 {
          // Rising edge (positive transition)
          if self.control & pia_control_bits::C2_ACTIVE_TRANSITION != 0 {
            self.control |= pia_control_bits::C2_ACTIVE_TRANSITION_FLAG;

            if self.control & pia_control_bits::C2_ENABLE_INTERRUPT != 0 {
              interrupt = true;
            }
          }
        } else {
          // Falling edge (negative transition)
          if self.control & pia_control_bits::C2_ACTIVE_TRANSITION == 0 {
            self.control |= pia_control_bits::C2_ACTIVE_TRANSITION_FLAG;

            if self.control & pia_control_bits::C2_ENABLE_INTERRUPT != 0 {
              interrupt = true;
            }
          }
        }
      }
    }

    self.control_lines = control_lines;

    interrupt
  }

  /// Write the C2 line.
  pub fn write_control(&mut self, value: u8) {
    self.control = value;

    if value & pia_control_bits::C2_DIRECTION != 0 {
      // CA2 is an output
      if value & pia_control_bits::C2_MANUAL_OUTPUT == 0 {
        todo!("Handshake and pulse modes not supported");
      } else {
        let output = value & pia_control_bits::C2_MANUAL_OUTPUT_VALUE != 0;
        println!("Writing C2: {}", output);
        self.port.write_c2(output);
      }
    }
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
  /// Flags that are set when a signal edge is detected.
  pub const C1_ACTIVE_TRANSITION_FLAG: u8 = 0b10000000;
  pub const C2_ACTIVE_TRANSITION_FLAG: u8 = 0b01000000;

  /// Set the direction of the C2 line.
  pub const C2_DIRECTION: u8 = 0b00100000; // 1 = output, 0 = input

  /// The control bits of the C2 line. Note that multiple meanings exist
  /// depending on the mode.
  pub const C2_CONTROL: u8 = 0b00011000;

  pub const C2_ACTIVE_TRANSITION: u8 = 0b00010000; // 0 = falling, 1 = rising
  pub const C2_ENABLE_INTERRUPT: u8 = 0b00001000; // 0 = disable IRQ, 1 = enable

  pub const C2_MANUAL_OUTPUT: u8 = 0b00010000; // 0 = handshake/pulse, 1 = direct output

  pub const C2_PULSE_OUTPUT: u8 = 0b00001000; // 0 = handshake on read, 1 = pulse on read
  pub const C2_MANUAL_OUTPUT_VALUE: u8 = 0b00001000; // 0 = low, 1 = high

  pub const DDR_SELECT: u8 = 0b00000100; // enable accessing DDR
  pub const C1_ACTIVE_TRANSITION: u8 = 0b00000010; // 0 = falling, 1 = rising
  pub const C1_ENABLE_INTERRUPT: u8 = 0b00000001; // 0 = disable IRQ, 1 = enable
}

/// The MOS 6520 Peripheral Interface Adapter (PIA), containing two ports and
/// some control lines.
pub struct Pia {
  a: PiaPortRegisters,
  b: PiaPortRegisters,
}

impl Pia {
  /// Create a new PIA with the two given port implementations.
  pub fn new(a: Box<dyn ControlLinesPort>, b: Box<dyn ControlLinesPort>) -> Self {
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
      0x01 => self.a.write_control(value),
      0x02 => self.b.write(value),
      0x03 => self.b.write_control(value),
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
