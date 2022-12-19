use std::rc::Rc;

use crate::{
  memory::{ActiveInterrupt, Memory, SystemInfo},
  platform::PlatformProvider,
};

/// Maps several Memory objects into a single contiguous address space.
/// Each mapped object is assigned a starting address, and reads and writes
/// will have the starting address subtracted from them before being passed
/// to the underlying Memory object.
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
  fn read(&self, address: u16, root: &Rc<dyn Memory>, platform: &Box<dyn PlatformProvider>) -> u8 {
    let mut memory = None;
    let mut offset = 0;

    for (start, mapped) in &self.mapping {
      if address as usize >= *start {
        memory = Some(mapped);
        offset = *start as u16;
      }
    }

    match memory {
      Some(memory) => memory.read(address - offset, root, platform),
      None => 0,
    }
  }

  fn write(
    &self,
    address: u16,
    value: u8,
    root: &Rc<dyn Memory>,
    platform: &Box<dyn PlatformProvider>,
  ) {
    let mut memory = None;
    let mut offset = 0;

    for (start, mapped) in &self.mapping {
      if address as usize >= *start {
        memory = Some(mapped);
        offset = *start as u16;
      }
    }

    match memory {
      Some(memory) => memory.write(address - offset, value, root, platform),
      None => (),
    };
  }

  fn reset(&self, root: &Rc<dyn Memory>, platform: &Box<dyn PlatformProvider>) {
    for (_, mapped) in &self.mapping {
      mapped.reset(root, platform);
    }
  }

  fn poll(
    &self,
    info: &SystemInfo,
    root: &Rc<dyn Memory>,
    platform: &Box<dyn PlatformProvider>,
  ) -> ActiveInterrupt {
    let mut highest = ActiveInterrupt::None;

    for (_, mapped) in &self.mapping {
      let interrupt = mapped.poll(info, root, platform);

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
