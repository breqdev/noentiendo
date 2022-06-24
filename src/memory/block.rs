use crate::memory::Memory;
use std::fs::File;
use std::io::Read;

pub struct BlockMemory {
  bits: u8,
  data: Vec<u8>,
}

impl BlockMemory {
  pub fn new(bits: u8) -> Self {
    Self {
      bits,
      data: vec![0; (1 << bits) as usize],
    }
  }

  pub fn from_file(bits: u8, path: &str) -> Self {
    let mut file = File::open(path).unwrap();
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();

    Self { bits, data }
  }
}

impl Memory for BlockMemory {
  fn read(&self, address: u16) -> u8 {
    self.data[(address & ((1 << self.bits) - 1)) as usize]
  }

  fn write(&mut self, address: u16, value: u8) {
    self.data[(address & ((1 << self.bits) - 1)) as usize] = value;
  }

  fn tick(&mut self) {}

  fn reset(&mut self) {
    for i in 0..self.data.len() {
      self.data[i] = 0;
    }
  }
}
