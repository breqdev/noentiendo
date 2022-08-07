use std::time::Instant;

pub enum ActiveInterrupt {
  None,
  NMI,
  IRQ,
}

pub trait InterruptTrigger: Send {
  fn poll(&mut self) -> ActiveInterrupt;
}

pub struct NullInterruptTrigger;

impl NullInterruptTrigger {
  pub fn new() -> NullInterruptTrigger {
    NullInterruptTrigger
  }
}

impl InterruptTrigger for NullInterruptTrigger {
  fn poll(&mut self) -> ActiveInterrupt {
    ActiveInterrupt::None
  }
}

pub struct PetInterruptTrigger {
  last_interrupt: Instant,
}

impl PetInterruptTrigger {
  pub fn new() -> PetInterruptTrigger {
    PetInterruptTrigger {
      last_interrupt: Instant::now(),
    }
  }
}

impl InterruptTrigger for PetInterruptTrigger {
  fn poll(&mut self) -> ActiveInterrupt {
    let now = Instant::now();
    let delta = now - self.last_interrupt;
    if delta.as_secs_f64() > (1.0 / 10.0) {
      self.last_interrupt = now;
      ActiveInterrupt::IRQ
    } else {
      ActiveInterrupt::None
    }
  }
}
