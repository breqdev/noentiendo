use crate::platform::PlatformProvider;
use crate::system::System;
use std::sync::Arc;

pub mod brooke;
pub mod easy;
pub mod pet;

pub use brooke::BrookeSystemFactory;
pub use easy::EasySystemFactory;
pub use pet::PetSystemFactory;

pub trait SystemFactory<RomRegistry> {
  fn create(roms: RomRegistry, graphics: Arc<dyn PlatformProvider>) -> System;
}
