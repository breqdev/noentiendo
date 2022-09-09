use crate::graphics::{scancodes, Color, GraphicsProvider, GraphicsService, WindowConfig};
use crate::isomorphic::sleep;
use async_trait::async_trait;
use instant::Instant;
use pixels::{Pixels, SurfaceTexture};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use wasm_bindgen_futures::spawn_local;
use winit::dpi::LogicalSize;
use winit::event::{ElementState, Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

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

pub struct CanvasGraphicsService {
  config: Arc<Mutex<Option<WindowConfig>>>,
  pixels: Arc<Mutex<Option<Pixels>>>,
  provider: Arc<CanvasGraphicsProvider>,
  ready: Arc<Mutex<bool>>,
  dirty: Arc<Mutex<bool>>,
  key_state: Arc<Mutex<[bool; 256]>>,
  last_key: Arc<Mutex<u8>>,
}

impl CanvasGraphicsService {
  pub fn new() -> Self {
    let config = Arc::new(Mutex::new(None));
    let pixels = Arc::new(Mutex::new(None));
    let ready = Arc::new(Mutex::new(false));
    let dirty = Arc::new(Mutex::new(false));
    let key_state = Arc::new(Mutex::new([false; 256]));
    let last_key = Arc::new(Mutex::new(0));

    Self {
      provider: Arc::new(CanvasGraphicsProvider::new(
        config.clone(),
        pixels.clone(),
        ready.clone(),
        dirty.clone(),
        key_state.clone(),
        last_key.clone(),
      )),
      config,
      pixels,
      ready,
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

#[async_trait(?Send)]
impl GraphicsService for CanvasGraphicsService {
  fn run(&mut self) {
    let event_loop = EventLoop::new();

    let config = self.get_config();

    let window = Arc::new(
      WindowBuilder::new()
        .with_title("noentiendo")
        .with_inner_size(LogicalSize::new(
          config.width as f64 * config.scale,
          config.height as f64 * config.scale,
        ))
        .build(&event_loop)
        .unwrap(),
    );

    let inner_size = window.inner_size();

    let pixels_arc = self.pixels.clone();
    let window_arc = window.clone();
    let ready_arc = self.ready.clone();

    spawn_local(async move {
      let surface_texture =
        SurfaceTexture::new(inner_size.width, inner_size.height, window_arc.as_ref());

      let pixels = Pixels::new_async(config.width, config.height, surface_texture)
        .await
        .unwrap();

      *pixels_arc.lock().unwrap() = Some(pixels);
      *ready_arc.lock().unwrap() = true;
    });

    #[cfg(target_arch = "wasm32")]
    {
      use winit::platform::web::WindowExtWebSys;

      // Attach winit canvas to body element
      web_sys::window()
        .and_then(|win| win.document())
        .and_then(|doc| doc.body())
        .and_then(|body| {
          body
            .append_child(&web_sys::Element::from(window.canvas()))
            .ok()
        })
        .expect("couldn't append canvas to document body");
    }

    let mut input = WinitInputHelper::new();

    let pixels = self.pixels.clone();
    let dirty = self.dirty.clone();
    let key_state = self.key_state.clone();
    let last_key = self.last_key.clone();

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

  async fn wait_for_pixels_async(&self) {
    while !*self.ready.lock().unwrap() {
      sleep(0.1).await;
    }
  }

  fn provider(&self) -> Arc<dyn GraphicsProvider> {
    self.provider.clone()
  }
}

pub struct CanvasGraphicsProvider {
  config: Arc<Mutex<Option<WindowConfig>>>,
  pixels: Arc<Mutex<Option<Pixels>>>,
  ready: Arc<Mutex<bool>>,
  dirty: Arc<Mutex<bool>>,
  key_state: Arc<Mutex<[bool; 256]>>,
  last_key: Arc<Mutex<u8>>,
}

impl CanvasGraphicsProvider {
  pub fn new(
    config: Arc<Mutex<Option<WindowConfig>>>,
    pixels: Arc<Mutex<Option<Pixels>>>,
    ready: Arc<Mutex<bool>>,
    dirty: Arc<Mutex<bool>>,
    key_state: Arc<Mutex<[bool; 256]>>,
    last_key: Arc<Mutex<u8>>,
  ) -> Self {
    Self {
      config,
      pixels,
      ready,
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

impl GraphicsProvider for CanvasGraphicsProvider {
  fn configure_window(&self, config: WindowConfig) {
    *self.config.lock().unwrap() = Some(config);
  }

  fn wait_for_pixels(&self) {
    if *self.ready.lock().unwrap() {
      return;
    }

    unimplemented!("synchronous operation not supported on wasm");
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
}
