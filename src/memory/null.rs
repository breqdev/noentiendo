use std::rc::Rc;

use crate::{
  memory::{ActiveInterrupt, Memory, SystemInfo},
  platform::PlatformProvider,
};

/// Memory that does nothing when read or written to.
pub struct NullMemory {
  warn: Option<&'static str>,
}

impl NullMemory {
  /// Create a new NullMemory that will not warn when read or written to.
  pub fn new() -> Self {
    Self { warn: None }
  }

  /// Create a new NullMemory that will warn when read or written to.
  pub fn with_warnings(message: &'static str) -> Self {
    Self {
      warn: Some(message),
    }
  }
}

impl Memory for NullMemory {
  fn read(
    &self,
    address: u16,
    _root: &Rc<dyn Memory>,
    _platform: &Box<dyn PlatformProvider>,
  ) -> u8 {
    if let Some(message) = self.warn {
      println!(
        "attempted to read from {} at address {:04x}",
        message, address
      );
    }
    0
  }

  fn write(
    &self,
    address: u16,
    value: u8,
    _root: &Rc<dyn Memory>,
    _platform: &Box<dyn PlatformProvider>,
  ) {
    if let Some(message) = self.warn {
      println!(
        "attempted to write {:02x} to {} at address {:04x}",
        value, message, address
      );
    }
  }

  fn reset(&self, _root: &Rc<dyn Memory>, _platform: &Box<dyn PlatformProvider>) {}

  fn poll(
    &self,
    _info: &SystemInfo,
    _root: &Rc<dyn Memory>,
    _platform: &Box<dyn PlatformProvider>,
  ) -> ActiveInterrupt {
    ActiveInterrupt::None
  }
}
