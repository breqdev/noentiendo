use crate::keyboard::{KeyAdapter, KeyPosition, KeyState};
mod keyboard;
use crate::platform::{
  Color, JoystickState, Platform, PlatformProvider, SyncPlatform, WindowConfig,
};
use crate::systems::System;
use gilrs::{Button, EventType, Gilrs};
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
  joystick_state: Arc<Mutex<JoystickState>>,
}

impl WinitPlatform {
  pub fn new() -> Self {
    let config = Arc::new(Mutex::new(None));
    let pixels = Arc::new(Mutex::new(None));
    let dirty = Arc::new(Mutex::new(false));
    let key_state = Arc::new(Mutex::new(KeyState::new()));
    let joystick_state = Arc::new(Mutex::new(JoystickState::empty()));

    Self {
      provider: Arc::new(WinitPlatformProvider::new(
        config.clone(),
        pixels.clone(),
        dirty.clone(),
        key_state.clone(),
        joystick_state.clone(),
      )),
      config,
      pixels,
      dirty,
      key_state,
      joystick_state,
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
  fn run(&mut self, mut system: Box<dyn System>) {
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

    let mut gilrs = Gilrs::new().unwrap();
    let joystick_state = self.joystick_state.clone();

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
            .resize_surface(size.width, size.height)
            .unwrap();
        }
      }

      match event {
        Event::MainEventsCleared => {
          let now = Instant::now();

          system.tick();

          {
            let mut joystick_state = joystick_state.lock().unwrap();
            loop {
              let next_event = gilrs.next_event();

              match next_event {
                Some(event) => match event.event {
                  EventType::ButtonPressed(button, _) => match button {
                    Button::DPadLeft => joystick_state.left = true,
                    Button::DPadRight => joystick_state.right = true,
                    Button::DPadUp => joystick_state.up = true,
                    Button::DPadDown => joystick_state.down = true,
                    Button::South => joystick_state.fire = true,
                    _ => {}
                  },
                  EventType::ButtonReleased(button, _) => match button {
                    Button::DPadLeft => joystick_state.left = false,
                    Button::DPadRight => joystick_state.right = false,
                    Button::DPadUp => joystick_state.up = false,
                    Button::DPadDown => joystick_state.down = false,
                    Button::South => joystick_state.fire = false,
                    _ => {}
                  },
                  _ => {}
                },
                None => break,
              }
            }
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
  joystick_state: Arc<Mutex<JoystickState>>,
}

impl WinitPlatformProvider {
  pub fn new(
    config: Arc<Mutex<Option<WindowConfig>>>,
    pixels: Arc<Mutex<Option<Pixels>>>,
    dirty: Arc<Mutex<bool>>,
    key_state: Arc<Mutex<KeyState<VirtualKeyCode>>>,
    joystick_state: Arc<Mutex<JoystickState>>,
  ) -> Self {
    Self {
      config,
      pixels,
      dirty,
      key_state,
      joystick_state,
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
    let frame = pixels.as_mut().unwrap().get_frame_mut();
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

  fn get_joystick_state(&self) -> JoystickState {
    *self.joystick_state.lock().unwrap()
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
