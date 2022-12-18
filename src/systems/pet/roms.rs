use crate::roms::RomFile;

#[cfg(target_arch = "wasm32")]
use js_sys::{Reflect, Uint8Array};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{JsCast, JsValue};

/// The set of ROM files required to run a PET system.
pub struct PetSystemRoms {
  /// Character ROM. Used to generate the 8x8 character bitmaps.
  pub character: RomFile,

  /// Basic ROM. Contains the BASIC interpreter.
  pub basic: RomFile,

  /// Editor ROM. Contains the screen editor functions.
  pub editor: RomFile,

  /// Kernal ROM. Contains the operating system.
  pub kernal: RomFile,
}

impl PetSystemRoms {
  #[cfg(not(target_arch = "wasm32"))]
  pub fn from_disk() -> Self {
    use crate::roms::DiskLoadable;

    let character = RomFile::from_file("pet/char.bin");
    let basic = RomFile::from_file("pet/basic.bin");
    let editor = RomFile::from_file("pet/editor.bin");
    let kernal = RomFile::from_file("pet/kernal.bin");

    Self {
      character,
      basic,
      editor,
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
    let editor = Reflect::get(value, &JsValue::from_str("editor"))
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
      editor: RomFile::from_uint8array(&editor),
      kernal: RomFile::from_uint8array(&kernal),
    }
  }
}
