use std::thread;

use instant::{Duration, Instant};

pub fn tick_until_target(tick: &mut dyn FnMut() -> Duration, target: Duration) -> Duration {
  let mut elapsed = Duration::ZERO;
  while elapsed < target {
    elapsed += tick();
  }
  elapsed
}

pub struct VariableTimeStep {
  previous_tick: Instant,
}

impl VariableTimeStep {
  pub fn new() -> Self {
    Self {
      previous_tick: Instant::now(),
    }
  }

  pub fn next_update_interval(&mut self) -> Duration {
    let now = Instant::now();
    let elapsed = now - self.previous_tick;
    self.previous_tick = now;
    elapsed
  }

  pub fn do_update(&mut self, tick: &mut dyn FnMut() -> Duration) {
    tick_until_target(tick, self.next_update_interval());
  }
}

pub struct FixedTimeStep {
  target_interval: Duration,
}

impl FixedTimeStep {
  pub fn new(framerate: f64) -> Self {
    Self {
      target_interval: Duration::from_secs_f64(1.0 / framerate),
    }
  }

  pub fn do_update(&mut self, tick: &mut dyn FnMut() -> Duration) {
    let now = Instant::now();
    tick_until_target(tick, self.target_interval);
    let elapsed = now.elapsed();

    if elapsed < self.target_interval {
      thread::sleep(self.target_interval - elapsed);
    }
  }
}
