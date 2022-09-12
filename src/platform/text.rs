use crate::platform::{Color, Platform, PlatformProvider, WindowConfig};
use crate::system::System;
use async_trait::async_trait;
use rand;
use std::io::Write;
use std::sync::Arc;
use std::thread;
use std::time::Instant;

pub struct TextPlatform {}

impl TextPlatform {
  pub fn new() -> Self {
    Self {}
  }
}

#[async_trait(?Send)]
impl Platform for TextPlatform {
  fn run(&mut self, mut system: System) {
    system.reset();

    system.registers.pc.load(0x0400); // Klaus tests

    let mut last_tick = Instant::now();
    let mut last_report = last_tick;

    loop {
      let duration = system.tick();
      let now = Instant::now();
      let elapsed = now - last_tick;
      if elapsed < duration {
        thread::sleep(duration - elapsed);
      }
      last_tick = now;

      if now - last_report > std::time::Duration::from_secs(1) {
        let pc = system.registers.pc.address();
        println!("Program Counter: {:02x}", pc);
        last_report = now;
      }
    }
  }

  async fn run_async(&mut self, _system: System) {
    unimplemented!()
  }

  fn provider(&self) -> Arc<dyn PlatformProvider> {
    Arc::new(TextPlatformProvider::new())
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

  fn is_pressed(&self, _key: u8) -> bool {
    false
  }

  fn get_last_key(&self) -> u8 {
    0
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
