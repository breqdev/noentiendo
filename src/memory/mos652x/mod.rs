pub mod cia;
pub mod pia;
pub mod via;

pub use cia::Cia;
pub use pia::Pia;
pub use via::Via;

use crate::memory::{Port, SystemInfo};

/// A port and its associated registers on the MOS 6522 VIA or MOS 6526 CIA.
pub struct PortRegisters {
  /// The Port implementation that this instance delegates to.
  port: Box<dyn Port>,

  /// Stores the current value written to the port.
  writes: u8,

  /// Data Direction Register. Each bit controls whether the line is an input (0) or output (1).
  ddr: u8,

  /// Latch enable: Present on the MOS 6522 VIA.
  latch_enabled: bool,
}

impl PortRegisters {
  pub fn new(port: Box<dyn Port>) -> Self {
    Self {
      port,
      writes: 0,
      ddr: 0,
      latch_enabled: false,
    }
  }

  /// Read from the port, respecting the DDR.
  pub fn read(&mut self) -> u8 {
    (self.port.read() & !self.ddr) | (self.writes & self.ddr)
  }

  /// Write to the port, respecting the DDR.
  pub fn write(&mut self, value: u8) {
    self.writes = value;
    self.port.write(value & self.ddr);
  }

  /// Poll the underlying port for interrupts.
  pub fn poll(&mut self, info: &SystemInfo) -> bool {
    self.port.poll(info)
  }

  /// Reset the port to its initial state.
  pub fn reset(&mut self) {
    self.ddr = 0;

    self.port.reset();
  }
}

/// The manner in which the timer will output signals to the port, if at all.
pub enum TimerOutput {
  /// The timer will not output to the port.
  None,

  /// The timer will output a single pulse on PB6 or PB7.
  Pulse,

  /// The timer will output a set number of pulses.
  PulseCount,

  /// The timer will toggle the output on PB6 or PB7.
  Toggle,
}

/// The source of the timer's clock, which controls the rate at which its clock decrements.
pub enum TimerClockSource {
  /// Use the internal system clock.
  Phi2,

  /// Use pulses on the external CNT pin.
  Count,

  /// Count underflows of the other timer.
  Chained,

  /// Count underflows of the other timer, but only if the CNT pin is high.
  ChainedCount,
}

/// A timer circuit on the MOS 6522 VIA or MOS 6526 CIA.
pub struct Timer {
  /// The latched value that the counter is reloaded from.
  latch: u16,

  /// The current value of the timer's internal counter.
  counter: u16,

  /// Whether the timer's interrupt flag is set.
  interrupt: bool,

  /// If false, the timer will fire once; if true, it will load the latch into the counter and keep going
  continuous: bool,

  /// Whether the timer is currently running (decrementing).
  running: bool,

  /// The manner in which the timer will output to the port.
  output: TimerOutput,

  /// The source of the timer's clock.
  clock_source: TimerClockSource,
}

impl Timer {
  pub fn new() -> Self {
    Self {
      latch: 0,
      counter: 0,
      interrupt: false,
      continuous: false,
      running: true,
      output: TimerOutput::None,
      clock_source: TimerClockSource::Phi2,
    }
  }

  /// Poll the timer (decrement the counter, fire the interrupt if necessary).
  pub fn poll(&mut self, _info: &SystemInfo) -> bool {
    if self.counter == 0 {
      if self.continuous {
        self.counter = self.latch
      } else {
        return false;
      }
    }

    if self.running {
      self.counter = self.counter.wrapping_sub(1);
    }

    if self.counter == 0 {
      self.interrupt = true;

      true
    } else {
      false
    }
  }

  /// Handle a read from the timer's data register on the MOS 6526 CIA.
  fn read_cia(&self) -> u8 {
    let clock_source = match self.clock_source {
      TimerClockSource::Phi2 => 0b00,
      TimerClockSource::Count => 0b01,
      TimerClockSource::Chained => 0b10,
      TimerClockSource::ChainedCount => 0b11,
    };

    let output = match self.output {
      TimerOutput::None => 0b00,
      TimerOutput::Pulse => 0b01,
      TimerOutput::Toggle => 0b10,
      TimerOutput::PulseCount => 0b11,
    };

    (clock_source << 4) | (!self.continuous as u8) << 3 | (output << 1) | (self.running as u8)
  }

  /// Handle a write to the timer's data register on the MOS 6526 CIA.
  fn write_cia(&mut self, value: u8) {
    self.running = (value & 0b0000_0001) != 0;
    self.continuous = !((value & 0b0000_1000) != 0);

    self.output = match value & 0b0000_0110 {
      0b0000_0000 => TimerOutput::None,
      0b0000_0010 => TimerOutput::Pulse,
      0b0000_0100 => TimerOutput::Toggle,
      0b0000_0110 => TimerOutput::PulseCount,
      _ => unreachable!(),
    };

    self.clock_source = match value & 0b0011_0000 {
      0b0000_0000 => TimerClockSource::Phi2,
      0b0001_0000 => TimerClockSource::Count,
      0b0010_0000 => TimerClockSource::Chained,
      0b0011_0000 => TimerClockSource::ChainedCount,
      _ => unreachable!(),
    };
  }

  /// Reset the timer's internal state.
  fn reset(&mut self) {
    self.latch = 0;
    self.counter = 0;
    self.interrupt = false;
    self.continuous = false;
    self.running = true;
    self.output = TimerOutput::None;
    self.clock_source = TimerClockSource::Phi2;
  }
}

/// The shift register used by the MOS 6522 VIA and MOS 6526 CIA.
pub struct ShiftRegister {
  /// The data currently in the shift register.
  data: u8,

  /// The control register used on the MOS 6522 VIA.
  control: u8,

  /// The current direction set on the MOS 6526 CIA.
  /// If 0, the shift register is in input mode; if 1, the shift register is in output mode.
  direction: bool,
}

impl ShiftRegister {
  pub fn new() -> Self {
    Self {
      data: 0,
      control: 0,
      direction: false,
    }
  }

  /// Reset the shift register's internal state.
  pub fn reset(&mut self) {
    self.data = 0;
    self.control = 0;
    self.direction = false;
  }
}

/// Registers for interrupt flags and interrupt enable bits.
/// Each bit from 0 to 6 corresponds to an interrupt source.
pub struct InterruptRegister {
  /// The current state of which interrupts are enabled.
  /// If a bit is set, the corresponding interrupt is enabled.
  pub interrupt_enable: u8,
}

impl InterruptRegister {
  /// Read the apparent value of the interrupt register, based on the provided interrupt enable bits.
  pub fn read_flags(&self, mut value: u8) -> u8 {
    if (value & self.interrupt_enable) != 0 {
      value |= 0x80;
    }

    value
  }

  /// Read the value of the interrupt enable register.
  pub fn read_enable(&self) -> u8 {
    self.interrupt_enable
  }

  /// Write to the interrupt enable register.
  pub fn write_enable(&mut self, value: u8) {
    if (value & 0x80) != 0 {
      // set bits
      self.interrupt_enable |= value & 0x7F;
    } else {
      // clear bits
      self.interrupt_enable &= !(value & 0x7F);
    }
  }

  /// Is the specified interrupt enabled?
  pub fn is_enabled(&self, interrupt: u8) -> bool {
    (self.interrupt_enable & interrupt) != 0
  }
}

impl InterruptRegister {
  fn new() -> Self {
    Self {
      interrupt_enable: 0,
    }
  }

  fn reset(&mut self) {
    self.interrupt_enable = 0;
  }
}
