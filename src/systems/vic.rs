use crate::memory::{BlockMemory, BranchMemory, NullMemory, RomFile};
use crate::platform::PlatformProvider;
use crate::system::System;
use crate::systems::SystemFactory;
use std::sync::Arc;

pub struct Vic20SystemRoms {
  pub character: RomFile,
  pub basic: RomFile,
  pub kernal: RomFile,
}

impl Vic20SystemRoms {
  #[cfg(not(target_arch = "wasm32"))]
  pub fn from_disk() -> Self {
    let character = RomFile::from_file("vic/char.bin");
    let basic = RomFile::from_file("vic/basic.bin");
    let kernal = RomFile::from_file("vic/kernal.bin");

    Self {
      character,
      basic,
      kernal,
    }
  }
}

pub struct Vic20SystemFactory {}

impl SystemFactory<Vic20SystemRoms> for Vic20SystemFactory {
  fn create(roms: Vic20SystemRoms, platform: Arc<dyn PlatformProvider>) -> System {
    let low_ram = BlockMemory::ram(0x0400);
    let main_ram = BlockMemory::ram(0x1000);
    let expansion_ram = NullMemory::new();

    let characters = BlockMemory::from_file(0x1000, roms.character);

    let basic_rom = BlockMemory::from_file(0x2000, roms.basic);

    let kernel_rom = BlockMemory::from_file(0x2000, roms.kernal);

    let memory = BranchMemory::new()
      .map(0x0000, Box::new(low_ram))
      .map(0x1000, Box::new(main_ram))
      .map(0x2000, Box::new(expansion_ram))
      .map(0x8000, Box::new(characters))
      .map(0xC000, Box::new(basic_rom))
      .map(0xE000, Box::new(kernel_rom));

    System::new(Box::new(memory), 1_000_000)
  }
}
