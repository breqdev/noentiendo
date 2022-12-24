use std::{cell::Cell, rc::Rc};

use super::{ActiveInterrupt, Memory, SystemInfo};

pub struct BankedMemory {
  banks: Vec<Box<dyn Memory>>,
  active: Rc<Cell<usize>>,
}

impl BankedMemory {
  pub fn new(active: Rc<Cell<usize>>) -> Self {
    Self {
      banks: Vec::new(),
      active,
    }
  }

  pub fn bank(mut self, memory: Box<dyn Memory>) -> Self {
    self.banks.push(memory);

    self
  }
}

impl Memory for BankedMemory {
  fn read(&mut self, address: u16) -> u8 {
    match self.banks.get_mut(self.active.get()) {
      Some(memory) => memory.read(address),
      None => panic!("Invalid bank {} selected", self.active.get()),
    }
  }

  fn write(&mut self, address: u16, value: u8) {
    match self.banks.get_mut(self.active.get()) {
      Some(memory) => memory.write(address, value),
      None => panic!("Invalid bank {} selected", self.active.get()),
    }
  }

  fn reset(&mut self) {
    for memory in self.banks.iter_mut() {
      memory.reset();
    }
  }

  fn poll(&mut self, info: &SystemInfo) -> ActiveInterrupt {
    let mut highest = ActiveInterrupt::None;

    for mapped in &mut self.banks {
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