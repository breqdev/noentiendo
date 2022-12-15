#![doc = include_str!("../README.md")]

mod execute;
mod fetch;
pub mod memory;
pub mod platform;
mod registers;
pub mod system;
pub mod systems;


#[cfg(target_arch = "wasm32")]
extern crate console_error_panic_hook;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use js_sys::Uint8Array;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub async fn main(
  basic: Uint8Array,
  character: Uint8Array,
  // editor: Uint8Array,
  kernal: Uint8Array,
) {
  console_error_panic_hook::set_once();

  use platform::{AsyncPlatform, CanvasPlatform, Platform};
  use systems::{SystemFactory, Vic20SystemFactory, Vic20SystemRoms};

  let mut platform = CanvasPlatform::new();
  // platform
  //   .provider()
  //   .request_window(platform::WindowConfig::new(1, 1, 2.0));

  // let romfile = memory::RomFile::from_uint8array(&rom);
  let basic = memory::RomFile::from_uint8array(&basic);
  let character = memory::RomFile::from_uint8array(&character);
  // let editor = memory::RomFile::from_uint8array(&editor);
  let kernal = memory::RomFile::from_uint8array(&kernal);

  // let roms = systems::PetSystemRoms {
  //   character,
  //   basic,
  //   editor,
  //   kernal,
  // };

  let roms = Vic20SystemRoms {
    character,
    basic,
    kernal,
  };

  let system = Vic20SystemFactory::create(roms, platform.provider());

  platform.run_async(system).await;
}
