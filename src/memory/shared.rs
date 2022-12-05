use crate::memory::{ActiveInterrupt, Memory, SystemInfo};
use std::sync::{Arc, Mutex};

pub struct SharedMemory {
  backing: Arc<Mutex<Box<dyn Memory>>>,
}

impl SharedMemory {
  pub fn new(backing: Box<dyn Memory>) -> Self {
    Self {
      backing: Arc::new(Mutex::new(backing)),
    }
  }

  pub fn clone(&self) -> Self {
    Self {
      backing: self.backing.clone(),
    }
  }
}

impl Memory for SharedMemory {
  fn read(&mut self, address: u16) -> u8 {
    self.backing.lock().unwrap().read(address)
  }

  fn write(&mut self, address: u16, value: u8) {
    self.backing.lock().unwrap().write(address, value)
  }

  fn reset(&mut self) {
    self.backing.lock().unwrap().reset()
  }

  fn poll(&mut self, system_info: &SystemInfo) -> ActiveInterrupt {
    self.backing.lock().unwrap().poll(system_info)
  }
}
