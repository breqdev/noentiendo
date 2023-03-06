use crate::memory::{
  mos::{ControlLines, ControlLinesPort, InterruptRegister, ShiftRegister, Timer, TimerOutput},
  ActiveInterrupt, Memory, SystemInfo,
};

pub struct ControlLineInterrupts {
  /// The previous state of the control lines.
  previous: ControlLines,

  /// The interrupt flags corresponding to the control lines.
  flags: ControlLines,
}

impl ControlLineInterrupts {
  pub fn new() -> Self {
    Self {
      previous: ControlLines::new(),
      flags: ControlLines::new(),
    }
  }

  pub fn poll(&mut self, pcr: u8, control: ControlLines) -> ControlLines {
    let mut interrupt = ControlLines::new();

    if control.c1 != self.previous.c1 {
      // CB1 edge
      if (control.c1 && pcr & pcr_bits::CA1_ACTIVE_TRANSITION != 0)
        || (!control.c1 && pcr & pcr_bits::CA1_ACTIVE_TRANSITION == 0)
      {
        self.flags.c1 = true;

        interrupt.c1 = true;
      }
    }

    if control.c2 != self.previous.c2 && pcr & pcr_bits::CA2_DIRECTION == 0 {
      // CB2 edge, input
      if (control.c2 && pcr & pcr_bits::CA2_ACTIVE_TRANSITION != 0)
        || (!control.c2 && pcr & pcr_bits::CB2_ACTIVE_TRANSITION == 0)
      {
        self.flags.c2 = true;

        interrupt.c2 = true;
      }
    }

    self.previous = control;

    interrupt
  }

  pub fn clear_flags(&mut self, pcr: u8) {
    self.flags.c1 = false;

    if pcr & pcr_bits::CA2_CLEAR_FLAG_ON_READ == 0 {
      self.flags.c2 = false;
    }
  }
}

/// A port and its associated registers on the MOS 6522 VIA.
pub struct PortRegisters {
  /// The Port implementation that this instance delegates to.
  port: Box<dyn ControlLinesPort>,

  /// Stores the current value written to the port.
  writes: u8,

  /// Data Direction Register. Each bit controls whether the line is an input (0) or output (1).
  ddr: u8,

  /// Latch enable.
  latch_enabled: bool,
}

impl PortRegisters {
  pub fn new(port: Box<dyn ControlLinesPort>) -> Self {
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
  pub fn poll(&mut self, cycles: u32, info: &SystemInfo) -> ControlLines {
    self.port.poll(cycles, info)
  }

  /// Reset the port to its initial state.
  pub fn reset(&mut self) {
    self.ddr = 0;

    self.port.reset();
  }
}

#[allow(dead_code)]
pub mod sr_control_bits {
  pub const SHIFT_DISABLED: u8 = 0b000;
  pub const SHIFT_IN_BY_T2: u8 = 0b001;
  pub const SHIFT_IN_BY_SYSTEM_CLOCK: u8 = 0b010;
  pub const SHIFT_IN_BY_EXTERNAL_CLOCK: u8 = 0b011; // PB6?

  pub const SHIFT_OUT_FREE_RUN: u8 = 0b100; // runs by T2, but disables the counter to run forever
  pub const SHIFT_OUT_BY_T2: u8 = 0b101;
  pub const SHIFT_OUT_BY_SYSTEM_CLOCK: u8 = 0b110;
  pub const SHIFT_OUT_BY_EXTERNAL_CLOCK: u8 = 0b111; // PB6?
}

/// The MOS 6522 Versatile Interface Adapter (VIA). Contains two ports,
/// two timers, a shift register, and some interrupt and control registers.
/// Source: <http://archive.6502.org/datasheets/mos_6522_preliminary_nov_1977.pdf>
pub struct Via {
  a: PortRegisters,
  b: PortRegisters,
  t1: Timer,
  t2: Timer,
  sr: ShiftRegister,
  interrupts: InterruptRegister,
  pcr: u8, // peripheral control register

