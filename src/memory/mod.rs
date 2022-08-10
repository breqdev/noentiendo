mod block;
mod branch;
pub mod easy;
mod null;
pub mod pet;
pub mod pia;
mod stdio;
pub mod systems;

pub use block::BlockMemory;
pub use branch::BranchMemory;
pub use null::NullMemory;
pub use stdio::MappedStdIO;

pub enum ActiveInterrupt {
  None,
  NMI,
  IRQ,
}

pub struct SystemInfo {
  pub cycle_count: u64,
}

pub trait Memory: Send {
  fn read(&mut self, address: u16) -> u8;
  fn write(&mut self, address: u16, value: u8);
  fn reset(&mut self);
  fn poll(&mut self, info: &SystemInfo) -> ActiveInterrupt;
}
