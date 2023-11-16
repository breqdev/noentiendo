use super::{ActiveInterrupt, Memory, SystemInfo};

pub struct LoggingMemory {
  backing: Box<dyn Memory>,
  message: String,
  offset: u16,
}

impl LoggingMemory {
  pub fn new(backing: Box<dyn Memory>, message: &str, offset: u16) -> LoggingMemory {
    LoggingMemory {
      backing,
      message: message.to_owned(),
      offset,
    }
  }
}

impl Memory for LoggingMemory {
  fn read(&mut self, address: u16) -> u8 {
    let value = self.backing.read(address);
    println!(
      "[Memory Read]: {} address {:04X}, value {:02X}",
      self.message,
      address + self.offset,
      value
    }
    value
  }

  fn write(&mut self, address: u16, value: u8) {
    self.backing.write(address, value);
    println!(
      "[Memory Write]: {} address {:04X}, value {:02X}",
      self.message,
      address + self.offset,
      value
    );
  }

  fn reset(&mut self) {
    self.backing.reset();
    println!("[Memory Reset]: {}", self.message);
  }

  fn poll(&mut self, cycles: u32, info: &SystemInfo) -> ActiveInterrupt {
    // println!("[Memory Poll]: {}", self.message);
    self.backing.poll(cycles, info)
  }
}
