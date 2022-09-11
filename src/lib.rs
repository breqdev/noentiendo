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
pub async fn main(rom: Uint8Array) {
  console_error_panic_hook::set_once();

  use platform::{CanvasPlatform, Platform};
  use systems::{EasySystemFactory, SystemFactory};

  let mut platform = CanvasPlatform::new();
  // platform
  //   .provider()
  //   .request_window(platform::WindowConfig::new(1, 1, 2.0));

  let romfile = memory::RomFile::from_uint8array(&rom);

  let system = EasySystemFactory::create(romfile, platform.provider());

  platform.run_async(system).await;
}
