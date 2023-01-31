use crate::keyboard::{KeyAdapter, KeyPosition, KeyState};
mod keyboard;
use crate::platform::{JoystickState, Platform, PlatformProvider, SyncPlatform, WindowConfig};
use crate::systems::System;
use crate::time::VariableTimeStep;
use gilrs::{Button, EventType, Gilrs};
use instant::Duration;
use keyboard::WinitAdapter;
use pixels::{Pixels, SurfaceTexture};
use rand;
use std::io::Write;
use std::sync::{Arc, Mutex};
use winit::dpi::LogicalSize;
use winit::event::{ElementState, Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

/// A platform implementation for desktop platforms using Winit and Pixels.
/// This platform runs synchronously.
pub struct WinitPlatform {
  config: Arc<Mutex<Option<WindowConfig>>>,
  provider: Arc<WinitPlatformProvider>,
  key_state: Arc<Mutex<KeyState<VirtualKeyCode>>>,
  joystick_state: Arc<Mutex<JoystickState>>,
}

impl WinitPlatform {
  pub fn new() -> Self {
    let config = Arc::new(Mutex::new(None));
    let key_state = Arc::new(Mutex::new(KeyState::new()));
    let joystick_state = Arc::new(Mutex::new(JoystickState::empty()));

    Self {
      provider: Arc::new(WinitPlatformProvider::new(
        config.clone(),
        key_state.clone(),
        joystick_state.clone(),
      )),
      config,
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

    let mut pixels =
      Pixels::new(current_config.width, current_config.height, surface_texture).unwrap();

    let mut input = WinitInputHelper::new();
    let key_state = self.key_state.clone();
    let config = self.config.clone();

    system.reset();

    let mut timer = VariableTimeStep::new(Duration::from_secs_f64(1.0 / 60.0));

    let mut gilrs = Gilrs::new().unwrap();
    let joystick_state = self.joystick_state.clone();

    event_loop.run(move |event, _, control_flow| {
      *control_flow = ControlFlow::Poll;

      if input.update(&event) {
        if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
          *control_flow = ControlFlow::Exit;
        }

        if let Some(size) = input.window_resized() {
          pixels.resize_surface(size.width, size.height).unwrap();
        }
      }

      match event {
        Event::MainEventsCleared => {
          timer.do_update(&mut || system.tick());

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

              window.set_inner_size(LogicalSize::new(
                new_config.width as f64 * new_config.scale,
                new_config.height as f64 * new_config.scale,
              ));

              let inner_size = window.inner_size();

              let surface_texture =
                SurfaceTexture::new(inner_size.width, inner_size.height, &window);

              pixels = Pixels::new(new_config.width, new_config.height, surface_texture).unwrap();
            }
          }

          window.request_redraw();

          // TODO: vsync?
        }
        Event::RedrawRequested(_) => {
          system.render(pixels.get_frame_mut(), config.lock().unwrap().unwrap());
          pixels.render().unwrap();
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
  key_state: Arc<Mutex<KeyState<VirtualKeyCode>>>,
  joystick_state: Arc<Mutex<JoystickState>>,
}

impl WinitPlatformProvider {
  pub fn new(
    config: Arc<Mutex<Option<WindowConfig>>>,
    key_state: Arc<Mutex<KeyState<VirtualKeyCode>>>,
    joystick_state: Arc<Mutex<JoystickState>>,
  ) -> Self {
    Self {
      config,
      key_state,
      joystick_state,
    }
  }
}

impl PlatformProvider for WinitPlatformProvider {
  fn request_window(&self, config: WindowConfig) {
    *self.config.lock().unwrap() = Some(config);
  }

  fn get_key_state(&self) -> KeyState<KeyPosition> {
    WinitAdapter::map(&self.key_state.lock().unwrap())
  }

  fn get_joystick_state(&self) -> JoystickState {
    *self.joystick_state.lock().unwrap()
  }

  fn print(&self, text: &str) {
    print!("{text}");
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
