use crate::keyboard::{KeyAdapter, KeyPosition, KeyState};
mod keyboard;
use crate::platform::{Color, Platform, PlatformProvider, SyncPlatform, WindowConfig};
use crate::system::System;
use instant::Instant;
use keyboard::WinitAdapter;
use pixels::{Pixels, SurfaceTexture};
use rand;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use winit::dpi::LogicalSize;
use winit::event::{ElementState, Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

/// A platform implementation for desktop platforms using Winit and Pixels.
/// This platform runs synchronously.
pub struct WinitPlatform {
  config: Arc<Mutex<Option<WindowConfig>>>,
  pixels: Arc<Mutex<Option<Pixels>>>,
  provider: Arc<WinitPlatformProvider>,
  dirty: Arc<Mutex<bool>>,
  key_state: Arc<Mutex<KeyState<VirtualKeyCode>>>,
}

impl WinitPlatform {
  pub fn new() -> Self {
    let config = Arc::new(Mutex::new(None));
    let pixels = Arc::new(Mutex::new(None));
    let dirty = Arc::new(Mutex::new(false));
    let key_state = Arc::new(Mutex::new(KeyState::new()));

    Self {
      provider: Arc::new(WinitPlatformProvider::new(
        config.clone(),
        pixels.clone(),
        dirty.clone(),
        key_state.clone(),
      )),
      config,
      pixels,
      dirty,
      key_state,
    }
  }

  fn get_config(&self) -> WindowConfig {
    let config = self.config.lock().unwrap();
    config.expect("WindowConfig not set")
  }
}

impl Platform for WinitPlatform {
  fn provider(&self) -> Arc<dyn PlatformProvider> {
    self.provider.clone()
  }
}

impl SyncPlatform for WinitPlatform {
  fn run(&mut self, mut system: System) {
    let event_loop = EventLoop::new();

    let mut current_config = self.get_config();

    let window = WindowBuilder::new()
      .with_title("noentiendo")
      .with_inner_size(LogicalSize::new(
        current_config.width as f64 * current_config.scale,
        current_config.height as f64 * current_config.scale,
      ))
      .build(&event_loop)
      .unwrap();

    let inner_size = window.inner_size();

    let surface_texture = SurfaceTexture::new(inner_size.width, inner_size.height, &window);

    *self.pixels.lock().unwrap() =
      Some(Pixels::new(current_config.width, current_config.height, surface_texture).unwrap());

    let mut input = WinitInputHelper::new();
    let pixels = self.pixels.clone();
    let dirty = self.dirty.clone();
    let key_state = self.key_state.clone();
    let config = self.config.clone();

    system.reset();

    let start = Instant::now();
    let mut last_tick = start;
    let mut last_report = start;
    let mut outstanding_ticks = 0.0;

    event_loop.run(move |event, _, control_flow| {
      *control_flow = ControlFlow::Poll;

      if input.update(&event) {
        if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
          *control_flow = ControlFlow::Exit;
        }

        if let Some(size) = input.window_resized() {
          pixels
            .lock()
            .unwrap()
            .as_mut()
            .unwrap()
            .resize_surface(size.width, size.height);
        }
      }

      match event {
        Event::MainEventsCleared => {
          let now = Instant::now();

          outstanding_ticks += (now - last_tick).as_secs_f64();
          let mut frame_ticks = 0.0;

          while outstanding_ticks > 0.0 && frame_ticks < Duration::from_millis(30).as_secs_f64() {
            let ticks = system.tick();
            outstanding_ticks -= ticks;
            frame_ticks += ticks;
          }

          last_tick = now;

          if now - last_report > Duration::from_secs(1) {
            println!(
              "cycles per second: {}",
              system.get_info().cycle_count as f64 / (now - start).as_secs_f64()
            );
            last_report = now;
          }

          {
            let new_config = config.lock().unwrap().unwrap();

            if new_config != current_config {
              current_config = new_config;
              *dirty.lock().unwrap() = true;

              window.set_inner_size(LogicalSize::new(
                new_config.width as f64 * new_config.scale,
                new_config.height as f64 * new_config.scale,
              ));

              let inner_size = window.inner_size();

              let surface_texture =
                SurfaceTexture::new(inner_size.width, inner_size.height, &window);

              *pixels.lock().unwrap() =
                Some(Pixels::new(new_config.width, new_config.height, surface_texture).unwrap());
            }
          }

          if *dirty.lock().unwrap() {
            window.request_redraw();
          }

          *control_flow = ControlFlow::WaitUntil(now + Duration::from_millis(17));
        }
        Event::RedrawRequested(_) => {
          *dirty.lock().unwrap() = false;
          pixels.lock().unwrap().as_ref().unwrap().render().unwrap();
        }
        Event::WindowEvent { event, .. } => match event {
          WindowEvent::KeyboardInput {
            input:
              winit::event::KeyboardInput {
                virtual_keycode: Some(key),
                state,
                ..
              },
            ..
          } => match state {
            ElementState::Pressed => {
              key_state.lock().unwrap().press(key);
            }
            ElementState::Released => {
              key_state.lock().unwrap().release(key);
            }
          },
          WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
          _ => (),
        },
        _ => (),
      }
    });
  }
}

pub struct WinitPlatformProvider {
  config: Arc<Mutex<Option<WindowConfig>>>,
  pixels: Arc<Mutex<Option<Pixels>>>,
  dirty: Arc<Mutex<bool>>,
  key_state: Arc<Mutex<KeyState<VirtualKeyCode>>>,
}

impl WinitPlatformProvider {
  pub fn new(
    config: Arc<Mutex<Option<WindowConfig>>>,
    pixels: Arc<Mutex<Option<Pixels>>>,
    dirty: Arc<Mutex<bool>>,
    key_state: Arc<Mutex<KeyState<VirtualKeyCode>>>,
  ) -> Self {
    Self {
      config,
      pixels,
      dirty,
      key_state,
    }
  }
  fn get_config(&self) -> WindowConfig {
    let config = self.config.lock().unwrap();
    config.expect("WindowConfig not set")
  }
}

impl PlatformProvider for WinitPlatformProvider {
  fn request_window(&self, config: WindowConfig) {
    *self.config.lock().unwrap() = Some(config);
  }

  fn set_pixel(&self, x: u32, y: u32, color: Color) {
    let mut pixels = self.pixels.lock().unwrap();
    let frame = pixels.as_mut().unwrap().get_frame();
    let config = self.get_config();

    if (x >= config.width) || (y >= config.height) {
      println!(
        "Invalid pixel coordinates ({}, {}) for dimensions ({}, {})",
        x, y, config.width, config.height
      );
      return;
    }

    let index = ((y * config.width + x) * 4) as usize;
    if index + 4 > frame.len() {
      // Race condition: the app has just requested a new window size, but the
      // framebuffer hasn't been resized yet
      *self.dirty.lock().unwrap() = true;
      return;
    }
    let pixel = &mut frame[index..(index + 4)];
    pixel.copy_from_slice(&color.to_rgba());
    *self.dirty.lock().unwrap() = true;
  }

  fn get_key_state(&self) -> KeyState<KeyPosition> {
    WinitAdapter::map(&self.key_state.lock().unwrap())
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
