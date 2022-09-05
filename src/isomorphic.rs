#[cfg(feature = "desktop")]
use rand::random;

#[cfg(feature = "desktop")]
use std::io::Write;

#[cfg(feature = "web")]
use wasm_bindgen::prelude::*;

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

#[cfg(feature = "desktop")]
pub fn readline() -> String {
  let mut input = String::new();
  print!("> ");
  std::io::stdout().flush().unwrap();
  std::io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");
  input
}

#[cfg(feature = "web")]
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
  fn prompt(message: &str) -> String;
  fn alert(message: &str);
}

#[cfg(feature = "web")]
pub fn readline() -> String {
  prompt(">")
}

#[cfg(all(not(feature = "desktop"), not(feature = "web")))]
pub fn readline() -> String {
  unimplemented!("not implemented on this platform")
}

#[cfg(feature = "desktop")]
pub fn writeline(message: &str) {
  println!("{}", message);
}

#[cfg(feature = "web")]
pub fn writeline(message: &str) {
  alert(message);
}

#[cfg(all(not(feature = "desktop"), not(feature = "web")))]
pub fn writeline(message: &str) {
  unimplemented!("not implemented on this platform")
}
