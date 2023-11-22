use crate::roms::RomFile;

/// The set of ROM files required to run a Commodore 64 system.
/// Roms from the ROMS directory in https://mirrors.apple2.org.za/ftp.apple.asimov.net/emulators/rom_images/
pub struct AiieSystemRoms {
  /// Character ROM. Used to generate the 7x8 character bitmaps.
  pub character: RomFile,

  /// Firmware ROM. Contains text drawing and I/O routines.
  pub firmware: RomFile,

  /// AppleSoft BASIC ROM. Contains the BASIC interpreter and editor.
  pub applesoft: RomFile,

  /// Monitor ROM. Contains reset vectors and the system monitor.
  pub monitor: RomFile,
}

impl AiieSystemRoms {
  #[cfg(not(target_arch = "wasm32"))]
  pub fn from_disk() -> Self {
    use crate::roms::DiskLoadable;

    let character = RomFile::from_file("aiie/character.bin");
    let firmware = RomFile::from_file("aiie/firmware.bin");
    let applesoft = RomFile::from_file("aiie/applesoft.bin");
    let monitor = RomFile::from_file("aiie/monitor.bin");

    Self {
      character,
      firmware,
      applesoft,
      monitor,
    }
  }
}
