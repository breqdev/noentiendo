mod execute;
mod fetch;
pub mod graphics;
mod isomorphic;
pub mod memory;
mod registers;
pub mod system;
pub mod systems;
mod time;

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

  use graphics::{CanvasGraphicsService, GraphicsService};
  use systems::{EasySystemFactory, SystemFactory};

  let mut graphics = CanvasGraphicsService::new();
  let romfile = memory::RomFile::from_uint8array(&rom);

  let mut system = EasySystemFactory::create(romfile, graphics.provider());

  let state = graphics.init_async().await;

  system.reset();

  graphics.run(state);
}
