use crate::graphics::{Color, GraphicsProvider, GraphicsService, WindowConfig};
use async_trait::async_trait;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub struct NullGraphicsService {}

impl NullGraphicsService {
  pub fn new() -> Self {
    Self {}
  }
}

#[async_trait(?Send)]
impl GraphicsService<()> for NullGraphicsService {
  fn init(&mut self) -> () {
    ()
  }

  async fn init_async(&mut self) -> () {
    ()
  }

  fn run(&mut self, _state: ()) {
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

  fn set_pixel(&self, _x: u32, _y: u32, _color: Color) {}

  fn is_pressed(&self, _key: u8) -> bool {
    false
  }

  fn get_last_key(&self) -> u8 {
    0
  }
}
