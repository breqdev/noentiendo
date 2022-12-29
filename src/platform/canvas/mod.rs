use crate::platform::KeyState;
use crate::platform::{AsyncPlatform, JoystickState, Platform, PlatformProvider, WindowConfig};
use crate::systems::System;
use async_trait::async_trait;
use js_sys::Math;
mod handles;
use handles::CanvasWindow;
use pixels::{Pixels, SurfaceTexture};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{Gamepad, GamepadButton, HtmlCanvasElement, KeyboardEvent};
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
  provider: Arc<CanvasPlatformProvider>,
  key_state: Arc<Mutex<KeyState<String>>>,
  joystick_state: Arc<Mutex<JoystickState>>,
}

impl CanvasPlatform {
  pub fn new() -> Self {
    let config = Arc::new(Mutex::new(None));
    let resize_requested = Arc::new(Mutex::new(false));
    let key_state = Arc::new(Mutex::new(KeyState::new()));
    let joystick_state = Arc::new(Mutex::new(JoystickState::empty()));

    Self {
      provider: Arc::new(CanvasPlatformProvider::new(
        config.clone(),
        resize_requested.clone(),
        key_state.clone(),
        joystick_state.clone(),
      )),
      config,
      resize_requested,
      key_state,
      joystick_state,
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
  async fn run_async(&mut self, mut system: Box<dyn System>) {
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

    let pixels = Rc::new(RefCell::new(
      Pixels::new_async(config.width, config.height, surface_texture)
        .await
        .unwrap(),
    ));

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

    system.reset();

    let config = self.config.clone();
    let resize_requested = self.resize_requested.clone();
    let joystick_state = self.joystick_state.clone();

    let interval = Closure::new(move || {
      let mut duration = Duration::ZERO;

      while duration < Duration::from_millis(20) {
        duration += system.tick();
      }

      {
        system.render(
          pixels.borrow_mut().get_frame_mut(),
          config.lock().unwrap().unwrap(),
        );
      }

      let gamepads = web_sys::window().unwrap().navigator().get_gamepads();

      if let Ok(gamepads) = gamepads {
        let first = gamepads.iter().find(|gamepad| gamepad.is_truthy());

        if let Some(gamepad) = first {
          let gamepad = gamepad.dyn_into::<Gamepad>().unwrap();

          let mut joystick_state = joystick_state.lock().unwrap();
          joystick_state.up = gamepad
            .buttons()
            .get(12)
            .dyn_into::<GamepadButton>()
            .map_or(false, |button| button.pressed());

          joystick_state.down = gamepad
            .buttons()
            .get(13)
            .dyn_into::<GamepadButton>()
            .map_or(false, |button| button.pressed());

          joystick_state.left = gamepad
            .buttons()
            .get(14)
            .dyn_into::<GamepadButton>()
            .map_or(false, |button| button.pressed());

          joystick_state.right = gamepad
            .buttons()
            .get(15)
            .dyn_into::<GamepadButton>()
            .map_or(false, |button| button.pressed());

          joystick_state.fire = gamepad
            .buttons()
            .get(0)
            .dyn_into::<GamepadButton>()
            .map_or(false, |button| button.pressed());
        }
      }

      if *resize_requested.lock().unwrap() {
        let config = config.lock().unwrap().unwrap();

        let width = (config.width as f64 * config.scale) as u32;
        let height = (config.height as f64 * config.scale) as u32;

        let pixels = pixels.clone();

        *resize_requested.lock().unwrap() = false;

        spawn_local(async move {
          let surface_texture = SurfaceTexture::new(width, height, &window);

          *pixels.borrow_mut() = Pixels::new_async(config.width, config.height, surface_texture)
            .await
            .unwrap();
        });

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

      pixels.borrow_mut().render().unwrap();
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
  key_state: Arc<Mutex<KeyState<String>>>,
  joystick_state: Arc<Mutex<JoystickState>>,
}

impl CanvasPlatformProvider {
  pub fn new(
    config: Arc<Mutex<Option<WindowConfig>>>,
    resize_requested: Arc<Mutex<bool>>,
    key_state: Arc<Mutex<KeyState<String>>>,
    joystick_state: Arc<Mutex<JoystickState>>,
  ) -> Self {
    Self {
      config,
      resize_requested,
      key_state,
      joystick_state,
    }
  }
}

impl PlatformProvider for CanvasPlatformProvider {
  fn request_window(&self, config: WindowConfig) {
    *self.config.lock().unwrap() = Some(config);
    *self.resize_requested.lock().unwrap() = true;
  }

  fn get_key_state(&self) -> KeyState<KeyPosition> {
    JavaScriptAdapter::map(&self.key_state.lock().unwrap())
  }

  fn get_joystick_state(&self) -> JoystickState {
    *self.joystick_state.lock().unwrap()
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
