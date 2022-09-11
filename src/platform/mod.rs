use crate::system::System;
#[cfg(target_arch = "wasm32")]
mod canvas;
pub mod scancodes;
mod text;
#[cfg(not(target_arch = "wasm32"))]
mod winit;
use async_trait::async_trait;
use std::sync::Arc;

#[cfg(target_arch = "wasm32")]
pub use self::canvas::{CanvasPlatform, CanvasPlatformProvider};
pub use self::text::{TextPlatform, TextPlatformProvider};
#[cfg(not(target_arch = "wasm32"))]
pub use self::winit::{WinitPlatform, WinitPlatformProvider};

#[async_trait(?Send)]
pub trait Platform {
  fn run(&mut self, system: System);
  async fn run_async(&mut self, system: System);
  fn provider(&self) -> Arc<dyn PlatformProvider>;
}

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

pub trait PlatformProvider: Send + Sync {
  // Window management
  fn request_window(&self, config: WindowConfig);

  // Graphics
  fn set_pixel(&self, x: u32, y: u32, color: Color);

  // Keyboard input
  fn is_pressed(&self, key: u8) -> bool;
  fn get_last_key(&self) -> u8;

  // Text I/O
  fn print(&self, text: &str);
  fn input(&self) -> String;
}
