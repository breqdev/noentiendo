use instant::{Duration, Instant};

use crate::cpu::Mos6502;
use crate::memory::BlockMemory;
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
    let mut cpu = Mos6502::new(Box::new(rom));

    cpu.registers.pc.load(0x0400);

    Box::new(KlausSystem {
      cpu,
      last_report: Instant::now(),
    })
  }
}

/// A system used to run Klaus Dormann's 6502 CPU test suite.
pub struct KlausSystem {
  cpu: Mos6502,
  last_report: Instant,
}

impl System for KlausSystem {
  fn tick(&mut self) -> Duration {
    if self.last_report.elapsed().as_secs() > 1 {
      println!("PC: {:04x}", self.cpu.registers.pc.address());
      self.last_report = Instant::now();
    };
    Duration::from_secs_f64(1.0 / 1_000_000.0) * self.cpu.tick().into()
  }

  fn reset(&mut self) {
    self.cpu.reset();
  }

  fn render(&mut self, _framebuffer: &mut [u8], _window: WindowConfig) {}
}
