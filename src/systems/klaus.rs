use instant::Duration;

use crate::cpu::{
  mos6502::{Mos6502, Mos6502Variant},
  Cpu,
};
use crate::memory::BlockMemory;
use crate::platform::{PlatformProvider, WindowConfig};
use crate::roms::RomFile;
use crate::systems::System;
use std::cell::Cell;
use std::rc::Rc;
use std::sync::Arc;

use super::SystemBuilder;

pub struct KlausSystemConfig {
  pub pc_report: Option<Rc<Cell<u16>>>,
  pub variant: Mos6502Variant,
}

/// A factory for creating a system that runs Klaus Dormann's 6502 CPU test suite.
pub struct KlausSystemBuilder;

impl SystemBuilder<KlausSystem, RomFile, KlausSystemConfig> for KlausSystemBuilder {
  fn build(
    rom: RomFile,
    config: KlausSystemConfig,
    _platform: Arc<dyn PlatformProvider>,
  ) -> Box<dyn System> {
    let rom = BlockMemory::from_file(0x10000, rom).set_writeable(true);
    let mut cpu = Mos6502::new(rom, config.variant);

    cpu.registers.pc.load(0x0400);

    Box::new(KlausSystem {
      cpu,
      pc: config.pc_report,
    })
  }
}

/// A system used to run Klaus Dormann's 6502 CPU test suite.
pub struct KlausSystem {
  cpu: Mos6502,
  pc: Option<Rc<Cell<u16>>>,
}

impl System for KlausSystem {
  fn tick(&mut self) -> Duration {
    self.cpu.tick();
    if let Some(pc) = &self.pc {
      pc.set(self.cpu.registers.pc.address());
    }
    Duration::ZERO
  }

  fn reset(&mut self) {
    self.cpu.reset();
  }

  fn render(&mut self, _framebuffer: &mut [u8], _window: WindowConfig) {}
}

#[cfg(test)]
mod tests {
  use crate::{
    platform::{Platform, TextPlatform},
    roms::DiskLoadable,
  };

  use super::*;

  #[test]
  fn test_klaus_6502() {
    let roms = RomFile::from_file("bin/klaus_6502.bin");
    let platform = TextPlatform::new();
    let pc = Rc::new(Cell::new(0));

    let mut system = KlausSystemBuilder::build(
      roms,
      KlausSystemConfig {
        pc_report: Some(pc.clone()),
        variant: Mos6502Variant::NMOS,
      },
      platform.provider(),
    );

    for _ in 0..=100000000 {
      system.tick();
    }

    assert_eq!(pc.get(), 0x3469);
  }

  #[test]
  fn test_klaus_65c02() {
    let roms = RomFile::from_file("bin/klaus_65C02.bin");
    let platform = TextPlatform::new();
    let pc = Rc::new(Cell::new(0));

    let mut system = KlausSystemBuilder::build(
      roms,
      KlausSystemConfig {
        pc_report: Some(pc.clone()),
        variant: Mos6502Variant::CMOS,
      },
      platform.provider(),
    );

    for _ in 0..=100000000 {
      system.tick();
    }

    assert_eq!(pc.get(), 0x24f1);
  }
}
