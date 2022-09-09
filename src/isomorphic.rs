#[cfg(not(target_arch = "wasm32"))]
use rand::random;

#[cfg(not(target_arch = "wasm32"))]
use std::io::Write;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::JsFuture;

#[cfg(target_arch = "wasm32")]
use js_sys::{Math, Promise};

#[cfg(target_arch = "wasm32")]
use web_sys::window;

#[cfg(not(target_arch = "wasm32"))]
pub fn random_u8() -> u8 {
  random::<u8>()
}

#[cfg(target_arch = "wasm32")]
pub fn random_u8() -> u8 {
  Math::floor(Math::random() * 255.0) as u8
}

#[cfg(not(target_arch = "wasm32"))]
pub fn readline() -> String {
  let mut input = String::new();
  print!("> ");
  std::io::stdout().flush().unwrap();
  std::io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");
  input
}

#[cfg(target_arch = "wasm32")]
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
  fn prompt(message: &str) -> String;
  fn alert(message: &str);
}

#[cfg(target_arch = "wasm32")]
pub fn readline() -> String {
  prompt(">")
}

#[cfg(not(target_arch = "wasm32"))]
pub fn writeline(message: &str) {
  println!("{}", message);
}

#[cfg(target_arch = "wasm32")]
pub fn writeline(message: &str) {
  alert(message);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn sleep(duration: std::time::Duration) {
  std::thread::sleep(duration);
}

#[cfg(target_arch = "wasm32")]
pub async fn sleep(duration: f64) {
  let promise = Promise::new(&mut |yes, _| {
    let win = window().unwrap();
    win
      .set_timeout_with_callback_and_timeout_and_arguments_0(&yes, (duration * 1000.0) as i32)
      .unwrap();
  });
  let js_fut = JsFuture::from(promise);
  js_fut.await.unwrap();
}