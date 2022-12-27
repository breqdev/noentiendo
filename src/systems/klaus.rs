use crate::cpu::Mos6502;
use crate::memory::{BlockMemory, BranchMemory};
use crate::platform::{PlatformProvider, WindowConfig};
use crate::roms::RomFile;
use crate::systems::System;
use std::sync::Arc;

use super::SystemBuilder;

/// A factory for creating a system that runs Klaus Dormann's 6502 CPU test suite.
pub struct KlausSystemBuilder;

impl SystemBuilder<KlausSystem, RomFile, ()> for KlausSystemBuilder {
  fn build(rom: RomFile, _config: (), _platform: Arc<dyn PlatformProvider>) -> Box<dyn System> {
    let rom = BlockMemory::from_file(0x10000, rom);

    let memory = BranchMemory::new().map(0x0000, Box::new(rom));

    Mos6502::new(Box::new(memory));

    Box::new(KlausSystem {})
  }
}

/// A system used to run Klaus Dormann's 6502 CPU test suite.
pub struct KlausSystem;

impl System for KlausSystem {
  fn tick(&mut self) -> instant::Duration {
    todo!()
  }

  fn reset(&mut self) {
    todo!()
  }

  fn render(&mut self, framebuffer: &mut [u8], window: WindowConfig) {
    todo!()
  }
}
