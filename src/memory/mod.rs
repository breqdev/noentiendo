mod block;
mod branch;
mod null;
pub mod pia;
mod romfile;

pub use block::BlockMemory;
pub use branch::BranchMemory;
pub use null::NullMemory;
pub use romfile::RomFile;

pub enum ActiveInterrupt {
  None,
  NMI,
  IRQ,
}

pub struct SystemInfo {
  pub cycles_per_second: u64,
  pub cycle_count: u64,
}

pub trait Memory: Send {
  fn read(&mut self, address: u16) -> u8;
  fn write(&mut self, address: u16, value: u8);
  fn reset(&mut self);
  fn poll(&mut self, info: &SystemInfo) -> ActiveInterrupt;
}
