use crate::memory::Memory;
use std::fs::File;
use std::io::Read;

pub struct BlockMemory {
  size: usize,
  data: Vec<u8>,
}

impl BlockMemory {
  pub fn new(size: usize) -> Self {
    Self {
      size,
      data: vec![0; size],
    }
  }

  pub fn from_file(size: usize, path: &str) -> Self {
    let mut file = File::open(path).unwrap();
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();

    Self { size, data }
  }
}

impl Memory for BlockMemory {
  fn read(&self, address: u16) -> u8 {
    self.data[(address as usize) % self.size]
  }

  fn write(&mut self, address: u16, value: u8) {
    self.data[(address as usize) % self.size] = value;
  }

  fn tick(&mut self) {}

  fn reset(&mut self) {
    for i in 0..self.data.len() {
      self.data[i] = 0;
    }
  }
}
