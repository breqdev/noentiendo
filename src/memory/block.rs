use crate::memory::{ActiveInterrupt, Memory, SystemInfo};
use crate::roms::RomFile;

/// Represents a simple block of contiguous memory, with no additional hardware.
/// This can be used to represent both RAM and ROM.
/// Reading from this memory is side-effect free.
pub struct BlockMemory {
  size: usize,
  data: Vec<u8>,
  persistent: bool,
}

impl BlockMemory {
  /// Create a BlockMemory of the given size which clears its contents when
  /// reset.
  pub fn ram(size: usize) -> Self {
    Self {
      size,
      data: vec![0; size],
      persistent: false,
    }
  }

  /// Create a BlockMemory of the given size which does not clear its contents
  /// when reset.
  pub fn rom(size: usize) -> Self {
    Self {
      size,
      data: vec![0; size],
      persistent: true,
    }
  }

  /// Create a BlockMemory of the given size which loads its contents from the
  /// given file when reset.
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

    data[..file_data.len()].copy_from_slice(&file_data[..]);

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
