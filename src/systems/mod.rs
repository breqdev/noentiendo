use crate::{
  cpu::Cpu,
  platform::{PlatformProvider, WindowConfig},
  trace::TraceHandler,
};
use instant::Duration;
use std::sync::Arc;

pub mod basic;
pub mod c64;
pub mod easy;
pub mod klaus;
pub mod pet;
pub mod vic;

pub trait BuildableSystem<RomRegistry, SystemConfig> {
  /// Instantiate this system from the given roms, configuration, and with I/O provided by the given
  /// platform provider.
  fn build(
    roms: RomRegistry,
    config: SystemConfig,
    platform: Arc<dyn PlatformProvider>,
  ) -> Box<dyn System>;
}

/// A representation of an emulated system.
pub trait System {
  /// Return a mutable reference to the CPU used in this system.
  fn get_cpu_mut(&mut self) -> Box<&mut dyn Cpu>;

  fn attach_trace_handler(&mut self, handler: Box<dyn TraceHandler>) {
    self.get_cpu_mut().attach_trace_handler(handler);
  }

  /// Advance the system by one tick.
  fn tick(&mut self) -> Duration;

  /// Reset the system's state.
  fn reset(&mut self);

  /// Render the current state of the system to the given framebuffer.
  fn render(&mut self, framebuffer: &mut [u8], window: WindowConfig);

  /// Clean up any resources used by this system.
  fn cleanup(&mut self) -> Result<(), &str> {
    self.get_cpu_mut().cleanup()
  }
}
