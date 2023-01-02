use crate::roms::RomFile;

/// The set of ROM files required to run a Commodore 64 system.
pub struct AiieSystemRoms {
  /// Character ROM. Used to generate the 8x8 character bitmaps.
  pub character: RomFile,

  /// Basic ROM. Contains the BASIC interpreter.
  // pub basic: RomFile,

  /// Kernel ROM. Contains the operating system.
  pub kernal: RomFile,
}

impl AiieSystemRoms {
  #[cfg(not(target_arch = "wasm32"))]
  pub fn from_disk() -> Self {
    use crate::roms::DiskLoadable;

    let character = RomFile::from_file("aiie/char.bin");
    // let basic = RomFile::from_file("aiie/basic.bin");
    let kernal = RomFile::from_file("aiie/kernal.bin");

    Self {
      character,
      // basic,
      kernal,
    }
  }
}
