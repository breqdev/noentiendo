#[cfg(feature = "web")]
use js_sys::Uint8Array;

#[cfg(feature = "desktop")]
use std::fs::File;

#[cfg(feature = "desktop")]
use std::io::Read;

pub struct RomFile {
  data: Vec<u8>,
}

impl RomFile {
  pub fn new() -> Self {
    Self { data: Vec::new() }
  }

  #[cfg(feature = "desktop")]
  pub fn from_file(path: &str) -> Self {
    let mut file = File::open(path).map_err(|e| e.to_string()).unwrap();
    let mut data = Vec::new();
    file
      .read_to_end(&mut data)
      .map_err(|e| e.to_string())
      .unwrap();
    Self { data }
  }

  #[cfg(feature = "web")]
  pub fn from_uint8array(array: &Uint8Array) -> Self {
    let mut data = Vec::new();
    for i in 0..array.length() {
      data.push(array.get_index(i));
    }
    Self { data }
  }

  pub fn get_data(self) -> Vec<u8> {
    self.data
  }
}
