#[cfg(feature = "desktop")]
use rand::random;

#[cfg(feature = "web")]
use js_sys::Math;

#[cfg(feature = "desktop")]
pub fn random_u8() -> u8 {
  random::<u8>()
}

#[cfg(feature = "web")]
pub fn random_u8() -> u8 {
  Math::floor(Math::random() * 255.0) as u8
}

#[cfg(all(not(feature = "desktop"), not(feature = "web")))]
pub fn random_u8() -> u8 {
  unimplemented!("not implemented on this platform")
}
