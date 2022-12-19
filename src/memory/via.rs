use std::{cell::Cell, rc::Rc};

use crate::{
  memory::{ActiveInterrupt, Memory, Port, SystemInfo},
  platform::PlatformProvider,
};

// MOS 6522

struct PortRegisters {
  port: Box<dyn Port>,
  writes: Cell<u8>, // if the DDR is write, allow reading the current written value
  ddr: Cell<u8>, // data direction register, each bit controls whether the line is an input (0) or output (1)
  latch_enabled: Cell<bool>,
}

impl PortRegisters {
  pub fn new(port: Box<dyn Port>) -> Self {
    Self {
      port,
      writes: Cell::new(0),
      ddr: Cell::new(0),
      latch_enabled: Cell::new(false),
    }
  }

  pub fn read(&self, root: &Rc<dyn Memory>, platform: &Box<dyn PlatformProvider>) -> u8 {
    (self.port.read(root, platform) & !self.ddr.get()) | (self.writes.get() & self.ddr.get())
  }

  pub fn write(&self, value: u8, root: &Rc<dyn Memory>, platform: &Box<dyn PlatformProvider>) {
    self.writes.set(value);
    self.port.write(value & self.ddr.get(), root, platform);
  }

  pub fn poll(&self, info: &SystemInfo) -> bool {
    self.port.poll(info)
  }

  pub fn reset(&self, root: &Rc<dyn Memory>, platform: &Box<dyn PlatformProvider>) {
    self.ddr.set(0);
    self.writes.set(0);

    self.port.reset(root, platform);
  }
}

struct Timer {
  latch: Cell<u16>,
  counter: Cell<u16>,
  interrupt: Cell<bool>,
  continuous: Cell<bool>, // if false, the timer will fire once; if true, it will load the latch into the counter and keep going
  pulse_counting: Cell<bool>, // if true, the timer will output a set number of pulses on PB6
  output_enable: Cell<bool>, // if true, the timer will output a pulse on PB7
}

impl Timer {
  pub fn new() -> Self {
    Self {
      latch: Cell::new(0),
      counter: Cell::new(0),
      interrupt: Cell::new(false),
      continuous: Cell::new(false),
      pulse_counting: Cell::new(false),
      output_enable: Cell::new(false),
    }
  }

