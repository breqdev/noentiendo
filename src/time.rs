#[cfg(not(target_arch = "wasm32"))]
use std::time::{Duration, Instant};

#[cfg(target_arch = "wasm32")]
use js_sys::Date;

#[cfg(not(target_arch = "wasm32"))]
pub struct Rate {
  delta: Duration,
  last: Instant,
}

#[cfg(target_arch = "wasm32")]
pub struct Rate {
  delta: f64,
  last: f64,
}

impl Rate {
  #[cfg(not(target_arch = "wasm32"))]
  pub fn new(delta: f64) -> Self {
    Self {
      delta: Duration::from_secs_f64(delta),
      last: Instant::now(),
    }
  }

  #[cfg(target_arch = "wasm32")]
  pub fn new(delta: f64) -> Self {
    Self {
      delta,
      last: Date::now(),
    }
  }

  #[cfg(not(target_arch = "wasm32"))]
  pub fn tick(&mut self) -> bool {
    let now = Instant::now();
    if now - self.last >= self.delta {
      self.last = now;
      true
    } else {
      false
    }
  }

  #[cfg(target_arch = "wasm32")]
  pub fn tick(&mut self) -> bool {
    let now = Date::now();
    if now - self.last >= self.delta {
      self.last = now;
      true
    } else {
      false
    }
  }

  #[cfg(not(target_arch = "wasm32"))]
  pub fn sleep(&mut self) {
    let now = Instant::now();
    if now - self.last < self.delta {
      std::thread::sleep(self.delta - (now - self.last));
    }
    self.last = Instant::now();
  }
}
