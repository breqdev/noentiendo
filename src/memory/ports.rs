use crate::memory::SystemInfo;

/// A Port that can be read from, written to, reset, or polled for interrupts.
/// Used in the MOS 6520 PIA and the 6522 VIA.
pub trait Port {
  /// Read a byte from the port. This is implementation-defined, and may have
  /// side effects.
  fn read(&mut self) -> u8;

  /// Write a byte to the port. This is implementation-defined.
  fn write(&mut self, value: u8);

  /// Poll the port for interrupts. A port may trigger an interrupt for any
  /// implementation-defined reason.
  fn poll(&mut self, info: &SystemInfo) -> bool;

  /// Reset the port to its initial state, analogous to a system reboot.
  fn reset(&mut self);
}

/// A Port that does nothing.
#[derive(Default)]
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_null() {
    let mut port = NullPort::new();
    assert_eq!(port.read(), 0);
    port.write(0x12);
    assert_eq!(port.read(), 0);
  }
}
