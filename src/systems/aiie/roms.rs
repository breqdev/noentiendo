use crate::roms::RomFile;

/// The set of ROM files required to run a Commodore 64 system.
/// Roms from the ROMS directory in https://mirrors.apple2.org.za/ftp.apple.asimov.net/emulators/rom_images/
pub struct AiieSystemRoms {
  /// Character ROM. Used to generate the 8x8 character bitmaps.
  // pub character: RomFile,

  /// Applesoft ROM. Contains the Applesoft BASIC interpreter.
  pub applesoft: RomFile,

  /// Monitor ROM. Contains the operating system.
  pub monitor: RomFile,
}

impl AiieSystemRoms {
  #[cfg(not(target_arch = "wasm32"))]
  pub fn from_disk() -> Self {
    use crate::roms::DiskLoadable;

    // let character = RomFile::from_file("aiie/char.bin");
    let applesoft = RomFile::from_file("aiie/applesoft.bin");
    let monitor = RomFile::from_file("aiie/monitor.bin");

    Self {
      // character,
      applesoft,
      monitor,
    }
  }
}
