use crate::keyboard::{KeyPosition, KeyState};
use crate::systems::System;
use async_trait::async_trait;
use std::sync::Arc;

#[cfg(target_arch = "wasm32")]
mod canvas;

#[cfg(not(target_arch = "wasm32"))]
mod text;

#[cfg(not(target_arch = "wasm32"))]
mod winit;

#[cfg(target_arch = "wasm32")]
pub use self::canvas::{CanvasPlatform, CanvasPlatformProvider};
#[cfg(not(target_arch = "wasm32"))]
pub use self::text::{TextPlatform, TextPlatformProvider};
#[cfg(not(target_arch = "wasm32"))]
pub use self::winit::{WinitPlatform, WinitPlatformProvider};

/// A Platform provides platform-specific functionality to the emulator.
/// It handles starting and ticking the system, and provides a PlatformProvider
/// to the system for screen/keyboard/etc. access.
pub trait Platform {
  fn provider(&self) -> Arc<dyn PlatformProvider>;
}

/// A platform which can be run synchronously.
pub trait SyncPlatform: Platform {
  fn run(&mut self, system: Box<dyn System>);
}

/// A platform which can be run asynchronously.
#[async_trait(?Send)]
pub trait AsyncPlatform: Platform {
  async fn setup(&mut self);
  async fn tick(&mut self, system: &mut Box<dyn System>);
}

/// Represents an RGB color with 8 bits per channel.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Color {
  r: u8,
  g: u8,
  b: u8,
}

impl Color {
  pub fn new(r: u8, g: u8, b: u8) -> Self {
    Self { r, g, b }
  }

  /// Convert the given color to an array of 4 bytes, where the last byte
  /// (alpha) is always 255.
  pub fn to_rgba(&self) -> [u8; 4] {
    [self.r, self.g, self.b, 255]
  }

  /// Convert the given color to a 32-bit integer, where the top 8 bits are
  /// unset (0), the next 8 bits are red, the next 8 bits are green, and the
  /// last 8 bits are blue.
  pub fn to_rgb(&self) -> u32 {
    (self.r as u32) << 16 | (self.g as u32) << 8 | self.b as u32
  }
}

/// Represents the current state of the connected joystick.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct JoystickState {
  pub up: bool,
  pub down: bool,
  pub left: bool,
  pub right: bool,
  pub fire: bool,
}

impl JoystickState {
  /// Create a new JoystickState with all buttons released.
  pub fn empty() -> Self {
    Self {
      up: false,
      down: false,
      left: false,
      right: false,
      fire: false,
    }
  }
}

/// Represents the configuration of a GUI window that the system can request
/// from the platform.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WindowConfig {
  pub width: u32,
  pub height: u32,
  pub scale: f64,
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

pub trait PlatformProvider {
  /// Request that the platform create a window of the specified size,
  /// with the specified scale factor. If a window already exists, the platform
  /// should resize it to the new size.
  fn request_window(&self, config: WindowConfig);

  /// Get the current state of the keyboard.
  fn get_key_state(&self) -> KeyState<KeyPosition>;

  /// Get the current state of the connected joystick.
  /// If no joystick is connected, this should return a default state.
  fn get_joystick_state(&self) -> JoystickState;

  /// Display the given string to the user, "out-of-band" from any other
  /// graphics. This is used for text-mode systems. Implementations may choose
  /// various ways to display this, such as a terminal message or a pop-up.
  fn print(&self, text: &str);

  /// Read a string input from the user, "out-of-band" from any other
  /// graphics. This is used for text-mode systems. Implementations may choose
  /// various ways to prompt for this, such as a terminal prompt or a pop-up
  /// dialog.
  fn input(&self) -> String;

  /// Return a random number between 0 and 255. This exists as some platforms
  /// (such as the web) have a different source of randomness.
  fn random(&self) -> u8;
}
