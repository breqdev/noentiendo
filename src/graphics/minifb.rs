use crate::graphics::{Color, GraphicsProvider, GraphicsService, WindowConfig};
use minifb::{Key, Window, WindowOptions};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

pub struct MinifbGraphicsService {
  config: Arc<Mutex<Option<WindowConfig>>>,
  pixels: Arc<Mutex<Vec<u32>>>,
  provider: Arc<MinifbGraphicsProvider>,
  ready: Arc<(Mutex<bool>, Condvar)>,
  last_key: Arc<Mutex<u8>>,
}

impl MinifbGraphicsService {
  pub fn new() -> Self {
    let config = Arc::new(Mutex::new(None));
    let pixels = Arc::new(Mutex::new(Vec::new()));
    let ready = Arc::new((Mutex::new(false), Condvar::new()));
    let last_key = Arc::new(Mutex::new(0));

    Self {
      provider: Arc::new(MinifbGraphicsProvider::new(
        config.clone(),
        pixels.clone(),
        ready.clone(),
        last_key.clone(),
      )),
      config,
      pixels,
      ready,
      last_key,
    }
  }

  fn _get_config(&self) -> WindowConfig {
    let config = self.config.lock().unwrap();
    config.clone().expect("WindowConfig not set")
  }
}

impl GraphicsService for MinifbGraphicsService {
  fn run(&mut self) {
    let config = self._get_config();

    let mut size = (
      (config.width as f64 * config.scale) as usize,
      (config.height as f64 * config.scale) as usize,
    );

    let mut window = Window::new(
      "noentiendo",
      size.0,
      size.1,
      WindowOptions {
        ..WindowOptions::default()
      },
    )
    .unwrap();

    *self.pixels.lock().unwrap() = vec![0; size.0 * size.1];

    {
      let (lock, cvar) = &*self.ready;
      let mut ready = lock.lock().unwrap();
      *ready = true;
      cvar.notify_one();
    }

    let pixels = self.pixels.clone();
    let last_key = self.last_key.clone();

    while window.is_open() && !window.is_key_down(Key::Escape) {
      let new_size = (window.get_size().0, window.get_size().1);
      if new_size != size {
        size = new_size;
        *pixels.lock().unwrap() = vec![0; size.0 * size.1];
        println!("resized to {}x{}", size.0, size.1);
      }

      // window.get_keys().iter().for_each(|key| match key {
      //   Key::W => println!("holding w!"),
      //   Key::T => println!("holding t!"),
      //   _ => (),
      // });

      // window.get_keys_released().iter().for_each(|key| match key {
      //   Key::W => println!("released w!"),
      //   Key::T => println!("released t!"),
      //   _ => (),
      // });

      thread::sleep(std::time::Duration::from_millis(100));

      window
        .update_with_buffer(pixels.lock().as_ref().unwrap(), new_size.0, new_size.1)
        .unwrap();
    }
  }

  fn provider(&self) -> Arc<dyn GraphicsProvider> {
    self.provider.clone()
  }
}

pub struct MinifbGraphicsProvider {
  config: Arc<Mutex<Option<WindowConfig>>>,
  pixels: Arc<Mutex<Vec<u32>>>,
  ready: Arc<(Mutex<bool>, Condvar)>,
  last_key: Arc<Mutex<u8>>,
}

impl MinifbGraphicsProvider {
  pub fn new(
    config: Arc<Mutex<Option<WindowConfig>>>,
    pixels: Arc<Mutex<Vec<u32>>>,
    ready: Arc<(Mutex<bool>, Condvar)>,
    last_key: Arc<Mutex<u8>>,
  ) -> Self {
    Self {
      config,
      pixels,
      ready,
      last_key,
    }
  }
  fn _get_config(&self) -> WindowConfig {
    let config = self.config.lock().unwrap();
    config.clone().expect("WindowConfig not set")
  }
}

impl GraphicsProvider for MinifbGraphicsProvider {
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
    let mut pixels = self.pixels.lock().unwrap();
    let config = self._get_config();

    if (x >= config.width) || (y >= config.height) {
      println!(
        "Invalid pixel coordinates ({}, {}) for dimensions ({}, {})",
        x, y, config.width, config.height
      );
      return;
    }

    let scaled_x = (x as f64 * config.scale) as usize;
    let scaled_y = (y as f64 * config.scale) as usize;
    let width = (config.width as f64 * config.scale) as usize;
    let pixel_size = config.scale as usize;

    let color = color.to_rgb();

    for i in 0..pixel_size {
      for j in 0..pixel_size {
        let index = (scaled_y + j) * width + (scaled_x + i);
        pixels[index] = color;
      }
    }
  }

  fn get_last_key(&self) -> u8 {
    self.last_key.lock().unwrap().clone()
  }
}
