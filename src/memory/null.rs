use crate::memory::{ActiveInterrupt, Memory, SystemInfo};

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
  fn read(&mut self, _address: u16) -> u8 {
    if let Some(message) = self.warn {
      println!(
        "attempted to read from {} at address {:04x}",
        message, _address
      );
    }
    0
  }

  fn write(&mut self, _address: u16, _value: u8) {
    if let Some(message) = self.warn {
      println!(
        "attempted to write to {} at address {:04x}",
        message, _address
      );
    }
  }

  fn reset(&mut self) {}

  fn poll(&mut self, _info: &SystemInfo) -> ActiveInterrupt {
    ActiveInterrupt::None
  }
}
