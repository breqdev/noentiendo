#[cfg(not(target_arch = "wasm32"))]
mod disk;

#[cfg(target_arch = "wasm32")]
mod wasm;

#[cfg(not(target_arch = "wasm32"))]
pub use disk::DiskLoadable;

#[cfg(target_arch = "wasm32")]
pub use wasm::JsValueLoadable;

/// Represents a predefined, immutable ROM file.
/// Useful for storing character, BASIC, kernal, etc. ROMs.
#[derive(Clone, Debug)]
pub struct RomFile {
  data: Vec<u8>,
}

impl RomFile {
  /// Creates a new ROM file from the given data.
  pub fn new(data: Vec<u8>) -> Self {
    Self { data }
  }

  /// Returns the contents of the ROM file.
  pub fn get_data(self) -> Vec<u8> {
    self.data
  }
}
