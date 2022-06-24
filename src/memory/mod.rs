mod block;
mod branch;
pub mod easy;
mod mappedio;
mod null;
pub mod systems;

pub use block::BlockMemory;
pub use branch::BranchMemory;
pub use mappedio::MappedIO;
pub use null::NullMemory;

// Commodore PET-style column screen memory
// (see https://www.chibiakumas.com/6502/platform4.php#LessonP38 for details)

pub trait Memory {
  fn read(&self, address: u16) -> u8;
  fn write(&mut self, address: u16, value: u8);
  fn tick(&mut self);
  fn reset(&mut self);
}