  // Control lines
  ca: ControlLineInterrupts,
  cb: ControlLineInterrupts,
}

#[allow(dead_code)]
pub mod interrupt_bits {
  pub const MASTER: u8 = 0b10000000;
  pub const T1_ENABLE: u8 = 0b01000000;
  pub const T2_ENABLE: u8 = 0b00100000;
  pub const CB1_ENABLE: u8 = 0b00010000;
  pub const CB2_ENABLE: u8 = 0b00001000;
  pub const SR_ENABLE: u8 = 0b00000100;
  pub const CA1_ENABLE: u8 = 0b00000010;
  pub const CA2_ENABLE: u8 = 0b00000001;
}

#[allow(dead_code)]
pub mod pcr_bits {
  /// Set the direction of the CB2 line.
  pub const CB2_DIRECTION: u8 = 0b10000000; // 1 = output, 0 = input

  /// The CB2 control pins have different functions in different modes.
  pub const CB2_CONTROL: u8 = 0b01100000;

  // Input mode flags:
  pub const CB2_ACTIVE_TRANSITION: u8 = 0b01000000;

  /// When set, reading or writing output register B does not clear the flag
  pub const CB2_CLEAR_FLAG_ON_READ: u8 = 0b00100000;

  pub const CB1_ACTIVE_TRANSITION: u8 = 0b00010000; // 0 = falling, 1 = rising

  /// Set the direction of the CA2 line.
  pub const CA2_DIRECTION: u8 = 0b00001000; // 1 = output, 0 = input

  /// The CA2 control pins have different functions in different modes.
  pub const CA2_CONTROL: u8 = 0b00000110;

  // Input mode flags:
  pub const CA2_ACTIVE_TRANSITION: u8 = 0b00000100;

  /// When set, reading or writing output register A does not clear the flag
  pub const CA2_CLEAR_FLAG_ON_READ: u8 = 0b00000010;

  pub const CA1_ACTIVE_TRANSITION: u8 = 0b00000001; // 0 = falling, 1 = rising
}

impl Via {
  pub fn new(a: Box<dyn ControlLinesPort>, b: Box<dyn ControlLinesPort>) -> Self {
    Self {
      a: PortRegisters::new(a),
      b: PortRegisters::new(b),
      t1: Timer::new(),
      t2: Timer::new(),
      sr: ShiftRegister::new(),
      interrupts: InterruptRegister::new(),
      pcr: 0,
      ca: ControlLineInterrupts::new(),
      cb: ControlLineInterrupts::new(),
    }
  }
}

impl Memory for Via {
  fn read(&mut self, address: u16) -> u8 {
    match address % 0x10 {
      0x00 => {
        self.cb.clear_flags(self.pcr >> 4);
        self.b.read()
      }
      0x01 => {
        self.ca.clear_flags(self.pcr);
        self.a.read()
      }
      0x02 => self.b.ddr,
      0x03 => self.a.ddr,
      0x04 => {
        self.t1.interrupt = false;
        (self.t1.counter & 0xff) as u8
      }
      0x05 => ((self.t1.counter >> 8) & 0xff) as u8,
      0x06 => (self.t1.latch & 0xff) as u8,
      0x07 => ((self.t1.latch >> 8) & 0xff) as u8,
      0x08 => {
        self.t2.interrupt = false;
        (self.t2.counter & 0xff) as u8
      }
      0x09 => ((self.t2.counter >> 8) & 0xff) as u8,
      0x0a => self.sr.data,
      0x0b => {
        let t1_output_enable = match self.t1.output {
          TimerOutput::None => false,
          TimerOutput::Pulse => true,
          _ => unreachable!(),
        };

        let t2_pulse_counting = match self.t2.output {
          TimerOutput::None => false,
          TimerOutput::PulseCount => true,
          _ => unreachable!(),
        };

        (t1_output_enable as u8) << 7
          | (self.t1.continuous as u8) << 6
          | (t2_pulse_counting as u8) << 5
          | self.sr.control << 2
          | (self.b.latch_enabled as u8) << 1
          | (self.a.latch_enabled as u8)
      }
      0x0c => self.pcr,
      0x0d => {
        let mut value = 0;
        if self.t1.interrupt {
          value |= interrupt_bits::T1_ENABLE;
        }
        if self.t2.interrupt {
          value |= interrupt_bits::T2_ENABLE;
        }

        self.interrupts.read_flags(value)
      }
      0x0e => self.interrupts.read_enable(),
      0x0f => self.a.read(),
      _ => unreachable!(),
    }
  }

