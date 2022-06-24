mod block;
mod branch;
mod easy;
mod mappedio;

pub use block::BlockMemory;
pub use branch::BranchMemory;
pub use easy::EasyMemory;
pub use mappedio::MappedIO;

// Commodore PET-style column screen memory
// (see https://www.chibiakumas.com/6502/platform4.php#LessonP38 for details)

pub trait Memory {
  fn read(&self, address: u16) -> u8;
  fn write(&mut self, address: u16, value: u8);
  fn tick(&mut self);
  fn reset(&mut self);
}
