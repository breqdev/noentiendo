pub trait Memory {
  fn read(&self, address: u16) -> Result<u8, ()>;
  fn write(&mut self, address: u16, value: u8) -> Result<(), ()>;
  fn reset(&mut self);
}

pub struct BlockMemory {
  data: Vec<u8>,
}

impl BlockMemory {
  pub fn new(size: usize) -> Self {
    Self {
      data: vec![0; size],
    }
  }
}

impl Memory for BlockMemory {
  fn read(&self, address: u16) -> Result<u8, ()> {
    if (address as usize) < self.data.len() {
      Ok(self.data[address as usize])
    } else {
      println!("invalid read! {:x}", address);
      Err(())
    }
  }

  fn write(&mut self, address: u16, value: u8) -> Result<(), ()> {
    if (address as usize) < self.data.len() {
      self.data[address as usize] = value;
      Ok(())
    } else {
      println!("invalid write! {:x}", address);
      Err(())
    }
  }

  fn reset(&mut self) {
    for i in 0..self.data.len() {
      self.data[i] = 0;
    }
  }
}

pub struct MappedIO {}

impl Memory for MappedIO {
  fn read(&self, address: u16) -> Result<u8, ()> {
    // TODO: keyboard input?
    Ok(0)
  }

  fn write(&mut self, address: u16, value: u8) -> Result<(), ()> {
    println!("{}", value);
    Ok(())
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
  split: u16,
}

impl Memory for BranchMemory {
  fn read(&self, address: u16) -> Result<u8, ()> {
    if address < self.split {
      self.low.read(address)
    } else {
      self.high.read(address - self.split)
    }
  }

  fn write(&mut self, address: u16, value: u8) -> Result<(), ()> {
    if address < self.split {
      self.low.write(address, value)
    } else {
      self.high.write(address - self.split, value)
    }
  }

  fn reset(&mut self) {
    self.low.reset();
    self.high.reset();
  }
}

impl BranchMemory {
  pub fn new(low: Box<dyn Memory>, high: Box<dyn Memory>, split: u16) -> Self {
    Self { low, high, split }
  }
}
