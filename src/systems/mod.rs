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

pub trait SystemFactory<RomRegistry> {
  fn create(roms: RomRegistry, platform: Arc<dyn PlatformProvider>) -> System;
}
