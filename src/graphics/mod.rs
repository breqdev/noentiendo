mod winit;

pub use self::winit::{WinitGraphicsProvider, WinitGraphicsService};
use std::sync::{Arc, Mutex};

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
  fn provider(&self) -> Arc<WinitGraphicsProvider>;
}

pub trait GraphicsProvider: Send + Sync {
  // Initialization
  fn configure_window(&self, config: WindowConfig);
  fn wait_for_pixels(&self);

  // Emulator Thread
  fn tick(&self);
  fn set_pixel(&self, x: u32, y: u32, color: Color);
  fn get_last_key(&self) -> u8;
}
