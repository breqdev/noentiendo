use std::rc::Rc;

use crate::memory::{BlockMemory, BranchMemory};
use crate::platform::PlatformProvider;
use crate::roms::RomFile;
use crate::system::System;
use crate::systems::SystemFactory;

/// A system used to run Klaus Dormann's 6502 CPU test suite.
pub struct KlausSystemFactory {}

impl SystemFactory<RomFile, ()> for KlausSystemFactory {
  fn create(rom: RomFile, _config: (), platform: Box<dyn PlatformProvider>) -> System {
    let rom = BlockMemory::from_file(0x10000, rom);

    let memory = BranchMemory::new().map(0x0000, Box::new(rom));

    System::new(Rc::new(memory), platform, 0)
  }
}
