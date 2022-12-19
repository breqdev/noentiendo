use crate::memory::{ActiveInterrupt, Memory, Port, SystemInfo};

// MOS 6522

struct PortRegisters {
  port: Box<dyn Port>,
  writes: u8, // if the DDR is write, allow reading the current written value
  ddr: u8, // data direction register, each bit controls whether the line is an input (0) or output (1)
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

  pub fn read(&mut self) -> u8 {
    (self.port.read() & !self.ddr) | (self.writes & self.ddr)
  }

  pub fn write(&mut self, value: u8) {
    self.writes = value;
    self.port.write(value & self.ddr);
  }

  pub fn poll(&mut self, info: &SystemInfo) -> bool {
    self.port.poll(info)
  }

  pub fn reset(&mut self) {
    self.ddr = 0;

    self.port.reset();
  }
}

struct Timer {
  latch: u16,
  counter: u16,
  interrupt: bool,
  continuous: bool, // if false, the timer will fire once; if true, it will load the latch into the counter and keep going
  pulse_counting: bool, // if true, the timer will output a set number of pulses on PB6
  output_enable: bool, // if true, the timer will output a pulse on PB7
}

impl Timer {
  pub fn new() -> Self {
    Self {
      latch: 0,
      counter: 0,
      interrupt: false,
      continuous: false,
      pulse_counting: false,
      output_enable: false,
    }
  }

  pub fn poll(&mut self, _info: &SystemInfo) -> bool {
    if self.counter == 0 {
      if self.continuous {
        self.counter = self.latch
      } else {
        return false;
      }
    }

    self.counter = self.counter.wrapping_sub(1);

    if self.counter == 0 {
      self.interrupt = true;

      true
    } else {
      false
    }
  }
}

pub mod sr_control_bits {
  pub const SHIFT_DISABLED: u8 = 0b000;
  pub const SHIFT_IN_BY_T2: u8 = 0b001;
  pub const SHIFT_IN_BY_SYSTEM_CLOCK: u8 = 0b010;
  pub const SHIFT_IN_BY_EXTERNAL_CLOCK: u8 = 0b011; // PB6?

  pub const SHIFT_OUT_FREE_RUN: u8 = 0b100; // runs by T2, but disables the counter to run forever
  pub const SHIFT_OUT_BY_T2: u8 = 0b101;
  pub const SHIFT_OUT_BY_SYSTEM_CLOCK: u8 = 0b110;
  pub const SHIFT_OUT_BY_EXTERNAL_CLOCK: u8 = 0b111; // PB6?

  pub const C1_ACTIVE_TRANSITION_FLAG: u8 = 0b10000000; // 1 = 0->1, 0 = 1->0
  pub const C2_ACTIVE_TRANSITION_FLAG: u8 = 0b01000000;
  pub const C2_DIRECTION: u8 = 0b00100000; // 1 = output, 0 = input
  pub const C2_CONTROL: u8 = 0b00011000; // ???
  pub const DDR_SELECT: u8 = 0b00000100; // enable accessing DDR
  pub const C1_CONTROL: u8 = 0b00000011; // interrupt status control
}

struct ShiftRegister {
  data: u8,
  control: u8,
}

impl ShiftRegister {
  pub fn new() -> Self {
    Self {
      data: 0,
      control: 0,
    }
  }
}

pub struct VIA {
  a: PortRegisters,
  b: PortRegisters,
  t1: Timer,
  t2: Timer,
  sr: ShiftRegister,
  pcr: u8, // peripheral control register
  ier: u8, // interrupt enable register
}

impl VIA {
  pub fn new(a: Box<dyn Port>, b: Box<dyn Port>) -> Self {
    Self {
      a: PortRegisters::new(a),
      b: PortRegisters::new(b),
      t1: Timer::new(),
      t2: Timer::new(),
      sr: ShiftRegister::new(),
      pcr: 0,
      ier: 0,
    }
  }
}

impl Memory for VIA {
  fn read(&mut self, address: u16) -> u8 {
    match address % 0x10 {
      0x00 => self.b.read(),
      0x01 => self.a.read(), // TODO: controls handshake?
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
        (self.t1.output_enable as u8) << 7
          | (self.t1.continuous as u8) << 6
          | (self.t2.pulse_counting as u8) << 5
          | self.sr.control << 2
          | (self.b.latch_enabled as u8) << 1
          | (self.a.latch_enabled as u8)
      }
      0x0c => self.pcr,
      0x0d => {
        let mut value = 0;
        if self.t1.interrupt {
          value |= 0b01000000;
        }
        if self.t2.interrupt {
          value |= 0b00100000;
        }

        if (value & self.ier) != 0 {
          value |= 0b10000000; // master flag
        }
        value
      }
      0x0e => self.ier,
      0x0f => self.a.read(),
      _ => unreachable!(),
    }
  }

  fn write(&mut self, address: u16, value: u8) {
    match address % 0x10 {
      0x00 => self.b.write(value),
      0x01 => self.a.write(value), // TODO: controls handshake?
      0x02 => self.b.ddr = value,
      0x03 => self.a.ddr = value,
      0x04 => self.t1.latch = (self.t1.latch & 0xff00) | (value as u16),
      0x05 => {
        self.t1.latch = (self.t1.latch & 0x00ff) | ((value as u16) << 8);
        self.t1.counter = self.t1.latch;
        self.t1.interrupt = false;
      }
      0x06 => self.t1.latch = (self.t1.latch & 0xff00) | (value as u16),
      0x07 => {
        self.t1.latch = (self.t1.latch & 0x00ff) | ((value as u16) << 8);
        self.t1.interrupt = false;
      }
      0x08 => self.t2.latch = (self.t2.latch & 0xff00) | (value as u16),
      0x09 => {
        self.t2.counter = (self.t2.latch & 0x00ff) | ((value as u16) << 8);
        self.t2.interrupt = false;
      }
      0x0a => self.sr.data = value,
      0x0b => {
        self.t1.output_enable = (value & 0b10000000) != 0;
        self.t1.continuous = (value & 0b01000000) != 0;
        self.t2.pulse_counting = (value & 0b00100000) != 0;
        self.sr.control = (value & 0b00011100) >> 2;
        self.b.latch_enabled = (value & 0b00000010) != 0;
        self.a.latch_enabled = (value & 0b00000001) != 0;
      }
      0x0c => self.pcr = value,
      0x0d => {
        if (value & 0b01000000) == 0 {
          self.t1.interrupt = false;
        }
        if (value & 0b00100000) == 0 {
          self.t2.interrupt = false;
        }
      }
      0x0e => self.ier = value,
      0x0f => self.a.write(value),
      _ => unreachable!(),
    }
  }

  fn reset(&mut self) {
    self.a.reset();
    self.b.reset();
  }

  fn poll(&mut self, info: &SystemInfo) -> ActiveInterrupt {
    if self.t1.poll(info) && (self.ier & 0b01000000) != 0 {
      return ActiveInterrupt::IRQ;
    }

    if self.t2.poll(info) && (self.ier & 0b00100000) != 0 {
      return ActiveInterrupt::IRQ;
    }

    if self.a.poll(info) || self.b.poll(info) {
      return ActiveInterrupt::IRQ;
    }

    ActiveInterrupt::None
  }
}