  pub fn poll(&self, _info: &SystemInfo) -> bool {
    if self.counter.get() == 0 {
      if self.continuous.get() {
        self.counter.set(self.latch.get())
      } else {
        return false;
      }
    }

    self.counter.set(self.counter.get().wrapping_sub(1));

    if self.counter.get() == 0 {
      self.interrupt.set(true);

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
  data: Cell<u8>,
  control: Cell<u8>,
}

impl ShiftRegister {
  pub fn new() -> Self {
    Self {
      data: Cell::new(0),
      control: Cell::new(0),
    }
  }
}

pub struct VIA {
  a: PortRegisters,
  b: PortRegisters,
  t1: Timer,
  t2: Timer,
  sr: ShiftRegister,
  pcr: Cell<u8>, // peripheral control register
  ier: Cell<u8>, // interrupt enable register
}

impl VIA {
  pub fn new(a: Box<dyn Port>, b: Box<dyn Port>) -> Self {
    Self {
      a: PortRegisters::new(a),
      b: PortRegisters::new(b),
      t1: Timer::new(),
      t2: Timer::new(),
      sr: ShiftRegister::new(),
      pcr: Cell::new(0),
      ier: Cell::new(0),
    }
  }
}

impl Memory for VIA {
  fn read(&self, address: u16, root: &Rc<dyn Memory>, platform: &Box<dyn PlatformProvider>) -> u8 {
    match address % 0x10 {
      0x00 => self.b.read(root, platform),
      0x01 => self.a.read(root, platform), // TODO: controls handshake?
      0x02 => self.b.ddr.get(),
      0x03 => self.a.ddr.get(),
      0x04 => {
        self.t1.interrupt.set(false);
        let value = (self.t1.counter.get() & 0xff) as u8;
        value
      }
      0x05 => ((self.t1.counter.get() >> 8) & 0xff) as u8,
      0x06 => (self.t1.latch.get() & 0xff) as u8,
      0x07 => ((self.t1.latch.get() >> 8) & 0xff) as u8,
      0x08 => {
        self.t2.interrupt.set(false);
        (self.t2.counter.get() & 0xff) as u8
      }
      0x09 => ((self.t2.counter.get() >> 8) & 0xff) as u8,
      0x0a => self.sr.data.get(),
      0x0b => {
        (self.t1.output_enable.get() as u8) << 7
          | (self.t1.continuous.get() as u8) << 6
          | (self.t2.pulse_counting.get() as u8) << 5
          | self.sr.control.get() << 2
          | (self.b.latch_enabled.get() as u8) << 1
          | (self.a.latch_enabled.get() as u8)
      }
      0x0c => self.pcr.get(),
      0x0d => {
        let mut value = 0;
        if self.t1.interrupt.get() {
          value |= 0b01000000;
        }
        if self.t2.interrupt.get() {
          value |= 0b00100000;
        }

        if (value & self.ier.get()) != 0 {
          value |= 0b10000000; // master flag
        }
        value
      }
      0x0e => self.ier.get(),
      0x0f => self.a.read(root, platform),
      _ => unreachable!(),
    }
  }

  fn write(
    &self,
    address: u16,
    value: u8,
    root: &Rc<dyn Memory>,
    platform: &Box<dyn PlatformProvider>,
  ) {
    match address % 0x10 {
      0x00 => self.b.write(value, root, platform),
      0x01 => self.a.write(value, root, platform), // TODO: controls handshake?
      0x02 => self.b.ddr.set(value),
      0x03 => self.a.ddr.set(value),
      0x04 => self
        .t1
        .latch
        .set((self.t1.latch.get() & 0xff00) | (value as u16)),
      0x05 => {
        self
          .t1
          .latch
          .set((self.t1.latch.get() & 0x00ff) | ((value as u16) << 8));
        self.t1.counter.set(self.t1.latch.get());
        self.t1.interrupt.set(false);
      }
      0x06 => self
        .t1
        .latch
        .set((self.t1.latch.get() & 0xff00) | (value as u16)),
      0x07 => {
        self
          .t1
          .latch
          .set((self.t1.latch.get() & 0x00ff) | ((value as u16) << 8));
        self.t1.interrupt.set(false);
      }
      0x08 => self
        .t2
        .latch
        .set((self.t2.latch.get() & 0xff00) | (value as u16)),
      0x09 => {
        self
          .t2
          .counter
          .set((self.t2.latch.get() & 0x00ff) | ((value as u16) << 8));
        self.t2.interrupt.set(false);
      }
      0x0a => self.sr.data.set(value),
      0x0b => {
        self.t1.output_enable.set((value & 0b10000000) != 0);
        self.t1.continuous.set((value & 0b01000000) != 0);
        self.t2.pulse_counting.set((value & 0b00100000) != 0);
        self.sr.control.set((value & 0b00011100) >> 2);
        self.b.latch_enabled.set((value & 0b00000010) != 0);
        self.a.latch_enabled.set((value & 0b00000001) != 0);
      }
      0x0c => self.pcr.set(value),
      0x0d => {
        if (value & 0b01000000) == 0 {
          self.t1.interrupt.set(false);
        }
        if (value & 0b00100000) == 0 {
          self.t2.interrupt.set(false);
        }
      }
      0x0e => self.ier.set(value),
      0x0f => self.a.write(value, root, platform),
      _ => unreachable!(),
    }
  }

  fn reset(&self, root: &Rc<dyn Memory>, platform: &Box<dyn PlatformProvider>) {
    self.a.reset(root, platform);
    self.b.reset(root, platform);
  }

  fn poll(
    &self,
    info: &SystemInfo,
    _root: &Rc<dyn Memory>,
    _platform: &Box<dyn PlatformProvider>,
  ) -> ActiveInterrupt {
    if self.t1.poll(info) && (self.ier.get() & 0b01000000) != 0 {
      return ActiveInterrupt::IRQ;
    }

    if self.t2.poll(info) && (self.ier.get() & 0b00100000) != 0 {
      return ActiveInterrupt::IRQ;
    }

    ActiveInterrupt::None
  }
}
