use crate::memory::SystemInfo;

pub trait Port: Send {
  fn read(&mut self) -> u8;
  fn write(&mut self, value: u8);
  fn poll(&mut self, info: &SystemInfo) -> bool;
  fn reset(&mut self);
}

pub struct NullPort {
  warn: Option<&'static str>,
}

impl NullPort {
  pub fn new() -> Self {
    Self { warn: None }
  }

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
