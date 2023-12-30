use crate::memory::{ActiveInterrupt, Memory};

/// Maps several Memory objects into a single contiguous address space.
/// Each mapped object is assigned a starting address, and reads and writes
/// will have the starting address subtracted from them before being passed
/// to the underlying Memory object.
#[derive(Default)]
pub struct BranchMemory {
  mapping: Vec<(usize, Box<dyn Memory>)>,
}

impl BranchMemory {
  /// Create a new BranchMemory with no mappings.
  pub fn new() -> Self {
    Self {
      mapping: Vec::new(),
    }
  }

  /// Map a new Memory object to the given starting address in this mapping.
  /// Returns this BranchMemory for chaining.
  pub fn map(mut self, address: usize, memory: impl Memory + 'static) -> Self {
    self.mapping.push((address, Box::new(memory)));

    self
  }
}

impl Memory for BranchMemory {
  fn read(&mut self, address: u16) -> u8 {
    let mut memory = None;
    let mut offset = 0;

    for (start, mapped) in &mut self.mapping {
      if address as usize >= *start {
        memory = Some(mapped);
        offset = *start as u16;
      }
    }

    match memory {
      Some(memory) => memory.read(address - offset),
      None => 0,
    }
  }

  fn write(&mut self, address: u16, value: u8) {
    let mut memory = None;
    let mut offset = 0;

    for (start, mapped) in &mut self.mapping {
      if address as usize >= *start {
        memory = Some(mapped);
        offset = *start as u16;
      }
    }

    if let Some(memory) = memory {
      memory.write(address - offset, value);
    }
  }

  fn reset(&mut self) {
    for (_, mapped) in &mut self.mapping {
      mapped.reset();
    }
  }

  fn poll(&mut self, cycles_since_poll: u64, total_cycle_count: u64) -> ActiveInterrupt {
    let mut highest = ActiveInterrupt::None;

    for (_, mapped) in &mut self.mapping {
      let interrupt = mapped.poll(cycles_since_poll, total_cycle_count);

      match interrupt {
        ActiveInterrupt::None => (),
        ActiveInterrupt::NMI => {
          highest = ActiveInterrupt::NMI;
        }
        ActiveInterrupt::IRQ => {
          highest = ActiveInterrupt::IRQ;
        }
      }
    }

    highest
  }
}

#[cfg(test)]
mod tests {
  use crate::memory::BlockMemory;

  use super::*;

  #[test]
  fn test_empty() {
    let mut memory = BranchMemory::new();

    assert_eq!(0, memory.read(0x0000));
    assert_eq!(0, memory.read(0x1234));
    assert_eq!(0, memory.read(0xFFFF));
  }

  #[test]
  fn test_single_block() {
    let mut block = BlockMemory::ram(0x1000);
    block.write(0x00, 0x12);
    block.write(0x34, 0x56);

    let mut memory = BranchMemory::new().map(0, block);

    assert_eq!(0x12, memory.read(0));
    assert_eq!(0x56, memory.read(0x34));

    memory.write(0x00, 0x34);
    memory.write(0x34, 0x78);

    assert_eq!(0x34, memory.read(0));
    assert_eq!(0x78, memory.read(0x34));
  }

  #[test]
  fn test_single_with_offset() {
    let mut block = BlockMemory::ram(0x1000);
    block.write(0x00, 0x12);
    block.write(0x34, 0x56);

    let mut memory = BranchMemory::new().map(0x100, block);

    assert_eq!(0, memory.read(0));
    assert_eq!(0, memory.read(0x34));

    assert_eq!(0x12, memory.read(0x100));
    assert_eq!(0x56, memory.read(0x134));

    // writing to nowhere should do nothing
    memory.write(0x00, 0x34);
    memory.write(0x34, 0x78);
    assert_eq!(0, memory.read(0));
    assert_eq!(0, memory.read(0x34));

    // writing to the block should work
    memory.write(0x100, 0x34);
    memory.write(0x134, 0x78);
    assert_eq!(0x34, memory.read(0x100));
    assert_eq!(0x78, memory.read(0x134));
  }

  #[test]
  fn test_multiple_blocks() {
    let mut block1 = BlockMemory::rom(0x1000);
    let mut block2 = BlockMemory::ram(0x1000);

    block1.write(0x00, 0x12);
    block1.write(0x34, 0x56);

    block2.write(0x00, 0x78);
    block2.write(0x34, 0x9A);

    let mut memory = BranchMemory::new().map(0x0000, block1).map(0x1000, block2);

    // test reads
    assert_eq!(0x00, memory.read(0x0000));
    assert_eq!(0x00, memory.read(0x0034));
    assert_eq!(0x78, memory.read(0x1000));
    assert_eq!(0x9A, memory.read(0x1034));

    // test writes
    memory.write(0x0000, 0x34);
    memory.write(0x1034, 0x78);

    assert_eq!(0x00, memory.read(0x0000));
    assert_eq!(0x00, memory.read(0x0034));
    assert_eq!(0x78, memory.read(0x1000));
    assert_eq!(0x78, memory.read(0x1034));

    // test reset
    memory.reset();

    assert_eq!(0x00, memory.read(0x0000));
    assert_eq!(0x00, memory.read(0x0034));
    assert_eq!(0x00, memory.read(0x1000));
    assert_eq!(0x00, memory.read(0x1034));
  }

  #[test]
  fn test_overlapping_blocks() {
    let mut block1 = BlockMemory::ram(0x1000);
    let mut block2 = BlockMemory::ram(0x1000);

    block1.write(0x000, 0x12);
    block1.write(0x234, 0x56);

    block2.write(0x000, 0x78);
    block2.write(0x134, 0x9A);

    let mut memory = BranchMemory::new().map(0x0000, block1).map(0x0100, block2);

    // test reads
    assert_eq!(0x12, memory.read(0x0000));
    assert_eq!(0x78, memory.read(0x0100));
    assert_eq!(0x9A, memory.read(0x0234));

    // test writes
    memory.write(0x0234, 0xFF);

    assert_eq!(0x12, memory.read(0x0000));
    assert_eq!(0x78, memory.read(0x0100));
    assert_eq!(0xFF, memory.read(0x0234));
  }
}
