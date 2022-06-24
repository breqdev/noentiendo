use crate::memory::Memory;

pub struct BranchMemory {
  mapping: Vec<(usize, Box<dyn Memory>)>,
}

impl BranchMemory {
  pub fn new() -> Self {
    Self {
      mapping: Vec::new(),
    }
  }

  pub fn map(mut self, address: usize, memory: Box<dyn Memory>) -> Self {
    self.mapping.push((address, memory));

    self
  }
}

impl Memory for BranchMemory {
  fn read(&self, address: u16) -> u8 {
    let mut memory = None;

    for (start, mapped) in &self.mapping {
      if address as usize >= *start {
        memory = Some(mapped);
      }
    }

    match memory {
      Some(memory) => memory.read(address),
      None => 0,
    }
  }

  fn write(&mut self, address: u16, value: u8) {
    let mut memory = None;

    for (start, mapped) in &mut self.mapping {
      if address as usize >= *start {
        memory = Some(mapped);
      }
    }

    match memory {
      Some(memory) => memory.write(address, value),
      None => (),
    };
  }

  fn tick(&mut self) {
    for (_, mapped) in &mut self.mapping {
      mapped.tick();
    }
  }

  fn reset(&mut self) {
    for (_, mapped) in &mut self.mapping {
      mapped.reset();
    }
  }
}
