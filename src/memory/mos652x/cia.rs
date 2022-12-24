use crate::memory::{mos652x::PortRegisters, ActiveInterrupt, Memory, Port, SystemInfo};

enum TimerClockSource {
  Phi2,        // system clock pulses
  Count,       // external CNT pin
  TimerA,      // timer A underflow
  TimerACount, // timer A underflow and external CNT pin
}

struct Timer {
  latch: u16,
  counter: u16,
  running: bool,

  /// if 1, the timer will output a pulse on PB6 (timer A) or PB7 (timer B)
  output_enable: bool,

  /// if 0, the timer will output a one-tick pulse; if 1, the timer will toggle the output
  toggle_pulse: bool,

  /// if 0, the timer will start counting again after reaching 0; if 1, the timer will stop
  continuous: bool,

  clock_source: TimerClockSource,

  interrupt: bool,
}

impl Timer {
  fn new() -> Self {
    Self {
      latch: 0,
      counter: 0,
      running: false,
      output_enable: false,
      toggle_pulse: false,
      continuous: false,
      clock_source: TimerClockSource::Phi2,
      interrupt: false,
    }
  }

  fn read(&self) -> u8 {
    let clock_source = match self.clock_source {
      TimerClockSource::Phi2 => 0,
      TimerClockSource::Count => 1,
      TimerClockSource::TimerA => 2,
      TimerClockSource::TimerACount => 3,
    };

    (clock_source << 4)
      | (self.continuous as u8) << 3
      | (self.toggle_pulse as u8) << 2
      | (self.output_enable as u8) << 1
      | (self.running as u8)
  }

  fn write(&mut self, value: u8) {
    self.running = (value & 0b0000_0001) != 0;
    self.output_enable = (value & 0b0000_0010) != 0;
    self.toggle_pulse = (value & 0b0000_0100) != 0;
    self.continuous = (value & 0b0000_1000) != 0;

    self.clock_source = match value & 0b0011_0000 {
      0b0000_0000 => TimerClockSource::Phi2,
      0b0001_0000 => TimerClockSource::Count,
      0b0010_0000 => TimerClockSource::TimerA,
      0b0011_0000 => TimerClockSource::TimerACount,
      _ => unreachable!(),
    };
  }

  fn reset(&mut self) {
    self.latch = 0;
    self.counter = 0;
    self.running = false;
    self.output_enable = false;
    self.toggle_pulse = false;
    self.continuous = false;
    self.clock_source = TimerClockSource::Phi2;
  }

  fn poll(&mut self, _info: &SystemInfo) -> bool {
    if self.counter == 0 {
      if !self.continuous {
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

struct ShiftRegister {
  data: u8,
  direction: bool, // if 0, the shift register is in input mode; if 1, the shift register is in output mode
}

impl ShiftRegister {
  pub fn new() -> Self {
    Self {
      data: 0,
      direction: false,
    }
  }

  pub fn reset(&mut self) {
    self.data = 0;
    self.direction = false;
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

pub struct InterruptRegister {
  pub interrupt_enable: u8,
  pub interrupt_flag: u8,
}

impl InterruptRegister {
  fn read(&self) -> u8 {
    let mut value = 0;

    if (value & self.interrupt_enable) != 0 {
      value |= interrupt_bits::MASTER;
    }

    value
  }

  fn write(&mut self, value: u8) {
    if (value & interrupt_bits::MASTER) != 0 {
      // set bits
      self.interrupt_enable |= value & 0b01111111;
    } else {
      // clear bits
      self.interrupt_enable &= !(value & 0b01111111);
    }
  }
}

impl InterruptRegister {
  fn new() -> Self {
    Self {
      interrupt_enable: 0,
      interrupt_flag: 0,
    }
  }

  fn reset(&mut self) {
    self.interrupt_enable = 0;
    self.interrupt_flag = 0;
  }
}

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
    // println!("Read from CIA at address {:04x}", address);
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
      0x0D => self.interrupts.read(),
      0x0E => {
        (self.timer_a.read() & 0b0011_1111)
          | ((self.shift_register.direction as u8) << 6)
          | ((self.time_clock.rtc_rate as u8) << 7)
      }
      0x0F => self.timer_b.read() | ((self.time_clock.write_action as u8) << 7),
      _ => unreachable!(),
    }
  }

  fn write(&mut self, address: u16, value: u8) {
    // println!(
    //   "Write to CIA at address {:04x}, value {:02x}",
    //   address, value
    // );
    match address % 0x10 {
      0x00 => self.a.write(value),
      0x01 => self.b.write(value),
      0x02 => self.a.ddr = value,
      0x03 => self.b.ddr = value,
      0x04 => self.timer_a.latch = (self.timer_a.latch & 0xFF00) | value as u16,
      0x05 => self.timer_a.latch = (self.timer_a.latch & 0x00FF) | ((value as u16) << 8), // TODO: reset?
      0x06 => self.timer_b.latch = (self.timer_b.latch & 0xFF00) | value as u16,
      0x07 => self.timer_b.latch = (self.timer_b.latch & 0x00FF) | ((value as u16) << 8), // TODO: reset?
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
      0x0D => self.interrupts.write(value),
      0x0E => {
        self.timer_a.write(value & 0b0011_1111);
        self.shift_register.direction = value & 0b0100_0000 != 0;
        self.time_clock.rtc_rate = value & 0b1000_0000 != 0;
      }
      0x0F => {
        self.timer_b.write(value & 0b0111_1111);
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

  fn poll(&mut self, info: &SystemInfo) -> ActiveInterrupt {
    if self.timer_a.poll(info) && (self.interrupts.interrupt_enable & interrupt_bits::TIMER_A) != 0
    {
      return ActiveInterrupt::IRQ;
    }

    if self.timer_b.poll(info) && (self.interrupts.interrupt_enable & interrupt_bits::TIMER_B) != 0
    {
      println!("timer b interrupt");
      return ActiveInterrupt::IRQ;
    }

    if self.a.poll(info) || self.b.poll(info) {
      return ActiveInterrupt::IRQ;
    }

    ActiveInterrupt::None
  }
}
