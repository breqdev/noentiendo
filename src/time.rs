use std::thread;

use instant::{Duration, Instant};

pub fn tick_until_target(
  tick: &mut dyn FnMut() -> Duration,
  target: Duration,
  timeout: Duration,
) -> Duration {
  let realtime_now = Instant::now();
  let mut elapsed = Duration::ZERO;
  while elapsed < target {
    elapsed += tick();
    if realtime_now.elapsed() > timeout {
      break;
    }
  }
  elapsed
}

pub struct VariableTimeStep {
  previous_tick: Instant,
  timeout: Duration,
}

impl VariableTimeStep {
  pub fn new(timeout: Duration) -> Self {
    Self {
      previous_tick: Instant::now(),
      timeout,
    }
  }

  pub fn next_update_interval(&mut self) -> Duration {
    let now = Instant::now();
    let elapsed = now - self.previous_tick;
    self.previous_tick = now;
    elapsed
  }

  pub fn do_update(&mut self, tick: &mut dyn FnMut() -> Duration) {
    tick_until_target(tick, self.next_update_interval(), self.timeout);
  }
}

pub struct FixedTimeStep {
  target_interval: Duration,
  timeout: Duration,
}

impl FixedTimeStep {
  pub fn new(framerate: f64, timeout: Duration) -> Self {
    Self {
      target_interval: Duration::from_secs_f64(1.0 / framerate),
      timeout,
    }
  }

  pub fn do_update(&mut self, tick: &mut dyn FnMut() -> Duration) {
    let now = Instant::now();
    tick_until_target(tick, self.target_interval, self.timeout);
    let elapsed = now.elapsed();

    if elapsed < self.target_interval {
      thread::sleep(self.target_interval - elapsed);
    }
  }
}
