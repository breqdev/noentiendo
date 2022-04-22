const SIZE: usize = 0x10000;

pub trait Memory {
  fn read(&self, address: u16) -> Result<u8, ()>;
  fn write(&mut self, address: u16, value: u8) -> Result<(), ()>;
  fn reset(&mut self);
}

pub struct BlockMemory {
  data: [u8; SIZE],
}

impl BlockMemory {
  pub fn new() -> Self {
    Self { data: [0; SIZE] }
  }
}

impl Memory for BlockMemory {
  fn read(&self, address: u16) -> Result<u8, ()> {
    if (address as usize) < SIZE {
      Ok(self.data[address as usize])
    } else {
      Err(())
    }
  }

  fn write(&mut self, address: u16, value: u8) -> Result<(), ()> {
    if (address as usize) < SIZE {
      self.data[address as usize] = value;
      Ok(())
    } else {
      Err(())
    }
  }

  fn reset(&mut self) {
    for i in 0..SIZE {
      self.data[i] = 0;
    }
  }
}
