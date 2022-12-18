use crate::platform::KeyState;
use crate::platform::{AsyncPlatform, Color, Platform, PlatformProvider, WindowConfig};
use crate::system::System;
use async_trait::async_trait;
use js_sys::Math;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent};
mod keyboard;
use crate::keyboard::{KeyAdapter, KeyPosition};
use keyboard::JavaScriptAdapter;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
  fn prompt(message: &str) -> String;
  fn alert(message: &str);
}

/// A platform implementation for the web.
/// This draws to a canvas element in the DOM and uses the web's keyboard
/// events. It ticks forward the emulator time on a specified interval.
/// This platform runs asynchronously (using the JS event loop).
pub struct CanvasPlatform {
  config: Arc<Mutex<Option<WindowConfig>>>,
  canvas: Arc<Mutex<Option<HtmlCanvasElement>>>,
  framebuffer: Arc<Mutex<Option<Vec<u8>>>>,
  provider: Arc<CanvasPlatformProvider>,
  key_state: Arc<Mutex<KeyState<String>>>,
}

impl CanvasPlatform {
  pub fn new() -> Self {
    let config = Arc::new(Mutex::new(None));
    let canvas = Arc::new(Mutex::new(None));
    let framebuffer = Arc::new(Mutex::new(None));
    let key_state = Arc::new(Mutex::new(KeyState::new()));

    Self {
      provider: Arc::new(CanvasPlatformProvider::new(
        config.clone(),
        canvas.clone(),
        framebuffer.clone(),
        key_state.clone(),
      )),
      config,
      canvas,
      framebuffer,
      key_state,
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
    duration += Duration::from_secs_f64(system.tick());
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

impl Platform for CanvasPlatform {
  fn provider(&self) -> Arc<dyn PlatformProvider> {
    self.provider.clone()
  }
}

#[async_trait(?Send)]
impl AsyncPlatform for CanvasPlatform {
  async fn run_async(&mut self, mut system: System) {
    let config = self.get_config();

    let width = (config.width as f64 * config.scale) as usize;
    let height = (config.height as f64 * config.scale) as usize;

    let canvas = web_sys::window()
      .unwrap()
      .document()
      .unwrap()
      .get_element_by_id("canvas")
      .unwrap()
      .dyn_into::<HtmlCanvasElement>()
      .unwrap();

    canvas.set_width(width as u32);
    canvas.set_height(height as u32);

    canvas
      .style()
      .set_property("width", &format!("{}px", width))
      .unwrap();
    canvas
      .style()
      .set_property("height", &format!("{}px", height))
      .unwrap();

    let context = canvas
      .get_context("2d")
      .unwrap()
      .unwrap()
      .dyn_into::<CanvasRenderingContext2d>()
      .unwrap();

    context.set_image_smoothing_enabled(false);
    context.set_fill_style(&JsValue::from_str("black"));
    context.fill_rect(0.0, 0.0, width as f64, height as f64);

    {
      let key_state = self.key_state.clone();

      let keydown = Closure::<dyn FnMut(_)>::new(move |event: KeyboardEvent| {
        event.prevent_default();
        key_state.lock().unwrap().press(event.code());
      });

      web_sys::window()
        .unwrap()
        .add_event_listener_with_callback("keydown", keydown.as_ref().unchecked_ref())
        .unwrap();
      keydown.forget();
    }

    {
      let key_state = self.key_state.clone();

      let keyup = Closure::<dyn FnMut(_)>::new(move |event: KeyboardEvent| {
        event.prevent_default();
        key_state.lock().unwrap().release(event.code())
      });

      web_sys::window()
        .unwrap()
        .add_event_listener_with_callback("keyup", keyup.as_ref().unchecked_ref())
        .unwrap();
      keyup.forget();
    }

    *self.canvas.lock().unwrap() = Some(canvas);

    let framebuffer = vec![0; width * height * 4];
    *self.framebuffer.lock().unwrap() = Some(framebuffer);

    system.reset();

    run_system(system);
  }
}

pub struct CanvasPlatformProvider {
  config: Arc<Mutex<Option<WindowConfig>>>,
  canvas: Arc<Mutex<Option<HtmlCanvasElement>>>,
  framebuffer: Arc<Mutex<Option<Vec<u8>>>>,
  key_state: Arc<Mutex<KeyState<String>>>,
}

impl CanvasPlatformProvider {
  pub fn new(
    config: Arc<Mutex<Option<WindowConfig>>>,
    canvas: Arc<Mutex<Option<HtmlCanvasElement>>>,
    framebuffer: Arc<Mutex<Option<Vec<u8>>>>,
    key_state: Arc<Mutex<KeyState<String>>>,
  ) -> Self {
    Self {
      config,
      canvas,
      framebuffer,
      key_state,
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
    let mut framebuffer = self.framebuffer.lock().unwrap();
    let buffer = framebuffer.as_mut().unwrap();
    let config = self.get_config();

    if (x >= config.width) || (y >= config.height) {
      println!(
        "Invalid pixel coordinates ({}, {}) for dimensions ({}, {})",
        x, y, config.width, config.height
      );
      return;
    }

    let index = ((y * config.width + x) * 4) as usize;
    let pixel = &mut buffer[index..(index + 4)];
    pixel.copy_from_slice(&color.to_rgba());

    let context = self
      .canvas
      .lock()
      .unwrap()
      .as_ref()
      .unwrap()
      .get_context("2d")
      .unwrap()
      .unwrap()
      .dyn_into::<CanvasRenderingContext2d>()
      .unwrap();

    context.set_fill_style(&JsValue::from_str(&format!(
      "rgb({}, {}, {})",
      color.r, color.g, color.b
    )));
    context.fill_rect(
      x as f64 * config.scale,
      y as f64 * config.scale,
      config.scale,
      config.scale,
    );
  }

  fn get_key_state(&self) -> KeyState<KeyPosition> {
    JavaScriptAdapter::map(&self.key_state.lock().unwrap())
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
