use crate::memory::{ActiveInterrupt, Memory, SystemInfo};

/// Memory that does nothing when read or written to.
#[derive(Default)]
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
  fn read(&mut self, address: u16) -> u8 {
    if let Some(message) = self.warn {
      println!("attempted to read from {message} at address {address:04x}",);
    }
    0
  }

  fn write(&mut self, address: u16, _value: u8) {
    if let Some(message) = self.warn {
      println!("attempted to write to {message} at address {address:04x}",);
    }
  }

  fn reset(&mut self) {}

  fn poll(&mut self, _cycles: u32, _info: &SystemInfo) -> ActiveInterrupt {
    ActiveInterrupt::None
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_null() {
    let mut memory = NullMemory::new();
    assert_eq!(memory.read(0x0000), 0);
    memory.write(0x0000, 0x12);
    assert_eq!(memory.read(0x0000), 0);
  }
}
