pub mod mos6502;

pub trait Cpu {
  fn reset(&mut self);

  /// Return the number of cycles elapsed since the system last reset.
  fn get_cycle_count(&self) -> u64;

  /// Execute a single instruction. Return the number of cycles elapsed.
  fn tick(&mut self) -> u8;
}
