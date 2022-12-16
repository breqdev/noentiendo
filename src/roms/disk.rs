use crate::roms::RomFile;
use std::{fs::File, io::Read};

pub trait DiskLoadable {
  fn from_file(path: &str) -> Self;
}

/// Creates a ROM file by loading the contents of a file.
impl DiskLoadable for RomFile {
  fn from_file(path: &str) -> Self {
    let mut file = File::open(path).expect("Could not open file");
    let mut data = Vec::new();
    file.read_to_end(&mut data).expect("Could not read file");

    RomFile::new(data)
  }
}
