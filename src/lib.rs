mod execute;
mod fetch;
pub mod graphics;
mod isomorphic;
pub mod memory;
mod registers;
pub mod system;
pub mod systems;
mod time;

#[cfg(feature = "web")]
extern crate console_error_panic_hook;

#[cfg(feature = "web")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "web")]
use js_sys::Uint8Array;

#[cfg(feature = "web")]
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn main(rom: &Uint8Array) {
  console_error_panic_hook::set_once();

  use graphics::{GraphicsService, NullGraphicsService};
  use systems::{BrookeSystemFactory, SystemFactory};

  let graphics = NullGraphicsService::new();
  let romfile = memory::RomFile::from_uint8array(rom);

  let mut system =
    BrookeSystemFactory::create(memory::RomFile::from_uint8array(rom), graphics.provider());

  system.reset();

  loop {
    system.tick();
  }
}
