use crate::roms::RomFile;
use std::{fs::File, io::Read};

/// Represents a value which can be loaded from a file.
pub trait DiskLoadable {
  /// Creates a ROM file by loading the contents of a file.
  fn from_file(path: &str) -> Self;
}

impl DiskLoadable for RomFile {
  fn from_file(path: &str) -> Self {
    let mut file = File::open(path).expect("Could not open file");
    let mut data = Vec::new();
    file.read_to_end(&mut data).expect("Could not read file");

    RomFile::new(data)
  }
}
