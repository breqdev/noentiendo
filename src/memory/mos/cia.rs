use crate::memory::{
  mos::{InterruptRegister, Port, ShiftRegister, Timer},
  ActiveInterrupt, Memory, SystemInfo,
};

/// A port and its associated registers on the MOS 6526 CIA.
pub struct PortRegisters {
  /// The Port implementation that this instance delegates to.
  port: Box<dyn Port>,

  /// Stores the current value written to the port.
  writes: u8,

  /// Data Direction Register. Each bit controls whether the line is an input (0) or output (1).
  ddr: u8,
}

impl PortRegisters {
  pub fn new(port: Box<dyn Port>) -> Self {
    Self {
      port,
      writes: 0,
      ddr: 0,
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

  /// Reset the port to its initial state.
  pub fn reset(&mut self) {
    self.ddr = 0;

    self.port.reset();
  }
}

struct TimeRegisters {
  tenth_seconds: u8,
  seconds: u8,
  ten_seconds: u8,
  minutes: u8,
  ten_minutes: u8,
  hours: u8,
  ten_hours: u8,
  am_pm: u8,
}

impl TimeRegisters {
  fn new() -> Self {
    Self {
      tenth_seconds: 0,
      seconds: 0,
      ten_seconds: 0,
      minutes: 0,
      ten_minutes: 0,
      hours: 0,
      ten_hours: 0,
      am_pm: 0,
    }
  }
}

struct TimeClock {
  time: TimeRegisters,
  alarm: TimeRegisters,
  rtc_rate: bool,     // if 0, runs at 60Hz, if 1, runs at 50Hz
  write_action: bool, // if 0, writes set the clock time; if 1, writes set the alarm time
}

impl TimeClock {
  fn new() -> Self {
    Self {
      time: TimeRegisters::new(),
      alarm: TimeRegisters::new(),
      rtc_rate: false,
      write_action: false,
    }
  }

  fn reset(&mut self) {
    self.time = TimeRegisters::new();
    self.alarm = TimeRegisters::new();
    self.rtc_rate = false;
    self.write_action = false;
  }
}

#[allow(dead_code)]
mod interrupt_bits {
  pub const TIMER_A: u8 = 0b0000_0001;
  pub const TIMER_B: u8 = 0b0000_0010;
  pub const ALARM: u8 = 0b0000_0100;
  pub const SHIFT_REGISTER: u8 = 0b0000_1000;
  pub const FLAG: u8 = 0b0001_0000;
  pub const MASTER: u8 = 0b1000_0000;
}

/// The MOS 6526 Complex Interface Adapter (CIA). Contains two ports, two timers,
/// a real-time clock, a shift register, and interrupt registers.
pub struct Cia {
  a: PortRegisters,
  b: PortRegisters,
  timer_a: Timer,
  timer_b: Timer,
  time_clock: TimeClock,
  shift_register: ShiftRegister,
  interrupts: InterruptRegister,
}

impl Cia {
  pub fn new(port_a: Box<dyn Port>, port_b: Box<dyn Port>) -> Self {
    Self {
      a: PortRegisters::new(port_a),
      b: PortRegisters::new(port_b),
      timer_a: Timer::new(),
      timer_b: Timer::new(),
      time_clock: TimeClock::new(),
      shift_register: ShiftRegister::new(),
      interrupts: InterruptRegister::new(),
    }
  }
}

impl Memory for Cia {
  fn read(&mut self, address: u16) -> u8 {
    match address % 0x10 {
      0x00 => self.a.read(),
      0x01 => self.b.read(),
      0x02 => self.a.ddr,
      0x03 => self.b.ddr,
      0x04 => self.timer_a.counter as u8,
      0x05 => (self.timer_a.counter >> 8) as u8,
      0x06 => self.timer_b.counter as u8,
      0x07 => (self.timer_b.counter >> 8) as u8,
      0x08 => self.time_clock.time.tenth_seconds,
      0x09 => (self.time_clock.time.ten_seconds << 4) | self.time_clock.time.seconds,
      0x0A => (self.time_clock.time.ten_minutes << 4) | self.time_clock.time.minutes,
      0x0B => {
        (self.time_clock.time.am_pm << 7)
          | (self.time_clock.time.ten_hours << 4)
          | self.time_clock.time.hours
      }
      0x0C => self.shift_register.data,
      0x0D => {
        // TODO: alarm and shift register flags
        let value = self
          .interrupts
          .read_flags((self.timer_a.interrupt as u8) | (self.timer_b.interrupt as u8) << 1);

        self.timer_a.interrupt = false;
        self.timer_b.interrupt = false;

        value
      }
      0x0E => {
        (self.timer_a.read_cia() & 0b0011_1111)
          | ((self.shift_register.direction as u8) << 6)
          | ((self.time_clock.rtc_rate as u8) << 7)
      }
      0x0F => self.timer_b.read_cia() | ((self.time_clock.write_action as u8) << 7),
      _ => unreachable!(),
    }
  }

  fn write(&mut self, address: u16, value: u8) {
    match address % 0x10 {
      0x00 => self.a.write(value),
      0x01 => self.b.write(value),
      0x02 => self.a.ddr = value,
      0x03 => self.b.ddr = value,
      0x04 => self.timer_a.latch = (self.timer_a.latch & 0xFF00) | value as u16,
      0x05 => {
        self.timer_a.latch = (self.timer_a.latch & 0x00FF) | ((value as u16) << 8);
        self.timer_a.counter = self.timer_a.latch as i32;
      }
      0x06 => self.timer_b.latch = (self.timer_b.latch & 0xFF00) | value as u16,
      0x07 => {
        self.timer_b.latch = (self.timer_b.latch & 0x00FF) | ((value as u16) << 8);
        self.timer_b.counter = self.timer_b.latch as i32;
      }
      0x08 => match self.time_clock.write_action {
        false => self.time_clock.time.tenth_seconds = value,
        true => self.time_clock.alarm.tenth_seconds = value,
      },
      0x09 => match self.time_clock.write_action {
        false => {
          self.time_clock.time.ten_seconds = value >> 4;
          self.time_clock.time.seconds = value & 0x0F;
        }
        true => {
          self.time_clock.alarm.ten_seconds = value >> 4;
          self.time_clock.alarm.seconds = value & 0x0F;
        }
      },
      0x0A => match self.time_clock.write_action {
        false => {
          self.time_clock.time.ten_minutes = value >> 4;
          self.time_clock.time.minutes = value & 0x0F;
        }
        true => {
          self.time_clock.alarm.ten_minutes = value >> 4;
          self.time_clock.alarm.minutes = value & 0x0F;
        }
      },
      0x0B => match self.time_clock.write_action {
        false => {
          self.time_clock.time.am_pm = value >> 7;
          self.time_clock.time.ten_hours = value >> 4;
          self.time_clock.time.hours = value & 0x0F;
        }
        true => {
          self.time_clock.alarm.am_pm = value >> 7;
          self.time_clock.alarm.ten_hours = value >> 4;
          self.time_clock.alarm.hours = value & 0x0F;
        }
      },
      0x0C => self.shift_register.data = value,
      0x0D => self.interrupts.write_enable(value),
      0x0E => {
        self.timer_a.write_cia(value & 0b0011_1111);
        self.shift_register.direction = value & 0b0100_0000 != 0;
        self.time_clock.rtc_rate = value & 0b1000_0000 != 0;
      }
      0x0F => {
        self.timer_b.write_cia(value & 0b0111_1111);
        self.time_clock.write_action = value & 0b1000_0000 != 0;
      }
      _ => unreachable!(),
    }
  }

  fn reset(&mut self) {
    self.a.reset();
    self.b.reset();
    self.timer_a.reset();
    self.timer_b.reset();
    self.time_clock.reset();
    self.shift_register.reset();
    self.interrupts.reset();
  }

  fn poll(&mut self, cycles: u32, info: &SystemInfo) -> ActiveInterrupt {
    if self.timer_a.poll(cycles, info)
      && (self.interrupts.interrupt_enable & interrupt_bits::TIMER_A) != 0
    {
      return ActiveInterrupt::IRQ;
    }

    if self.timer_b.poll(cycles, info)
      && (self.interrupts.interrupt_enable & interrupt_bits::TIMER_B) != 0
    {
      return ActiveInterrupt::IRQ;
    }

    // TODO: poll FLAG pin

    ActiveInterrupt::None
  }
}

#[cfg(test)]
mod tests {
  use crate::memory::mos::NullPort;

  use super::*;

  #[test]
  fn test_read_write() {
    let mut cia = Cia::new(Box::new(NullPort::new()), Box::new(NullPort::new()));

    // writes without DDR shouldn't be reflected in reads
    cia.write(0x00, 0b10101010);
    assert_eq!(0, cia.read(0x00));
    cia.write(0x01, 0b00110011);
    assert_eq!(0, cia.read(0x01));

    // write to the DDR
    cia.write(0x02, 0b11110000);
    cia.write(0x03, 0b00111100);

    // now, our past writes should be reflected in reads
    // (masked by the DDR)
    assert_eq!(0b10100000, cia.read(0x00));
    assert_eq!(0b11110000, cia.read(0x02));
    assert_eq!(0b00110000, cia.read(0x01));
    assert_eq!(0b00111100, cia.read(0x03));

    // and future writes should be reflected in reads
    cia.write(0x00, 0b01010101);
    assert_eq!(0b01010000, cia.read(0x00));
  }

  #[test]
  fn test_timer_a() {
    let mut cia = Cia::new(Box::new(NullPort::new()), Box::new(NullPort::new()));

    // enable timer A interrupts
    cia.write(0x0D, interrupt_bits::MASTER | interrupt_bits::TIMER_A);

    // set the timer to count down from 0x10
    cia.write(0x04, 0x10);
    cia.write(0x05, 0x00);

    // start the timer, and disable continuous operation
    cia.write(0x0E, 0b0000_1001);

    for _ in 0..0x0F {
      assert_eq!(ActiveInterrupt::None, cia.poll(1, &SystemInfo::default()));
    }

    assert_eq!(ActiveInterrupt::IRQ, cia.poll(1, &SystemInfo::default()));

    // polling again shouldn't do anything
    for _ in 0..0x20 {
      assert_eq!(ActiveInterrupt::None, cia.poll(1, &SystemInfo::default()));
    }
  }

  #[test]
  fn test_timer_b() {
    let mut cia = Cia::new(Box::new(NullPort::new()), Box::new(NullPort::new()));

    // enable timer B interrupts
    cia.write(0x0D, interrupt_bits::MASTER | interrupt_bits::TIMER_B);

    // set the timer to count down from 0x1234
    cia.write(0x06, 0x34);
    cia.write(0x07, 0x12);

    // start the timer, and disable continuous operation
    cia.write(0x0F, 0b0000_1001);

    for _ in 0..0x1233 {
      assert_eq!(ActiveInterrupt::None, cia.poll(1, &SystemInfo::default()));
    }

    assert_eq!(ActiveInterrupt::IRQ, cia.poll(1, &SystemInfo::default()));
  }

  #[test]
  fn test_timer_a_continuous() {
    let mut cia = Cia::new(Box::new(NullPort::new()), Box::new(NullPort::new()));

    // enable timer A interrupts
    cia.write(0x0D, interrupt_bits::MASTER | interrupt_bits::TIMER_A);

    // set the timer to count down from 0x10
    cia.write(0x04, 0x10);
    cia.write(0x05, 0x00);

    // start the timer, and enable continuous operation
    cia.write(0x0E, 0b0000_0001);

    assert_eq!(
      ActiveInterrupt::None,
      cia.poll(0x0F, &SystemInfo::default())
    );

    assert_eq!(ActiveInterrupt::IRQ, cia.poll(1, &SystemInfo::default()));

    assert_eq!(
      ActiveInterrupt::None,
      cia.poll(0x0F, &SystemInfo::default())
    );

    assert_eq!(ActiveInterrupt::IRQ, cia.poll(1, &SystemInfo::default()));
  }

  #[test]
  fn test_ier_timers() {
    let mut cia = Cia::new(Box::new(NullPort::new()), Box::new(NullPort::new()));

    // enable timer 1 interrupts
    cia.write(0x0D, interrupt_bits::MASTER | interrupt_bits::TIMER_A);

    // set timer 1 to count down from 0x10
    cia.write(0x04, 0x10);
    cia.write(0x05, 0x00);

    // set timer 2 to count down from 0x08
    cia.write(0x06, 0x08);
    cia.write(0x07, 0x00);

    // set timers to running
    cia.write(0x0E, 0b0000_1001);
    cia.write(0x0F, 0b0000_1001);

    // timer 1 should interrupt first
    for _ in 0..0x0F {
      assert_eq!(ActiveInterrupt::None, cia.poll(1, &SystemInfo::default()));
    }

    assert_eq!(ActiveInterrupt::IRQ, cia.poll(1, &SystemInfo::default()));
  }

  #[test]
  fn test_interrupt_flags() {
    let mut cia = Cia::new(Box::new(NullPort::new()), Box::new(NullPort::new()));

    // enable timer 1 interrupts
    cia.write(0x0D, interrupt_bits::MASTER | interrupt_bits::TIMER_A);

    // set timer 1 to continuous mode
    cia.write(0x0E, 0b0000_0001);

    // set timer 1 to count down from 0x10
    cia.write(0x04, 0x10);
    cia.write(0x05, 0x00);

    // set timer 2 to count down from 0x08
    cia.write(0x06, 0x08);
    cia.write(0x07, 0x00);

    // timer 2 shouldn't trigger an interrupt
    for _ in 0..0x08 {
      assert_eq!(ActiveInterrupt::None, cia.poll(1, &SystemInfo::default()));
    }

    // ...but the flag register should be set
    assert_eq!(interrupt_bits::TIMER_B, cia.read(0x0D));

    // and subsequent reads to the flag register should be 0
    assert_eq!(0x00, cia.read(0x0D));

    // timer 1 should then trigger an interrupt
    for _ in 0..0x07 {
      assert_eq!(ActiveInterrupt::None, cia.poll(1, &SystemInfo::default()));
    }
    assert_eq!(ActiveInterrupt::IRQ, cia.poll(1, &SystemInfo::default()));

    // ...and set the corresponding flag, plus the master bit
    assert_eq!(
      interrupt_bits::MASTER | interrupt_bits::TIMER_A,
      cia.read(0x0D)
    );

    // and subsequent reads to the flag register should be 0
    assert_eq!(0x00, cia.read(0x0D));

    // if we let timer 1 run again, it should set the flag again
    for _ in 0..0x0F {
      assert_eq!(ActiveInterrupt::None, cia.poll(1, &SystemInfo::default()));
    }
    assert_eq!(ActiveInterrupt::IRQ, cia.poll(1, &SystemInfo::default()));
  }
}