  fn write(&mut self, address: u16, value: u8) {
    match address % 0x10 {
      0x00 => {
        self.cb.clear_flags(self.pcr >> 4);
        self.b.write(value)
      }
      0x01 => {
        self.ca.clear_flags(self.pcr);
        self.a.write(value)
      }
      0x02 => self.b.ddr = value,
      0x03 => self.a.ddr = value,
      0x04 => self.t1.latch = (self.t1.latch & 0xff00) | (value as u16),
      0x05 => {
        self.t1.latch = (self.t1.latch & 0x00ff) | ((value as u16) << 8);
        self.t1.counter = self.t1.latch as i32;
        self.t1.running = true;
        self.t1.interrupt = false;
      }
      0x06 => self.t1.latch = (self.t1.latch & 0xff00) | (value as u16),
      0x07 => {
        self.t1.latch = (self.t1.latch & 0x00ff) | ((value as u16) << 8);
        self.t1.interrupt = false;
      }
      0x08 => self.t2.latch = (self.t2.latch & 0xff00) | (value as u16),
      0x09 => {
        self.t2.latch = (self.t2.latch & 0x00ff) | ((value as u16) << 8);
        self.t2.counter = self.t2.latch as i32;
        self.t2.running = true;
        self.t2.interrupt = false;
      }
      0x0a => self.sr.data = value,
      0x0b => {
        self.t1.continuous = (value & 0b01000000) != 0;
        self.sr.control = (value & 0b00011100) >> 2;
        self.b.latch_enabled = (value & 0b00000010) != 0;
        self.a.latch_enabled = (value & 0b00000001) != 0;

        self.t1.output = if (value & 0b10000000) != 0 {
          TimerOutput::Pulse
        } else {
          TimerOutput::None
        };
        self.t2.output = if (value & 0b00100000) != 0 {
          TimerOutput::PulseCount
        } else {
          TimerOutput::None
        };
      }
      0x0c => self.pcr = value,
      0x0d => {
        if (value & interrupt_bits::T1_ENABLE) == 0 {
          self.t1.interrupt = false;
        }
        if (value & interrupt_bits::T2_ENABLE) == 0 {
          self.t2.interrupt = false;
        }
      }
      0x0e => self.interrupts.write_enable(value),
      0x0f => self.a.write(value),
      _ => unreachable!(),
    }
  }

  fn reset(&mut self) {
    self.a.reset();
    self.b.reset();
  }

  fn poll(&mut self, cycles: u32, info: &SystemInfo) -> ActiveInterrupt {
    if self.t1.poll(cycles, info) && self.interrupts.is_enabled(interrupt_bits::T1_ENABLE) {
      return ActiveInterrupt::IRQ;
    }

    if self.t2.poll(cycles, info) && self.interrupts.is_enabled(interrupt_bits::T2_ENABLE) {
      return ActiveInterrupt::IRQ;
    }

    let ca = self.a.poll(cycles, info);
    let cb = self.b.poll(cycles, info);

    let ca_interrupts = self.ca.poll(self.pcr, ca);
    let cb_interrupts = self.cb.poll(self.pcr >> 4, cb);

    if ca_interrupts.c1 && self.interrupts.is_enabled(interrupt_bits::CA1_ENABLE) {
      return ActiveInterrupt::IRQ;
    }

    if cb_interrupts.c2 && self.interrupts.is_enabled(interrupt_bits::CB2_ENABLE) {
      return ActiveInterrupt::IRQ;
    }

    if ca_interrupts.c1 && self.interrupts.is_enabled(interrupt_bits::CA1_ENABLE) {
      return ActiveInterrupt::IRQ;
    }

    if cb_interrupts.c2 && self.interrupts.is_enabled(interrupt_bits::CB2_ENABLE) {
      return ActiveInterrupt::IRQ;
    }

    ActiveInterrupt::None
  }
}

#[cfg(test)]
mod tests {
  use crate::memory::mos::NullPort;

