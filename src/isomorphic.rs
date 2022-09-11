#[cfg(not(target_arch = "wasm32"))]
use rand::random;

#[cfg(target_arch = "wasm32")]
use js_sys::Math;

#[cfg(not(target_arch = "wasm32"))]
pub fn random_u8() -> u8 {
  random::<u8>()
}

#[cfg(target_arch = "wasm32")]
pub fn random_u8() -> u8 {
  Math::floor(Math::random() * 255.0) as u8
}
