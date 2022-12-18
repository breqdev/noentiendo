use crate::platform::PlatformProvider;
use crate::system::System;
use std::sync::Arc;

pub mod brooke;
pub mod easy;
pub mod klaus;
pub mod pet;
pub mod vic;

pub use brooke::BrookeSystemFactory;
pub use easy::EasySystemFactory;
pub use klaus::KlausSystemFactory;
pub use pet::{PetSystemFactory, PetSystemRoms};
pub use vic::{Vic20SystemFactory, Vic20SystemRoms};

/// A factory for creating a system from a set of ROM files and a platform.
pub trait SystemFactory<RomRegistry, SystemConfig> {
  fn create(roms: RomRegistry, config: SystemConfig, platform: Arc<dyn PlatformProvider>)
    -> System;
}
