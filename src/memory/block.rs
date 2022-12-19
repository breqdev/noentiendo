use std::{cell::RefCell, rc::Rc};

use crate::memory::{ActiveInterrupt, Memory, SystemInfo};
use crate::platform::PlatformProvider;
use crate::roms::RomFile;

/// Represents a simple block of contiguous memory, with no additional hardware.
/// This can be used to represent both RAM and ROM.
/// Reading from this memory is side-effect free.
pub struct BlockMemory {
  size: usize,
  data: RefCell<Vec<u8>>,
  persistent: bool,
}

impl BlockMemory {
  /// Create a BlockMemory of the given size which clears its contents when
  /// reset.
  pub fn ram(size: usize) -> Self {
    Self {
      size,
      data: RefCell::new(vec![0; size]),
      persistent: false,
    }
  }

  /// Create a BlockMemory of the given size which does not clear its contents
  /// when reset.
  pub fn rom(size: usize) -> Self {
    Self {
      size,
      data: RefCell::new(vec![0; size]),
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

    for i in 0..file_data.len() {
      data[i] = file_data[i];
    }

    Self {
      size,
      data: RefCell::new(data),
      persistent: true,
    }
  }
}

impl Memory for BlockMemory {
  fn read(
    &self,
    address: u16,
    _root: &Rc<dyn Memory>,
    _platform: &Box<dyn PlatformProvider>,
  ) -> u8 {
    self.data.borrow()[(address as usize) % self.size]
  }

  fn write(
    &self,
    address: u16,
    value: u8,
    _root: &Rc<dyn Memory>,
    _platform: &Box<dyn PlatformProvider>,
  ) {
    self.data.borrow_mut()[(address as usize) % self.size] = value;
  }

  fn reset(&self, _root: &Rc<dyn Memory>, _platform: &Box<dyn PlatformProvider>) {
    if !self.persistent {
      let mut data = self.data.borrow_mut();

      for i in 0..data.len() {
        data[i] = 0;
      }
    }
  }

  fn poll(
    &self,
    _info: &SystemInfo,
    _root: &Rc<dyn Memory>,
    _platform: &Box<dyn PlatformProvider>,
  ) -> ActiveInterrupt {
    ActiveInterrupt::None
  }
}
