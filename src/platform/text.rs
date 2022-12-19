use crate::keyboard::{KeyPosition, KeyState};
use crate::platform::{Color, Platform, PlatformProvider, SyncPlatform, WindowConfig};
use crate::system::System;
use rand;
use std::io::Write;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

/// Represents a platform which exclusively operates over text mode,
/// without any visible graphical output. This reads from and writes to the
/// terminal.
/// This platform runs synchronously.
pub struct TextPlatform {}

impl TextPlatform {
  pub fn new() -> Self {
    Self {}
  }
}

impl Platform for TextPlatform {
  fn provider(&self) -> Arc<dyn PlatformProvider> {
    Arc::new(TextPlatformProvider::new())
  }
}

impl SyncPlatform for TextPlatform {
  fn run(&mut self, mut system: System) {
    system.reset();

    // system.registers.pc.load(0x0400); // Klaus tests

    let mut last_tick = Instant::now();
    let mut last_report = last_tick;

    loop {
      let mut duration = Duration::ZERO;
      if system.get_info().cycles_per_second > 0 {
        while duration < Duration::from_millis(16) {
          duration += Duration::from_secs_f64(system.tick());
        }
      } else {
        for _ in 0..1000 {
          system.tick();
        }
      }
      let now = Instant::now();
      let elapsed = now - last_tick;
      if elapsed < duration {
        thread::sleep(duration - elapsed);
      }
      last_tick = now;

      if now - last_report > std::time::Duration::from_secs_f64(0.1) {
        let pc = system.registers.pc.address();
        println!("Program Counter: {:02x}", pc);
        last_report = now;
      }
    }
  }
}

pub struct TextPlatformProvider {}

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
