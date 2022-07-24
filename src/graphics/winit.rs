use crate::graphics::{Color, GraphicsProvider, GraphicsService, WindowConfig};
use pixels::{Pixels, SurfaceTexture};
use std::sync::{Arc, Condvar, Mutex};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};
use winit_input_helper::WinitInputHelper;

pub struct WinitGraphicsService {
  config: Arc<Mutex<Option<WindowConfig>>>,
  pixels: Arc<Mutex<Option<Pixels>>>,
  provider: Arc<WinitGraphicsProvider>,
  ready: Arc<(Mutex<bool>, Condvar)>,
  dirty: Arc<Mutex<bool>>,
  last_key: Arc<Mutex<u8>>,
}

impl WinitGraphicsService {
  pub fn new() -> Self {
    let config = Arc::new(Mutex::new(None));
    let pixels = Arc::new(Mutex::new(None));
    let ready = Arc::new((Mutex::new(false), Condvar::new()));
    let dirty = Arc::new(Mutex::new(false));
    let last_key = Arc::new(Mutex::new(0));

    Self {
      provider: Arc::new(WinitGraphicsProvider::new(
        config.clone(),
        pixels.clone(),
        ready.clone(),
        dirty.clone(),
        last_key.clone(),
      )),
      config,
      pixels,
      ready,
      dirty,
      last_key,
    }
  }

  fn _get_config(&self) -> WindowConfig {
    let config = self.config.lock().unwrap();
    config.clone().expect("WindowConfig not set")
  }
}

impl GraphicsService for WinitGraphicsService {
  fn run(&mut self) {
    let event_loop = EventLoop::new();

    let config = self._get_config();

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

    {
      let (lock, cvar) = &*self.ready;
      let mut ready = lock.lock().unwrap();
      *ready = true;
      cvar.notify_one();
    }

    let mut input = WinitInputHelper::new();

    let pixels = self.pixels.clone();
    let dirty = self.dirty.clone();
    let last_key = self.last_key.clone();

    event_loop.run(move |event, _, control_flow| {
      *control_flow = ControlFlow::Poll;

      if input.update(&event) {
        if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
          panic!("Quit");
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
          // Application update code.

          // Queue a RedrawRequested event.
          //
          // You only need to call this if you've determined that you need to redraw, in
          // applications which do not always need to. Applications that redraw continuously
          // can just render here instead.
          if *dirty.lock().unwrap() {
            window.request_redraw();
            *dirty.lock().unwrap() = false;
          }
        }
        Event::RedrawRequested(_) => {
          // Redraw the application.
          //
          // It's preferable for applications that do not render continuously to render in
          // this event rather than in MainEventsCleared, since rendering in here allows
          // the program to gracefully handle redraws requested by the OS.
          pixels.lock().unwrap().as_mut().unwrap().render().unwrap();
        }
        Event::WindowEvent { event, .. } => match event {
          WindowEvent::KeyboardInput { input, .. } => {
            if let Some(key) = input.virtual_keycode {
              *last_key.lock().unwrap() = key as u8;
            }
          }
          _ => {}
        },
        _ => (),
      }
    });
  }

  fn provider(&self) -> Arc<WinitGraphicsProvider> {
    self.provider.clone()
  }
}

pub struct WinitGraphicsProvider {
  config: Arc<Mutex<Option<WindowConfig>>>,
  pixels: Arc<Mutex<Option<Pixels>>>,
  ready: Arc<(Mutex<bool>, Condvar)>,
  dirty: Arc<Mutex<bool>>,
  last_key: Arc<Mutex<u8>>,
}

impl WinitGraphicsProvider {
  pub fn new(
    config: Arc<Mutex<Option<WindowConfig>>>,
    pixels: Arc<Mutex<Option<Pixels>>>,
    ready: Arc<(Mutex<bool>, Condvar)>,
    dirty: Arc<Mutex<bool>>,
    last_key: Arc<Mutex<u8>>,
  ) -> Self {
    Self {
      config,
      pixels,
      ready,
      dirty,
      last_key,
    }
  }
  fn _get_config(&self) -> WindowConfig {
    let config = self.config.lock().unwrap();
    config.clone().expect("WindowConfig not set")
  }
}

impl GraphicsProvider for WinitGraphicsProvider {
  fn configure_window(&self, config: WindowConfig) {
    *self.config.lock().unwrap() = Some(config);
  }

  fn wait_for_pixels(&self) {
    let (mutex, condvar) = &*self.ready;
    let mut ready = mutex.lock().unwrap();
    while !*ready {
      ready = condvar.wait(ready).unwrap();
    }
  }

  fn set_pixel(&self, x: u32, y: u32, color: Color) {
    self.wait_for_pixels();

    let mut pixels = self.pixels.lock().unwrap();
    let frame = pixels.as_mut().unwrap().get_frame();
    let config = self._get_config();

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

  fn get_last_key(&self) -> u8 {
    self.last_key.lock().unwrap().clone()
  }
}
