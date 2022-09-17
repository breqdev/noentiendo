use crate::platform::{scancodes, Color, Platform, PlatformProvider, SyncPlatform, WindowConfig};
use crate::system::System;
use instant::Instant;
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

fn virtual_key_to_ascii(code: VirtualKeyCode, shift: bool) -> Option<u8> {
  if (code as u8) <= 36 {
    let code = code as u8;
    if code >= 10 {
      Some('A' as u8 + code - 10)
    } else {
      if shift {
        match code {
          0 => Some('!' as u8),
          1 => Some('@' as u8),
          2 => Some('#' as u8),
          3 => Some('$' as u8),
          4 => Some('%' as u8),
          5 => Some('^' as u8),
          6 => Some('&' as u8),
          7 => Some('*' as u8),
          8 => Some('(' as u8),
          9 => Some(')' as u8),
          _ => None,
        }
      } else {
        if code == 9 {
          Some('0' as u8)
        } else {
          Some('1' as u8 + code)
        }
      }
    }
  } else {
    match code {
      VirtualKeyCode::Space => Some(' ' as u8),
      VirtualKeyCode::Return => Some(scancodes::RETURN as u8),
      VirtualKeyCode::Back => Some(scancodes::BACKSPACE as u8),

      // remap shift to windows for now
      // TODO: this should be patched at the system level for the PET
      VirtualKeyCode::LWin => Some(scancodes::LSHIFT as u8),
      VirtualKeyCode::RWin => Some(scancodes::RSHIFT as u8),
      _ => {
        if !shift {
          match code {
            VirtualKeyCode::Grave => Some('`' as u8),
            VirtualKeyCode::Minus => Some('-' as u8),
            VirtualKeyCode::Equals => Some('=' as u8),
            VirtualKeyCode::LBracket => Some('[' as u8),
            VirtualKeyCode::RBracket => Some(']' as u8),
            VirtualKeyCode::Backslash => Some('\\' as u8),
            VirtualKeyCode::Semicolon => Some(';' as u8),
            VirtualKeyCode::Apostrophe => Some('\'' as u8),
            VirtualKeyCode::Comma => Some(',' as u8),
            VirtualKeyCode::Period => Some('.' as u8),
            VirtualKeyCode::Slash => Some('/' as u8),
            _ => None,
          }
        } else {
          match code {
            VirtualKeyCode::Grave => Some('~' as u8),
            VirtualKeyCode::Minus => Some('_' as u8),
            VirtualKeyCode::Equals => Some('+' as u8),
            VirtualKeyCode::LBracket => Some('{' as u8),
            VirtualKeyCode::RBracket => Some('}' as u8),
            VirtualKeyCode::Backslash => Some('|' as u8),
            VirtualKeyCode::Semicolon => Some(':' as u8),
            VirtualKeyCode::Apostrophe => Some('"' as u8),
            VirtualKeyCode::Comma => Some('<' as u8),
            VirtualKeyCode::Period => Some('>' as u8),
            VirtualKeyCode::Slash => Some('?' as u8),
            _ => None,
          }
        }
      }
    }
  }
}

pub struct WinitPlatform {
  config: Arc<Mutex<Option<WindowConfig>>>,
  pixels: Arc<Mutex<Option<Pixels>>>,
  provider: Arc<WinitPlatformProvider>,
  dirty: Arc<Mutex<bool>>,
  key_state: Arc<Mutex<[bool; 256]>>,
  last_key: Arc<Mutex<u8>>,
}

impl WinitPlatform {
  pub fn new() -> Self {
    let config = Arc::new(Mutex::new(None));
    let pixels = Arc::new(Mutex::new(None));
    let dirty = Arc::new(Mutex::new(false));
    let key_state = Arc::new(Mutex::new([false; 256]));
    let last_key = Arc::new(Mutex::new(0));

    Self {
      provider: Arc::new(WinitPlatformProvider::new(
        config.clone(),
        pixels.clone(),
        dirty.clone(),
        key_state.clone(),
        last_key.clone(),
      )),
      config,
      pixels,
      dirty,
      key_state,
      last_key,
    }
  }

  fn get_config(&self) -> WindowConfig {
    let config = self.config.lock().unwrap();
    config.clone().expect("WindowConfig not set")
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

    let config = self.get_config();

    let window = WindowBuilder::new()
      .with_title("noentiendo")
      .with_inner_size(LogicalSize::new(
        config.width as f64 * config.scale,
        config.height as f64 * config.scale,
      ))
      .build(&event_loop)
      .unwrap();

    let inner_size = window.inner_size();

    let surface_texture = SurfaceTexture::new(inner_size.width, inner_size.height, &window);

    *self.pixels.lock().unwrap() =
      Some(Pixels::new(config.width, config.height, surface_texture).unwrap());

    let mut input = WinitInputHelper::new();
    let pixels = self.pixels.clone();
    let dirty = self.dirty.clone();
    let key_state = self.key_state.clone();
    let last_key = self.last_key.clone();

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
                modifiers,
                ..
              },
            ..
          } => match state {
            ElementState::Pressed => {
              let key = virtual_key_to_ascii(key, modifiers.shift());
              if let Some(key) = key {
                key_state.lock().unwrap()[key as usize] = true;
                *last_key.lock().unwrap() = key;
              }
            }
            ElementState::Released => {
              let key = virtual_key_to_ascii(key, modifiers.shift());
              if let Some(key) = key {
                key_state.lock().unwrap()[key as usize] = false;
              }
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
  key_state: Arc<Mutex<[bool; 256]>>,
  last_key: Arc<Mutex<u8>>,
}

impl WinitPlatformProvider {
  pub fn new(
    config: Arc<Mutex<Option<WindowConfig>>>,
    pixels: Arc<Mutex<Option<Pixels>>>,
    dirty: Arc<Mutex<bool>>,
    key_state: Arc<Mutex<[bool; 256]>>,
    last_key: Arc<Mutex<u8>>,
  ) -> Self {
    Self {
      config,
      pixels,
      dirty,
      key_state,
      last_key,
    }
  }
  fn get_config(&self) -> WindowConfig {
    let config = self.config.lock().unwrap();
    config.clone().expect("WindowConfig not set")
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
    let pixel = &mut frame[index..(index + 4)];
    pixel.copy_from_slice(&color.to_rgba());
    *self.dirty.lock().unwrap() = true;
  }

  fn is_pressed(&self, key: u8) -> bool {
    self.key_state.lock().unwrap()[key as usize]
  }

  fn get_last_key(&self) -> u8 {
    self.last_key.lock().unwrap().clone()
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
