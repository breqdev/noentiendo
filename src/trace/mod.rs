#[cfg(not(target_arch = "wasm32"))]
pub mod file;

/// Trace information provided after each instruction by the CPU.
pub struct CpuTrace {
  pub address: u16,
  pub opcode: u8,
}

/// An item which can handle a CPU trace (e.g. logging to a file)
pub trait TraceHandler {
  /// Handle a trace event.
  fn handle(&mut self, trace: &CpuTrace);
}
