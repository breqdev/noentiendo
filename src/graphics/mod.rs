mod null;
pub mod scancodes;

#[cfg(feature = "desktop")]
mod winit;
#[cfg(feature = "desktop")]
pub use self::winit::{WinitGraphicsProvider, WinitGraphicsService};

pub use self::null::{NullGraphicsProvider, NullGraphicsService};
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

pub trait GraphicsService {
  fn run(&mut self);
  fn provider(&self) -> Arc<dyn GraphicsProvider>;
}

pub trait GraphicsProvider: Send + Sync {
  fn configure_window(&self, config: WindowConfig);
  fn wait_for_pixels(&self);

  fn set_pixel(&self, x: u32, y: u32, color: Color);
  fn is_pressed(&self, key: u8) -> bool;
  fn get_last_key(&self) -> u8;
}
