use crate::memory::{ActiveInterrupt, Memory, SystemInfo};

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
  pub fn map(mut self, address: usize, memory: Box<dyn Memory>) -> Self {
    self.mapping.push((address, memory));

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

  fn poll(&mut self, info: &SystemInfo) -> ActiveInterrupt {
    let mut highest = ActiveInterrupt::None;

    for (_, mapped) in &mut self.mapping {
      let interrupt = mapped.poll(info);

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
