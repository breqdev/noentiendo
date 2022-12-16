/// Represents a predefined, immutable ROM file.
/// Useful for storing character, BASIC, kernal, etc. ROMs.
pub struct RomFile {
  data: Vec<u8>,
}

impl RomFile {
  pub fn new(data: Vec<u8>) -> Self {
    Self { data }
  }

  /// Returns the contents of the ROM file.
  pub fn get_data(self) -> Vec<u8> {
    self.data
  }
}
