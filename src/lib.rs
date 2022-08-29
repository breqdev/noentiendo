mod execute;
mod fetch;
pub mod graphics;
pub mod memory;
mod registers;
pub mod system;
mod utils;

#[cfg(feature = "web")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "web")]
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
  fn alert(s: &str);
}

#[cfg(feature = "web")]
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn greet() {
  alert("Hello from noentiendo!");
}
