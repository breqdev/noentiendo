use crate::trace::TraceHandler;

pub mod mos6502;

pub trait Cpu {
  /// Reset this CPU, clearing internal state.
  fn reset(&mut self);

  /// Attach the given handler to receive trace events from this CPU.
  fn attach_trace_handler(&mut self, trace: Box<dyn TraceHandler>);

  /// Return the number of cycles elapsed since the system last reset.
  fn get_cycle_count(&self) -> u64;

  /// Execute a single instruction. Return the number of cycles elapsed.
  fn tick(&mut self) -> u8;
}
