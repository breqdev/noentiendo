use crate::roms::RomFile;
use js_sys::Uint8Array;

/// Represents a value which can be loaded from a Uint8Array.
pub trait JsValueLoadable {
  /// Creates a ROM file by loading the contents of a Uint8Array.
  fn from_uint8array(array: &Uint8Array) -> Self;
}

impl JsValueLoadable for RomFile {
  #[cfg(target_arch = "wasm32")]
  fn from_uint8array(array: &Uint8Array) -> Self {
    let mut data = Vec::new();
    for i in 0..array.length() {
      data.push(array.get_index(i));
    }

    RomFile::new(data)
  }
}
