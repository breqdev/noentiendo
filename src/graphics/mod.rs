mod null;
pub mod scancodes;

#[cfg(not(target_arch = "wasm32"))]
mod winit;
#[cfg(not(target_arch = "wasm32"))]
pub use self::winit::{WinitGraphicsProvider, WinitGraphicsService};

#[cfg(target_arch = "wasm32")]
mod canvas;
#[cfg(target_arch = "wasm32")]
pub use self::canvas::{CanvasGraphicsProvider, CanvasGraphicsService};

pub use self::null::{NullGraphicsProvider, NullGraphicsService};
use async_trait::async_trait;
use std::sync::Arc;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color {
  r: u8,
  g: u8,
  b: u8,
}

impl Color {
  pub fn new(r: u8, g: u8, b: u8) -> Self {
    Self { r, g, b }
  }

  pub fn to_rgba(&self) -> [u8; 4] {
    [self.r, self.g, self.b, 255]
  }

  pub fn to_rgb(&self) -> u32 {
    (self.r as u32) << 16 | (self.g as u32) << 8 | self.b as u32
  }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WindowConfig {
  width: u32,
  height: u32,
  scale: f64,
}

impl WindowConfig {
  pub fn new(width: u32, height: u32, scale: f64) -> Self {
    Self {
      width,
      height,
      scale,
    }
  }
}

#[async_trait(?Send)]
pub trait GraphicsService<GraphicsState> {
  fn init(&mut self) -> GraphicsState;
  async fn init_async(&mut self) -> GraphicsState;

  fn run(&mut self, state: GraphicsState);

  fn provider(&self) -> Arc<dyn GraphicsProvider>;
}

pub trait GraphicsProvider: Send + Sync {
  fn configure_window(&self, config: WindowConfig);

  fn set_pixel(&self, x: u32, y: u32, color: Color);
  fn is_pressed(&self, key: u8) -> bool;
  fn get_last_key(&self) -> u8;
}
