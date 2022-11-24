use crate::memory::pia::{NullPort, PIA};
use crate::memory::{BlockMemory, BranchMemory, NullMemory, RomFile};
use crate::platform::PlatformProvider;
use crate::system::System;
use crate::systems::SystemFactory;
use std::sync::{Arc, Mutex};

mod character;
mod chip;
mod color;
mod vram;
use character::VicCharacterRam;
use chip::{VicChip, VicChipIO};
use color::VicColorRam;
use vram::VicVram;

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
    let main_ram = BlockMemory::ram(0x0E00);

    let vic_chip = Arc::new(Mutex::new(VicChip::new(platform, roms.character)));
    let via1 = PIA::new(Box::new(NullPort::with_warnings("VIA1 Port A")), Box::new(NullPort::with_warnings("VIA1 Port B")));
    let via2 = PIA::new(Box::new(NullPort::with_warnings("VIA2 Port A")), Box::new(NullPort::with_warnings("VIA2 Port B")));

    let basic_rom = BlockMemory::from_file(0x2000, roms.basic);
    let kernel_rom = BlockMemory::from_file(0x2000, roms.kernal);

    let vram = VicVram::new(vic_chip.clone());
    let characters = VicCharacterRam::new(vic_chip.clone());
    let colors = VicColorRam::new(vic_chip.clone());
    let chip_io = VicChipIO::new(vic_chip.clone());

    let memory = BranchMemory::new()
      .map(0x0000, Box::new(low_ram))
      .map(0x0400, Box::new(NullMemory::new()))
      .map(0x1000, Box::new(main_ram))
      .map(0x1E00, Box::new(vram))
      .map(0x2000, Box::new(NullMemory::new()))
      // .map(0x2000, Box::new(expansion_ram))
      .map(0x8000, Box::new(characters))
      .map(0x9000, Box::new(chip_io))
      .map(0x9010, Box::new(via1))
      .map(0x9020, Box::new(via2))
      .map(0x9030, Box::new(NullMemory::new()))
      .map(0x9600, Box::new(colors))
      .map(0xC000, Box::new(basic_rom))
      .map(0xE000, Box::new(kernel_rom));

    System::new(Box::new(memory), 1_000_000)
  }
}
