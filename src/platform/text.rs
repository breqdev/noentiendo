use crate::keyboard::{KeyPosition, KeyState};
use crate::platform::{Color, Platform, PlatformProvider, SyncPlatform, WindowConfig};
use crate::systems::System;
use crate::time::FixedTimeStep;
use rand;
use std::io::Write;
use std::sync::Arc;

use super::JoystickState;

/// Represents a platform which exclusively operates over text mode,
/// without any visible graphical output. This reads from and writes to the
/// terminal.
/// This platform runs synchronously.
pub struct TextPlatform {
  report: bool, // TODO: log program counter + cycle count
}

impl TextPlatform {
  pub fn new(report: bool) -> Self {
    Self { report }
  }
}

impl Platform for TextPlatform {
  fn provider(&self) -> Arc<dyn PlatformProvider> {
    Arc::new(TextPlatformProvider::new())
  }
}

impl SyncPlatform for TextPlatform {
  fn run(&mut self, mut system: Box<dyn System>) {
    system.reset();

    // system.registers.pc.load(0x0400); // Klaus tests

    let mut timer = FixedTimeStep::new(60.0);

    loop {
      timer.do_update(&mut || system.tick());
    }
  }
}

pub struct TextPlatformProvider;

impl TextPlatformProvider {
  pub fn new() -> Self {
    Self {}
  }
}

impl PlatformProvider for TextPlatformProvider {
  fn request_window(&self, _config: WindowConfig) {}

  fn set_pixel(&self, _x: u32, _y: u32, _color: Color) {}

  fn get_key_state(&self) -> KeyState<KeyPosition> {
    KeyState::new()
  }

  fn get_joystick_state(&self) -> JoystickState {
    JoystickState::empty()
  }

  fn print(&self, text: &str) {
    print!("{}", text);
  }

  fn input(&self) -> String {
    let mut input = String::new();
    print!("> ");
    std::io::stdout().flush().unwrap();
    std::io::stdin()
      .read_line(&mut input)
      .expect("Failed to read line");
    input
  }

  fn random(&self) -> u8 {
    rand::random()
  }
}
