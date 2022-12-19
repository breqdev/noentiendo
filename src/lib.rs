#![doc = include_str!("../README.md")]

mod execute;
mod fetch;
pub mod keyboard;
pub mod memory;
pub mod platform;
mod registers;
pub mod roms;
pub mod system;
pub mod systems;

#[cfg(target_arch = "wasm32")]
extern crate console_error_panic_hook;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn main(roms: &JsValue, system: &JsValue) {
  console_error_panic_hook::set_once();

  use js_sys::Reflect;
  use keyboard::KeyMappingStrategy;
  use platform::{AsyncPlatform, CanvasPlatform, Platform};
  use systems::{
    pet::PetSystemConfig, vic::Vic20SystemConfig, PetSystemFactory, PetSystemRoms, SystemFactory,
    Vic20SystemFactory, Vic20SystemRoms,
  };
  use wasm_bindgen_futures::spawn_local;

  let mut platform = CanvasPlatform::new();

  let pet_object = Reflect::get(&roms, &JsValue::from_str("pet")).unwrap();
  let vic_object = Reflect::get(&roms, &JsValue::from_str("vic")).unwrap();

  let pet_roms = PetSystemRoms::from_jsvalue(&pet_object);
  let vic_roms = Vic20SystemRoms::from_jsvalue(&vic_object);

  let system = match system.as_string().unwrap().as_str() {
    "pet" => PetSystemFactory::create(
      pet_roms,
      PetSystemConfig {
        mapping: KeyMappingStrategy::Symbolic,
      },
      platform.provider(),
    ),
    "vic" => Vic20SystemFactory::create(
      vic_roms,
      Vic20SystemConfig {
        mapping: KeyMappingStrategy::Symbolic,
      },
      platform.provider(),
    ),
    _ => panic!("Unknown system"),
  };

  spawn_local(async move {
    platform.run_async(system).await;
  });
}
