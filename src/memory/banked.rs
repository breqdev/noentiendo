use std::{cell::Cell, rc::Rc};

use super::{ActiveInterrupt, Memory, SystemInfo};

/// Represents the memory banking features found in the Commodore 64 and other
/// devices. Multiple memory implementations are all mapped to the same
/// address space. The active implementation is selected by external logic.
pub struct BankedMemory {
  banks: Vec<Box<dyn Memory>>,
  active: Rc<Cell<(usize, usize)>>,
}

impl BankedMemory {
  /// Create a new, empty banked memory.
  pub fn new(active: Rc<Cell<(usize, usize)>>) -> Self {
    Self {
      banks: Vec::new(),
      active,
    }
  }

  /// Add a new memory implementation to the banked memory.
  pub fn bank(mut self, memory: impl Memory + 'static) -> Self {
    self.banks.push(Box::new(memory));

    self
  }
}

impl Memory for BankedMemory {
  fn read(&mut self, address: u16) -> u8 {
    match self.banks.get_mut(self.active.get().0) {
      Some(memory) => memory.read(address),
      None => panic!("Invalid bank {} selected", self.active.get().0),
    }
  }

  fn write(&mut self, address: u16, value: u8) {
    match self.banks.get_mut(self.active.get().1) {
      Some(memory) => memory.write(address, value),
      None => panic!("Invalid bank {} selected", self.active.get().1),
    }
  }

  fn reset(&mut self) {
    for memory in self.banks.iter_mut() {
      memory.reset();
    }
  }

  fn poll(&mut self, cycles: u32, info: &SystemInfo) -> ActiveInterrupt {
    let mut highest = ActiveInterrupt::None;

    for mapped in &mut self.banks {
      let interrupt = mapped.poll(cycles, info);

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
