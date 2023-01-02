use crate::platform::{PlatformProvider, WindowConfig};
use instant::Duration;
use std::sync::Arc;

pub mod aiie;
pub mod basic;
pub mod c64;
pub mod easy;
pub mod klaus;
pub mod pet;
pub mod vic;

pub trait SystemBuilder<SystemType, RomRegistry, SystemConfig> {
  /// Create a new system from the given roms, configuration, and with I/O provided by the given
  /// platform provider.
  fn build(
    roms: RomRegistry,
    config: SystemConfig,
    platform: Arc<dyn PlatformProvider>,
  ) -> Box<dyn System>;
}

/// A representation of an emulated system.
pub trait System {
  /// Advance the system by one tick.
  fn tick(&mut self) -> Duration;

  /// Reset the system's state.
  fn reset(&mut self);

  /// Render the current state of the system to the given framebuffer.
  fn render(&mut self, framebuffer: &mut [u8], window: WindowConfig);
}
