use crate::roms::RomFile;

#[cfg(target_arch = "wasm32")]
use js_sys::Reflect;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

#[cfg(target_arch = "wasm32")]
use js_sys::Uint8Array;

/// The set of ROM files required to run a Commodore 64 system.
pub struct C64SystemRoms {
  /// Character ROM. Used to generate the 8x8 character bitmaps.
  pub character: RomFile,

  /// Basic ROM. Contains the BASIC interpreter.
  pub basic: RomFile,

  /// Kernel ROM. Contains the operating system.
  pub kernal: RomFile,
}

impl C64SystemRoms {
  #[cfg(not(target_arch = "wasm32"))]
  pub fn from_disk() -> Self {
    use crate::roms::DiskLoadable;

    let character = RomFile::from_file("c64/char.bin");
    let basic = RomFile::from_file("c64/basic.bin");
    let kernal = RomFile::from_file("c64/kernal.bin");

    Self {
      character,
      basic,
      kernal,
    }
  }

  #[cfg(target_arch = "wasm32")]
  pub fn from_jsvalue(value: &JsValue) -> Self {
    use crate::roms::JsValueLoadable;

    let character = Reflect::get(value, &JsValue::from_str("char"))
      .unwrap()
      .dyn_into::<Uint8Array>()
      .unwrap();
    let basic = Reflect::get(value, &JsValue::from_str("basic"))
      .unwrap()
      .dyn_into::<Uint8Array>()
      .unwrap();
    let kernal = Reflect::get(value, &JsValue::from_str("kernal"))
      .unwrap()
      .dyn_into::<Uint8Array>()
      .unwrap();

    Self {
      character: RomFile::from_uint8array(&character),
      basic: RomFile::from_uint8array(&basic),
      kernal: RomFile::from_uint8array(&kernal),
    }
  }
}
