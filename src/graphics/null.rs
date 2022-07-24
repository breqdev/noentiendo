use crate::graphics::{Color, GraphicsProvider, GraphicsService, WindowConfig};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub struct NullGraphicsService {}

impl NullGraphicsService {
  pub fn new() -> Self {
    Self {}
  }
}

impl GraphicsService for NullGraphicsService {
  fn run(&mut self) {
    loop {
      thread::sleep(Duration::from_millis(10));
    }
  }

  fn provider(&self) -> Arc<dyn GraphicsProvider> {
    Arc::new(NullGraphicsProvider::new())
  }
}

pub struct NullGraphicsProvider {}

impl NullGraphicsProvider {
  pub fn new() -> Self {
    Self {}
  }
}

impl GraphicsProvider for NullGraphicsProvider {
  fn configure_window(&self, _config: WindowConfig) {}

  fn wait_for_pixels(&self) {}

  fn set_pixel(&self, _x: u32, _y: u32, _color: Color) {}

  fn get_last_key(&self) -> u8 {
    0
  }
}
