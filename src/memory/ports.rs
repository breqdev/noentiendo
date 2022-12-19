use std::rc::Rc;

use crate::{
  memory::{Memory, SystemInfo},
  platform::PlatformProvider,
};

/// A Port that can be read from, written to, reset, or polled for interrupts.
/// Used in the MOS 6520 PIA and the 6522 VIA.
pub trait Port {
  /// Read a byte from the port. This is implementation-defined, and may have
  /// side effects.
  fn read(&self, root: &Rc<dyn Memory>, platform: &Box<dyn PlatformProvider>) -> u8;

  /// Write a byte to the port. This is implementation-defined.
  fn write(&self, value: u8, root: &Rc<dyn Memory>, platform: &Box<dyn PlatformProvider>);

  /// Poll the port for interrupts. A port may trigger an interrupt for any
  /// implementation-defined reason.
  fn poll(&self, info: &SystemInfo) -> bool;

  /// Reset the port to its initial state, analogous to a system reboot.
  fn reset(&self, root: &Rc<dyn Memory>, platform: &Box<dyn PlatformProvider>);
}

/// A Port that does nothing.
pub struct NullPort {
  warn: Option<&'static str>,
}

impl NullPort {
  /// Create a new NullPort that will not warn when read or written to.
  pub fn new() -> Self {
    Self { warn: None }
  }

  /// Create a new NullPort that will warn when read or written to.
  pub fn with_warnings(message: &'static str) -> Self {
    Self {
      warn: Some(message),
    }
  }
}

impl Port for NullPort {
  fn read(&self, _root: &Rc<dyn Memory>, _platform: &Box<dyn PlatformProvider>) -> u8 {
    if let Some(message) = self.warn {
      println!("attempted to read from {} at address {:04x}", message, 0);
    }
    0
  }

  fn write(&self, value: u8, _root: &Rc<dyn Memory>, _platform: &Box<dyn PlatformProvider>) {
    if let Some(message) = self.warn {
      println!("attempted to write {:02x} to port {}", value, message);
    }
  }

  fn poll(&self, _info: &SystemInfo) -> bool {
    false
  }

  fn reset(&self, _root: &Rc<dyn Memory>, _platform: &Box<dyn PlatformProvider>) {}
}
