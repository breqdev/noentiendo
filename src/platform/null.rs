use crate::platform::{Color, Platform, PlatformProvider, WindowConfig};
use crate::system::System;
use async_trait::async_trait;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub struct NullPlatform {}

impl NullPlatform {
  pub fn new() -> Self {
    Self {}
  }
}

#[async_trait(?Send)]
impl Platform for NullPlatform {
  fn run(&mut self, system: System) {
    loop {
      thread::sleep(Duration::from_millis(10));
    }
  }

  async fn run_async(&mut self, system: System) {}

  fn provider(&self) -> Arc<dyn PlatformProvider> {
    Arc::new(NullPlatformProvider::new())
  }
}

pub struct NullPlatformProvider {}

impl NullPlatformProvider {
  pub fn new() -> Self {
    Self {}
  }
}

impl PlatformProvider for NullPlatformProvider {
  fn request_window(&self, _config: WindowConfig) {}

  fn set_pixel(&self, _x: u32, _y: u32, _color: Color) {}

  fn is_pressed(&self, _key: u8) -> bool {
    false
  }

  fn get_last_key(&self) -> u8 {
    0
  }
}
