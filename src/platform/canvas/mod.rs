use crate::platform::KeyState;
use crate::platform::{
  AsyncPlatform, Color, JoystickState, Platform, PlatformProvider, WindowConfig,
};
use crate::system::System;
use async_trait::async_trait;
use js_sys::Math;
mod handles;
use handles::CanvasWindow;
use pixels::{Pixels, SurfaceTexture};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlCanvasElement, KeyboardEvent};
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
  resize_requested: Arc<Mutex<bool>>,
  canvas: Arc<Mutex<Option<HtmlCanvasElement>>>,
  pixels: Arc<Mutex<Option<Pixels>>>,
  provider: Arc<CanvasPlatformProvider>,
  key_state: Arc<Mutex<KeyState<String>>>,
}

impl CanvasPlatform {
  pub fn new() -> Self {
    let config = Arc::new(Mutex::new(None));
    let resize_requested = Arc::new(Mutex::new(false));
    let canvas = Arc::new(Mutex::new(None));
    let pixels = Arc::new(Mutex::new(None));
    let key_state = Arc::new(Mutex::new(KeyState::new()));

    Self {
      provider: Arc::new(CanvasPlatformProvider::new(
        config.clone(),
        resize_requested.clone(),
        pixels.clone(),
        key_state.clone(),
      )),
      config,
      resize_requested,
      canvas,
      pixels,
      key_state,
    }
  }

  fn get_config(&self) -> WindowConfig {
    let config = self.config.lock().unwrap();
    config.clone().expect("WindowConfig not set")
  }
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

    canvas.set_attribute("data-raw-handle", "1").unwrap();
    let window = CanvasWindow::new(&canvas);

    let surface_texture = SurfaceTexture::new(
      (config.width as f64 * config.scale) as u32,
      (config.height as f64 * config.scale) as u32,
      &window,
    );

    *self.pixels.lock().unwrap() = Some(
      Pixels::new_async(config.width, config.height, surface_texture)
        .await
        .unwrap(),
    );

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

    system.reset();

    let pixels = self.pixels.clone();
    let config = self.config.clone();
    let canvas = self.canvas.clone();
    let resize_requested = self.resize_requested.clone();

    let interval = Closure::new(move || {
      let mut duration = Duration::ZERO;

      while duration < Duration::from_millis(20) {
        duration += Duration::from_secs_f64(system.tick());
      }

      if *resize_requested.lock().unwrap() {
        let config = config.lock().unwrap().unwrap();

        let width = (config.width as f64 * config.scale) as u32;
        let height = (config.height as f64 * config.scale) as u32;

        let pixels = pixels.clone();

        *resize_requested.lock().unwrap() = false;

        spawn_local(async move {
          let surface_texture = SurfaceTexture::new(width, height, &window);

          *pixels.lock().unwrap() = Some(
            Pixels::new_async(config.width, config.height, surface_texture)
              .await
              .unwrap(),
          );
        });

        let canvas_mutex = canvas.lock().unwrap();
        let canvas = canvas_mutex.as_ref().unwrap();

        canvas
          .style()
          .set_property("width", &format!("{}px", width))
          .unwrap();

        canvas
          .style()
          .set_property("height", &format!("{}px", height))
          .unwrap();

        canvas.set_width(width);
        canvas.set_height(height);
      }

      pixels.lock().unwrap().as_mut().unwrap().render().unwrap();
    });

    web_sys::window()
      .unwrap()
      .set_interval_with_callback_and_timeout_and_arguments_0(interval.as_ref().unchecked_ref(), 20)
      .unwrap();

    interval.forget();
  }
}

pub struct CanvasPlatformProvider {
  config: Arc<Mutex<Option<WindowConfig>>>,
  resize_requested: Arc<Mutex<bool>>,
  pixels: Arc<Mutex<Option<Pixels>>>,
  key_state: Arc<Mutex<KeyState<String>>>,
}

impl CanvasPlatformProvider {
  pub fn new(
    config: Arc<Mutex<Option<WindowConfig>>>,
    resize_requested: Arc<Mutex<bool>>,
    pixels: Arc<Mutex<Option<Pixels>>>,
    key_state: Arc<Mutex<KeyState<String>>>,
  ) -> Self {
    Self {
      config,
      resize_requested,
      pixels,
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
    *self.resize_requested.lock().unwrap() = true;
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
      return;
    }
    let pixel = &mut frame[index..(index + 4)];
    pixel.copy_from_slice(&color.to_rgba());
  }

  fn get_key_state(&self) -> KeyState<KeyPosition> {
    JavaScriptAdapter::map(&self.key_state.lock().unwrap())
  }

  fn get_joystick_state(&self) -> JoystickState {
    JoystickState::empty()
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
