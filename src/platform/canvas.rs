use crate::platform::{scancodes, Color, Platform, PlatformProvider, WindowConfig};
use crate::system::System;
use async_trait::async_trait;
use instant::Instant;
use js_sys::Math;
use pixels::{Pixels, SurfaceTexture};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use winit::{
  dpi::LogicalSize,
  event::{ElementState, Event, VirtualKeyCode, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  platform::web::WindowBuilderExtWebSys,
  window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
  fn prompt(message: &str) -> String;
  fn alert(message: &str);
}

fn virtual_key_to_ascii(code: VirtualKeyCode) -> Option<u8> {
  if (code as u8) <= 36 {
    let code = code as u8;
    Some(match code {
      0..=8 => '1' as u8 + code,
      9 => '0' as u8,
      10..=36 => 'A' as u8 + code - 10,
      _ => unreachable!(),
    })
  } else {
    match code {
      VirtualKeyCode::Space => Some(' ' as u8),
      VirtualKeyCode::Return => Some(scancodes::RETURN as u8),
      VirtualKeyCode::Back => Some(scancodes::BACKSPACE as u8),
      VirtualKeyCode::LShift => Some(scancodes::LSHIFT as u8),
      VirtualKeyCode::RShift => Some(scancodes::RSHIFT as u8),
      VirtualKeyCode::Apostrophe => Some('"' as u8), // should be ', but PET has separate keys
      _ => None,
    }
  }
}

pub struct CanvasPlatform {
  config: Arc<Mutex<Option<WindowConfig>>>,
  pixels: Arc<Mutex<Option<Pixels>>>,
  provider: Arc<CanvasPlatformProvider>,
  dirty: Arc<Mutex<bool>>,
  key_state: Arc<Mutex<[bool; 256]>>,
  last_key: Arc<Mutex<u8>>,
}

impl CanvasPlatform {
  pub fn new() -> Self {
    let config = Arc::new(Mutex::new(None));
    let pixels = Arc::new(Mutex::new(None));
    let dirty = Arc::new(Mutex::new(false));
    let key_state = Arc::new(Mutex::new([false; 256]));
    let last_key = Arc::new(Mutex::new(0));

    Self {
      provider: Arc::new(CanvasPlatformProvider::new(
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

fn run_system(mut system: System) {
  let mut duration = Duration::ZERO;

  while duration < Duration::from_millis(20) {
    duration += system.tick();
  }

  let closure = Closure::once_into_js(move || {
    run_system(system);
  });

  web_sys::window()
    .unwrap()
    .set_timeout_with_callback_and_timeout_and_arguments_0(
      closure.unchecked_ref(),
      duration.as_millis() as i32,
    )
    .unwrap();
}

#[async_trait(?Send)]
impl Platform for CanvasPlatform {
  fn run(&mut self, _system: System) {
    unimplemented!("WebAssembly cannot run synchronously, to avoid blocking the main thread");
  }

  async fn run_async(&mut self, mut system: System) {
    let event_loop = EventLoop::new();

    let config = self.get_config();

    let canvas = web_sys::window()
      .unwrap()
      .document()
      .unwrap()
      .get_element_by_id("canvas")
      .unwrap()
      .dyn_into::<web_sys::HtmlCanvasElement>()
      .unwrap();

    let window = Arc::new(
      WindowBuilder::new()
        .with_title("noentiendo")
        .with_inner_size(LogicalSize::new(
          config.width as f64 * config.scale,
          config.height as f64 * config.scale,
        ))
        .with_canvas(Some(canvas))
        .build(&event_loop)
        .unwrap(),
    );

    let inner_size = window.inner_size();

    let pixels_arc = self.pixels.clone();
    let window_arc = window.clone();
    let surface_texture =
      SurfaceTexture::new(inner_size.width, inner_size.height, window_arc.as_ref());

    let pixels = Pixels::new_async(config.width, config.height, surface_texture)
      .await
      .unwrap();

    *pixels_arc.lock().unwrap() = Some(pixels);

    let mut input = WinitInputHelper::new();

    let pixels = self.pixels.clone();
    let dirty = self.dirty.clone();
    let key_state = self.key_state.clone();
    let last_key = self.last_key.clone();

    system.reset();

    run_system(system);

    event_loop.run(move |event, _, control_flow| {
      *control_flow = ControlFlow::WaitUntil(Instant::now() + Duration::from_millis(17));

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
          if *dirty.lock().unwrap() {
            window.request_redraw();
          }
        }
        Event::RedrawRequested(_) => {
          *dirty.lock().unwrap() = false;
          pixels.lock().unwrap().as_ref().unwrap().render().unwrap();
        }
        Event::WindowEvent {
          event:
            WindowEvent::KeyboardInput {
              input:
                winit::event::KeyboardInput {
                  virtual_keycode: Some(key),
                  state,
                  ..
                },
              ..
            },
          ..
        } => match state {
          ElementState::Pressed => {
            let key = virtual_key_to_ascii(key);
            if let Some(key) = key {
              key_state.lock().unwrap()[key as usize] = true;
              *last_key.lock().unwrap() = key;
            }
          }
          ElementState::Released => {
            let key = virtual_key_to_ascii(key);
            if let Some(key) = key {
              key_state.lock().unwrap()[key as usize] = false;
            }
          }
        },
        _ => (),
      }
    });
  }

  fn provider(&self) -> Arc<dyn PlatformProvider> {
    self.provider.clone()
  }
}

pub struct CanvasPlatformProvider {
  config: Arc<Mutex<Option<WindowConfig>>>,
  pixels: Arc<Mutex<Option<Pixels>>>,
  dirty: Arc<Mutex<bool>>,
  key_state: Arc<Mutex<[bool; 256]>>,
  last_key: Arc<Mutex<u8>>,
}

impl CanvasPlatformProvider {
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

impl PlatformProvider for CanvasPlatformProvider {
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
    alert(text);
  }

  fn input(&self) -> String {
    prompt("> ")
  }

  fn random(&self) -> u8 {
    Math::floor(Math::random() * 255.0) as u8
  }
}
