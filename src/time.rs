#[cfg(feature = "desktop")]
use std::time::{Duration, Instant};

#[cfg(feature = "web")]
use js_sys::Date;

#[cfg(feature = "desktop")]
pub struct Rate {
  delta: Duration,
  last: Instant,
}

#[cfg(feature = "web")]
pub struct Rate {
  delta: f64,
  last: f64,
}

#[cfg(all(not(feature = "desktop"), not(feature = "web")))]
pub struct Rate {
}

impl Rate {
  #[cfg(feature = "desktop")]
  pub fn new(delta: f64) -> Self {
    Self {
      delta: Duration::from_secs_f64(delta),
      last: Instant::now(),
    }
  }

  #[cfg(feature = "web")]
  pub fn new(delta: f64) -> Self {
    Self {
      delta,
      last: Date::now(),
    }
  }

  #[cfg(all(not(feature = "desktop"), not(feature = "web")))]
  pub fn new(delta: f64) -> Self {
    unimplemented!("not implemented on this platform")
  }

  #[cfg(feature = "desktop")]
  pub fn tick(&mut self) -> bool {
    let now = Instant::now();
    if now - self.last >= self.delta {
      self.last = now;
      true
    } else {
      false
    }
  }

  #[cfg(feature = "web")]
  pub fn tick(&mut self) -> bool {
    let now = Date::now();
    if now - self.last >= self.delta {
      self.last = now;
      true
    } else {
      false
    }
  }

  #[cfg(feature = "desktop")]
  pub fn sleep(&mut self) {
    let now = Instant::now();
    if now - self.last < self.delta {
      std::thread::sleep(self.delta - (now - self.last));
    }
    self.last = Instant::now();
  }
}
