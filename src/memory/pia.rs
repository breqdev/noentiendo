use std::{cell::Cell, rc::Rc};

use crate::{
  memory::{ActiveInterrupt, Memory, Port, SystemInfo},
  platform::PlatformProvider,
};

// MOS 6520

/// The registers associated with a single port in a MOS 6520 PIA.
struct PortRegisters {
  /// The port itself.
  port: Box<dyn Port>,

  /// If the DDR is write, the current written value.
  writes: Cell<u8>,

  /// Data direction register. Each bit controls whether the line is an input (0) or output (1)
  ddr: Cell<u8>,

  // Control register. Each bit has a specific function.
  pub control: Cell<u8>,
}

impl PortRegisters {
  /// Create a new PortRegisters with the given port.
  pub fn new(port: Box<dyn Port>) -> Self {
    Self {
      port,
      writes: Cell::new(0),
      ddr: Cell::new(0),
      control: Cell::new(0),
    }
  }

  /// Read from either the port or the DDR, depending on the DDR_SELECT bit in
  /// the control register.
  /// When reading the port, bor each bit, if the DDR is set to read, this
  /// reads directly from the port. If the DDR is set to write, this reads from
  /// the written value.
  pub fn read(&self, root: &Rc<dyn Memory>, platform: &Box<dyn PlatformProvider>) -> u8 {
    if self.control.get() & control_bits::DDR_SELECT != 0 {
      (self.port.read(root, platform) & !self.ddr.get()) | (self.writes.get() & self.ddr.get())
    } else {
      self.ddr.get()
    }
  }

  /// Write to either the port or the DDR, depending on the DDR_SELECT bit in
  /// the control register.
  /// Respects the DDR, so if a bit in the DDR is set to read, then that bit
  /// will not be written.
  pub fn write(&self, value: u8, root: &Rc<dyn Memory>, platform: &Box<dyn PlatformProvider>) {
    if self.control.get() & control_bits::DDR_SELECT != 0 {
      self.writes.set(value);
      self.port.write(value & self.ddr.get(), root, platform);
    } else {
      self.ddr.set(value);
    }
  }

  /// Poll the underlying port for interrupts.
  pub fn poll(&self, info: &SystemInfo) -> bool {
    self.port.poll(info)
  }

  /// Reset the DDR, control register, and underlying port.
  pub fn reset(&self, root: &Rc<dyn Memory>, platform: &Box<dyn PlatformProvider>) {
    self.ddr.set(0);
    self.control.set(0);
    self.writes.set(0);

    self.port.reset(root, platform);
  }
}

/// The meanings of each bit in the control register.
pub mod control_bits {
  pub const C1_ACTIVE_TRANSITION_FLAG: u8 = 0b10000000; // 1 = 0->1, 0 = 1->0
  pub const C2_ACTIVE_TRANSITION_FLAG: u8 = 0b01000000;
  pub const C2_DIRECTION: u8 = 0b00100000; // 1 = output, 0 = input
  pub const C2_CONTROL: u8 = 0b00011000; // ???
  pub const DDR_SELECT: u8 = 0b00000100; // enable accessing DDR
  pub const C1_CONTROL: u8 = 0b00000011; // interrupt status control
}

/// A MOS 6520 PIA, containing two ports.
pub struct PIA {
  a: PortRegisters,
  b: PortRegisters,
}

impl PIA {
  /// Create a new PIA with the two given port implementations.
  pub fn new(a: Box<dyn Port>, b: Box<dyn Port>) -> Self {
    Self {
      a: PortRegisters::new(a),
      b: PortRegisters::new(b),
    }
  }
}

impl Memory for PIA {
  fn read(&self, address: u16, root: &Rc<dyn Memory>, platform: &Box<dyn PlatformProvider>) -> u8 {
    match address % 0x04 {
      0x00 => self.a.read(root, platform),
      0x01 => self.a.control.get(),
      0x02 => self.b.read(root, platform),
      0x03 => self.b.control.get(),
      _ => unreachable!(),
    }
  }

  fn write(
    &self,
    address: u16,
    value: u8,
    root: &Rc<dyn Memory>,
    platform: &Box<dyn PlatformProvider>,
  ) {
    match address % 0x04 {
      0x00 => self.a.write(value, root, platform),
      0x01 => self.a.control.set(value),
      0x02 => self.b.write(value, root, platform),
      0x03 => self.b.control.set(value),
      _ => unreachable!(),
    }
  }

  fn reset(&self, root: &Rc<dyn Memory>, platform: &Box<dyn PlatformProvider>) {
    self.a.reset(root, platform);
    self.b.reset(root, platform);
  }

  fn poll(
    &self,
    info: &SystemInfo,
    _root: &Rc<dyn Memory>,
    _platform: &Box<dyn PlatformProvider>,
  ) -> ActiveInterrupt {
    let a = self.a.poll(info);
    let b = self.b.poll(info);

    if a || b {
      ActiveInterrupt::IRQ
    } else {
      ActiveInterrupt::None
    }
  }
}
