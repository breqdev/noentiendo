use crate::cpu::Mos6502;
use crate::platform::PlatformProvider;
use std::sync::Arc;

pub mod brooke;
pub mod c64;
pub mod easy;
pub mod klaus;
pub mod pet;
pub mod vic;

pub use brooke::BrookeSystemFactory;
pub use c64::{C64SystemConfig, C64SystemFactory, C64SystemRoms};
pub use easy::EasySystemFactory;
pub use klaus::KlausSystemFactory;
pub use pet::{PetSystemConfig, PetSystemFactory, PetSystemRoms};
pub use vic::{Vic20SystemConfig, Vic20SystemFactory, Vic20SystemRoms};

/// A factory for creating a system from a set of ROM files and a platform.
pub trait SystemFactory<RomRegistry, SystemConfig> {
  fn create(
    roms: RomRegistry,
    config: SystemConfig,
    platform: Arc<dyn PlatformProvider>,
  ) -> Mos6502;
}
