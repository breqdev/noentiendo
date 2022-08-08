use crate::memory::{ActiveInterrupt, Memory};
use std::fs::File;
use std::io::Read;

pub struct BlockMemory {
  size: usize,
  data: Vec<u8>,
  persistent: bool,
}

impl BlockMemory {
  pub fn ram(size: usize) -> Self {
    Self {
      size,
      data: vec![0; size],
      persistent: false,
    }
  }

  pub fn rom(size: usize) -> Self {
    Self {
      size,
      data: vec![0; size],
      persistent: true,
    }
  }

  pub fn from_file(size: usize, path: &str) -> Self {
    let mut file = File::open(path).unwrap();
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();

    Self {
      size,
      data,
      persistent: true,
    }
  }
}

impl Memory for BlockMemory {
  fn read(&self, address: u16) -> u8 {
    self.data[(address as usize) % self.size]
  }

  fn write(&mut self, address: u16, value: u8) {
    self.data[(address as usize) % self.size] = value;
  }

  fn reset(&mut self) {
    if !self.persistent {
      for i in 0..self.data.len() {
        self.data[i] = 0;
      }
    }
  }

  fn poll(&mut self) -> ActiveInterrupt {
    ActiveInterrupt::None
  }
}
