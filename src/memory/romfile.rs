#[cfg(target_arch = "wasm32")]
use js_sys::Uint8Array;

#[cfg(not(target_arch = "wasm32"))]
use std::fs::File;

#[cfg(not(target_arch = "wasm32"))]
use std::io::Read;

/// Represents a predefined, immutable ROM file.
/// Useful for storing character, BASIC, kernal, etc. ROMs.
pub struct RomFile {
  data: Vec<u8>,
}

impl RomFile {
  /// Creates a ROM file by loading the contents of a file.
  #[cfg(not(target_arch = "wasm32"))]
  pub fn from_file(path: &str) -> Self {
    let mut file = File::open(path).map_err(|e| e.to_string()).unwrap();
    let mut data = Vec::new();
    file
      .read_to_end(&mut data)
      .map_err(|e| e.to_string())
      .unwrap();
    Self { data }
  }

  /// Creates a ROM file by loading the contents of a Uint8Array.
  #[cfg(target_arch = "wasm32")]
  pub fn from_uint8array(array: &Uint8Array) -> Self {
    let mut data = Vec::new();
    for i in 0..array.length() {
      data.push(array.get_index(i));
    }
    Self { data }
  }

  /// Returns the contents of the ROM file.
  pub fn get_data(self) -> Vec<u8> {
    self.data
  }
}
