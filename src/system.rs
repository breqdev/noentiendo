use crate::execute::Execute;
use crate::fetch::Fetch;
use crate::memory::{ActiveInterrupt, Memory, SystemInfo};
use crate::registers::{flags, Registers};
use std::thread;
use std::time::{Duration, Instant};

pub struct System {
  pub registers: Registers,
  memory: Box<dyn Memory>,
  cycles_per_second: u64,
  time_delta: Duration,
  last_tick: Instant,
  cycle_count: u64,
  last_report_time: Instant,
  last_report_cycles: u64,
}

pub trait MemoryIO {
  fn read(&mut self, address: u16) -> u8;
  fn write(&mut self, address: u16, value: u8);
  fn read_word(&mut self, address: u16) -> u16;
  fn write_word(&mut self, address: u16, value: u16);
}

impl MemoryIO for System {
  fn read(&mut self, address: u16) -> u8 {
    self.memory.read(address)
  }

  fn read_word(&mut self, address: u16) -> u16 {
    let lo = self.memory.read(address);
    let hi = self.memory.read(address + 1);
    (hi as u16) << 8 | lo as u16
  }

  fn write(&mut self, address: u16, value: u8) {
    self.memory.write(address, value);
  }

  fn write_word(&mut self, address: u16, value: u16) {
    self.memory.write(address, value as u8);
    self.memory.write(address + 1, (value >> 8) as u8);
  }
}

pub trait Stack {
  fn push(&mut self, value: u8);
  fn pop(&mut self) -> u8;

  fn push_word(&mut self, value: u16);
  fn pop_word(&mut self) -> u16;
}

impl Stack for System {
  fn push(&mut self, value: u8) {
    self.registers.sp.push();
    self.write(self.registers.sp.address(), value);
  }

  fn pop(&mut self) -> u8 {
    let value = self.read(self.registers.sp.address());
    self.registers.sp.pop();
    value
  }

  fn push_word(&mut self, value: u16) {
    self.push((value & 0xFF) as u8);
    self.push((value >> 8) as u8);
  }

  fn pop_word(&mut self) -> u16 {
    let hi = self.pop();
    let lo = self.pop();
    (hi as u16) << 8 | lo as u16
  }
}

pub trait InterruptHandler {
  fn interrupt(&mut self, maskable: bool);
}

impl InterruptHandler for System {
  fn interrupt(&mut self, maskable: bool) {
    self.registers.sr.set(flags::INTERRUPT);
    self.push_word(self.registers.pc.address());
    self.push(self.registers.sr.get());

    let dest = match maskable {
      false => self.read_word(0xFFFA),
      true => self.read_word(0xFFFE),
    };

    self.registers.pc.load(dest);
  }
}

impl System {
  pub fn new(memory: Box<dyn Memory>, cycles_per_second: u64) -> System {
    System {
      registers: Registers::new(),
      memory,
      cycles_per_second,
      time_delta: Duration::from_micros(if cycles_per_second == 0 {
        0
      } else {
        (1_000_000.0 / cycles_per_second as f64) as u64
      }),
      last_tick: Instant::now(),
      cycle_count: 0,
      last_report_time: Instant::now(),
      last_report_cycles: 0,
    }
  }

  pub fn reset(&mut self) {
    self.memory.reset();
    self.registers.reset();
    let pc_address = self.read_word(0xFFFC);
    self.registers.pc.load(pc_address);
  }

  pub fn tick(&mut self) {
    let opcode = self.fetch();
    match self.execute(opcode) {
      Ok(()) => {}
      Err(_) => {
        panic!(
          "Failed to execute instruction {:02x} at {:04x}",
          opcode,
          self.registers.pc.address()
        );
      }
    }
    self.cycle_count += 1;

    let info = SystemInfo {
      cycles_per_second: self.cycles_per_second,
      cycle_count: self.cycle_count,
    };

    match self.memory.poll(&info) {
      ActiveInterrupt::None => (),
      ActiveInterrupt::NMI => self.interrupt(false),
      ActiveInterrupt::IRQ => self.interrupt(true),
    }

    let elapsed = Instant::now().duration_since(self.last_tick);
    if elapsed < self.time_delta {
      thread::sleep(self.time_delta - elapsed);
    }
    self.last_tick = Instant::now();

    if (self.last_tick - self.last_report_time).as_secs() > 1 {
      println!(
        "Cycles: {} ({}/s)",
        self.cycle_count,
        self.cycle_count - self.last_report_cycles
      );
      self.last_report_time = self.last_tick;
      self.last_report_cycles = self.cycle_count;
    }
  }
}
