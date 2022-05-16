use std::fs::File;
use std::io::Read;

pub trait Memory {
  fn read(&self, address: u16) -> u8;
  fn write(&mut self, address: u16, value: u8);
  fn reset(&mut self);
}

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
    self.data[address as usize & ((1 << self.bits) - 1)]
  }

  fn write(&mut self, address: u16, value: u8) {
    self.data[address as usize & ((1 << self.bits) - 1)] = value;
  }

  fn reset(&mut self) {
    for i in 0..self.data.len() {
      self.data[i] = 0;
    }
  }
}

pub struct MappedIO {}

impl Memory for MappedIO {
  fn read(&self, _address: u16) -> u8 {
    // TODO: keyboard input?
    0
  }

  fn write(&mut self, _address: u16, value: u8) {
    println!("{}", value);
  }

  fn reset(&mut self) {}
}

impl MappedIO {
  pub fn new() -> Self {
    Self {}
  }
}

pub struct BranchMemory {
  low: Box<dyn Memory>,
  high: Box<dyn Memory>,
  bits: u8,
}

impl Memory for BranchMemory {
  fn read(&self, address: u16) -> u8 {
    if address < (1 << (self.bits - 1)) {
      self.low.read(address)
    } else {
      self.high.read(address - (1 << (self.bits - 1)))
    }
  }

  fn write(&mut self, address: u16, value: u8) {
    if address < (1 << (self.bits - 1)) {
      self.low.write(address, value)
    } else {
      self.high.write(address - (1 << (self.bits - 1)), value)
    }
  }

  fn reset(&mut self) {
    self.low.reset();
    self.high.reset();
  }
}

impl BranchMemory {
  pub fn new(low: Box<dyn Memory>, high: Box<dyn Memory>, bits: u8) -> Self {
    Self { low, high, bits }
  }
}
