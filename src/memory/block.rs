use crate::memory::{ActiveInterrupt, Memory, SystemInfo};
use crate::roms::RomFile;

/// Represents a simple block of contiguous memory, with no additional hardware.
/// This can be used to represent both RAM and ROM.
/// Reading from this memory is side-effect free.
pub struct BlockMemory {
  size: usize,
  data: Vec<u8>,
  persistent: bool,
  writeable: bool,
}

impl BlockMemory {
  /// Create a BlockMemory of the given size which clears its contents when
  /// reset.
  pub fn ram(size: usize) -> Self {
    Self {
      size,
      data: vec![0; size],
      persistent: false,
      writeable: true,
    }
  }

  /// Create a BlockMemory of the given size which does not clear its contents
  /// when reset.
  pub fn rom(size: usize) -> Self {
    Self {
      size,
      data: vec![0; size],
      persistent: true,
      writeable: false,
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
      writeable: false,
    }
  }

  /// Change whether this BlockMemory is writable.
  pub fn set_writeable(mut self, writeable: bool) -> Self {
    self.writeable = writeable;

    self
  }
}

impl Memory for BlockMemory {
  fn read(&mut self, address: u16) -> u8 {
    self.data[(address as usize) % self.size]
  }

  fn write(&mut self, address: u16, value: u8) {
    if self.writeable {
      self.data[(address as usize) % self.size] = value;
    }
  }

  fn reset(&mut self) {
    if !self.persistent {
      for i in 0..self.data.len() {
        self.data[i] = 0;
      }
    }
  }

  fn poll(&mut self, _cycles: u32, _info: &SystemInfo) -> ActiveInterrupt {
    ActiveInterrupt::None
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_ram() {
    let mut mem = BlockMemory::ram(0x1000);
    assert_eq!(0x00, mem.read(0x123));

    mem.write(0x123, 0x45);
    assert_eq!(0x45, mem.read(0x123));
    assert_eq!(0x00, mem.read(0x124));

    // test wraparound
    assert_eq!(0x45, mem.read(0x1123));

    mem.reset();
    assert_eq!(0x00, mem.read(0x123));
  }

  #[test]
  fn test_rom() {
    let mut mem = BlockMemory::rom(0x1000);
    assert_eq!(0x00, mem.read(0x123));

    // persistent memory should not be writeable
    mem.write(0x123, 0x45);
    assert_eq!(0x00, mem.read(0x123));

    mem.reset();
  }

  #[test]
  fn test_from_file() {
    let file = RomFile::new(vec![0x12, 0x34, 0x56, 0x78]);
    let mut mem = BlockMemory::from_file(0x1000, file);

    assert_eq!(0x12, mem.read(0x000));
    assert_eq!(0x34, mem.read(0x001));
    assert_eq!(0x56, mem.read(0x002));
    assert_eq!(0x78, mem.read(0x003));

    // test wraparound
    assert_eq!(0x12, mem.read(0x1000));

    mem.reset();
    // persistent memory should not be reset
    assert_eq!(0x12, mem.read(0x000));

    // test that the rest of the memory is zeroed
    assert_eq!(0x00, mem.read(0x004));
    assert_eq!(0x00, mem.read(0xFFF));
  }

  #[test]
  #[should_panic]
  fn test_from_file_too_large() {
    let file = RomFile::new(vec![0x12, 0x34, 0x56, 0x78]);
    BlockMemory::from_file(0x03, file);
  }
}
