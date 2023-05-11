use super::{ActiveInterrupt, Memory, SystemInfo};

pub struct LoggingMemory(pub Box<dyn Memory>);

impl Memory for LoggingMemory {
  fn read(&mut self, address: u16) -> u8 {
    let value = self.0.read(address);
    println!(
      "[Memory Read]: address {:04X}, value {:02X}",
      address, value
    );
    value
  }

  fn write(&mut self, address: u16, value: u8) {
    self.0.write(address, value);
    println!(
      "[Memory Write]: address {:04X}, value {:02X}",
      address, value
    );
  }

  fn reset(&mut self) {
    self.0.reset();
    println!("[Memory Reset]");
  }

  fn poll(&mut self, cycles: u32, info: &SystemInfo) -> ActiveInterrupt {
    println!("[Memory Poll]");
    self.0.poll(cycles, info)
  }
}