  use super::*;

  #[test]
  fn test_read_write() {
    let mut via = Via::new(Box::new(NullPort::new()), Box::new(NullPort::new()));

    // writes without DDR shouldn't be reflected in reads
    via.write(0x00, 0b10101010);
    assert_eq!(0, via.read(0x00));
    via.write(0x01, 0b00110011);
    assert_eq!(0, via.read(0x01));

    // write to the DDR
    via.write(0x02, 0b11110000);
    via.write(0x03, 0b00111100);

    // now, our past writes should be reflected in reads
    // (masked by the DDR)
    assert_eq!(0b10100000, via.read(0x00));
    assert_eq!(0b11110000, via.read(0x02));
    assert_eq!(0b00110000, via.read(0x01));
    assert_eq!(0b00111100, via.read(0x03));

    // and future writes should be reflected in reads
    via.write(0x00, 0b01010101);
    assert_eq!(0b01010000, via.read(0x00));
  }

  #[test]
  fn test_timer_1() {
    let mut via = Via::new(Box::new(NullPort::new()), Box::new(NullPort::new()));

    // enable timer 1 interrupts
    via.write(0x0e, interrupt_bits::MASTER | interrupt_bits::T1_ENABLE);

    // set the timer to count down from 0x10
    via.write(0x04, 0x10);
    via.write(0x05, 0x00);

    for _ in 0..0x0F {
      assert_eq!(ActiveInterrupt::None, via.poll(1, &SystemInfo::default()));
    }

    assert_eq!(ActiveInterrupt::IRQ, via.poll(1, &SystemInfo::default()));

    // polling again shouldn't do anything
    for _ in 0..0x20 {
      assert_eq!(ActiveInterrupt::None, via.poll(1, &SystemInfo::default()));
    }
  }

  #[test]
  fn test_timer_2() {
    let mut via = Via::new(Box::new(NullPort::new()), Box::new(NullPort::new()));

    // enable timer 2 interrupts
    via.write(0x0e, interrupt_bits::MASTER | interrupt_bits::T2_ENABLE);

    // set the timer to count down from 0x1234
    via.write(0x08, 0x34);

    // polling now shouldn't do anything
    assert_eq!(ActiveInterrupt::None, via.poll(1, &SystemInfo::default()));

    // timer begins when the high byte is written
    via.write(0x09, 0x12);

    for _ in 0..0x1233 {
      assert_eq!(ActiveInterrupt::None, via.poll(1, &SystemInfo::default()));
    }

    assert_eq!(ActiveInterrupt::IRQ, via.poll(1, &SystemInfo::default()));
  }

  #[test]
  fn test_t1_continuous() {
    let mut via = Via::new(Box::new(NullPort::new()), Box::new(NullPort::new()));

    // enable timer 1 interrupts
    via.write(0x0e, interrupt_bits::MASTER | interrupt_bits::T1_ENABLE);

    // set timer 1 to continuous mode
    via.write(0x0b, 0b01000000);

    // set the timer to count down from 0x10
    via.write(0x04, 0x10);
    via.write(0x05, 0x00);

    for _ in 0..0x0F {
      assert_eq!(ActiveInterrupt::None, via.poll(1, &SystemInfo::default()));
    }

    assert_eq!(ActiveInterrupt::IRQ, via.poll(1, &SystemInfo::default()));

    for _ in 0..0x0F {
      assert_eq!(ActiveInterrupt::None, via.poll(1, &SystemInfo::default()));
    }

    assert_eq!(ActiveInterrupt::IRQ, via.poll(1, &SystemInfo::default()));
  }

