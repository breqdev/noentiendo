use crate::memory::{ActiveInterrupt, Memory, RomFile, SystemInfo};

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

  pub fn from_file(size: usize, file: RomFile) -> Self {
    let mut data = vec![0; size];
    let file_data = file.get_data();

    if file_data.len() > size {
      panic!(
        "File of size {} is too large for memory block of size {}",
        file_data.len(),
        size
      );
    }

    for i in 0..file_data.len() {
      data[i] = file_data[i];
    }

    Self {
      size,
      data,
      persistent: true,
    }
  }
}

impl Memory for BlockMemory {
  fn read(&mut self, address: u16) -> u8 {
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

  fn poll(&mut self, _info: &SystemInfo) -> ActiveInterrupt {
    ActiveInterrupt::None
  }
}
