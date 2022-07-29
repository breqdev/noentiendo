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