  #[test]
  fn test_ier_register() {
    let mut via = Via::new(Box::new(NullPort::new()), Box::new(NullPort::new()));

    // put something in the register
    via.write(
      0x0e,
      interrupt_bits::MASTER | interrupt_bits::T1_ENABLE | interrupt_bits::SR_ENABLE,
    );

    // we should read this with the master bit cleared
    assert_eq!(
      interrupt_bits::T1_ENABLE | interrupt_bits::SR_ENABLE,
      via.read(0x0e)
    );

    // *set* bits -- this shouldn't clear any
    via.write(
      0x0e,
      interrupt_bits::MASTER | interrupt_bits::T1_ENABLE | interrupt_bits::T2_ENABLE,
    );
    assert_eq!(
      interrupt_bits::T1_ENABLE | interrupt_bits::SR_ENABLE | interrupt_bits::T2_ENABLE,
      via.read(0x0e)
    );

    // *clear* bits
    via.write(0x0e, interrupt_bits::T2_ENABLE | interrupt_bits::SR_ENABLE);
    assert_eq!(interrupt_bits::T1_ENABLE, via.read(0x0e));
  }

  #[test]
  fn test_ier_timers() {
    let mut via = Via::new(Box::new(NullPort::new()), Box::new(NullPort::new()));

    // enable timer 1 interrupts
    via.write(0x0e, interrupt_bits::MASTER | interrupt_bits::T1_ENABLE);

    // set timer 1 to count down from 0x10
    via.write(0x04, 0x10);
    via.write(0x05, 0x00);

    // set timer 2 to count down from 0x08
    via.write(0x08, 0x08);
    via.write(0x09, 0x00);

    // timer 1 should interrupt first
    for _ in 0..0x0F {
      assert_eq!(ActiveInterrupt::None, via.poll(1, &SystemInfo::default()));
    }

    assert_eq!(ActiveInterrupt::IRQ, via.poll(1, &SystemInfo::default()));
  }

  #[test]
  fn test_ifr() {
    let mut via = Via::new(Box::new(NullPort::new()), Box::new(NullPort::new()));

    // enable timer 1 interrupts
    via.write(0x0e, interrupt_bits::MASTER | interrupt_bits::T1_ENABLE);

    // set timer 1 to continuous mode
    via.write(0x0b, 0b01000000);

    // set timer 1 to count down from 0x10
    via.write(0x04, 0x10);
    via.write(0x05, 0x00);

    // set timer 2 to count down from 0x08
    via.write(0x08, 0x08);
    via.write(0x09, 0x00);

    // timer 2 shouldn't trigger an interrupt
    for _ in 0..0x08 {
      assert_eq!(ActiveInterrupt::None, via.poll(1, &SystemInfo::default()));
    }

    // ...but the flag register should be set
    assert_eq!(interrupt_bits::T2_ENABLE, via.read(0x0d));

    // timer 1 should then trigger an interrupt
    for _ in 0..0x07 {
      assert_eq!(ActiveInterrupt::None, via.poll(1, &SystemInfo::default()));
    }
    assert_eq!(ActiveInterrupt::IRQ, via.poll(1, &SystemInfo::default()));

    // ...and set the corresponding flag, plus the master bit
    assert_eq!(
      interrupt_bits::MASTER | interrupt_bits::T1_ENABLE | interrupt_bits::T2_ENABLE,
      via.read(0x0d)
    );

    // clearing the master bit should have no effect
    via.write(0x0d, !interrupt_bits::MASTER);
    assert_eq!(
      interrupt_bits::MASTER | interrupt_bits::T1_ENABLE | interrupt_bits::T2_ENABLE,
      via.read(0x0d)
    );

    // clearing just timer 1 should clear the master bit
    via.write(0x0d, !interrupt_bits::T1_ENABLE);
    assert_eq!(interrupt_bits::T2_ENABLE, via.read(0x0d));

    // clearing timer 2 should work as expected
    via.write(0x0d, !interrupt_bits::T2_ENABLE);
    assert_eq!(0, via.read(0x0d));

    // if we let timer 1 run again, it should set the flag again
    for _ in 0..0x0F {
      assert_eq!(ActiveInterrupt::None, via.poll(1, &SystemInfo::default()));
    }
    assert_eq!(ActiveInterrupt::IRQ, via.poll(1, &SystemInfo::default()));
  }
}
