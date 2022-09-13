use crate::execute::Execute;
use crate::fetch::Fetch;
use crate::memory::{ActiveInterrupt, Memory, SystemInfo};
use crate::registers::{flags, Registers};

pub struct System {
  pub registers: Registers,
  memory: Box<dyn Memory>,
  cycles_per_second: u64,
  cycle_count: u64,
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
    self.write(self.registers.sp.address(), value);
    self.registers.sp.push();
  }

  fn pop(&mut self) -> u8 {
    self.registers.sp.pop();
    let value = self.read(self.registers.sp.address());
    value
  }

  fn push_word(&mut self, value: u16) {
    self.push((value >> 8) as u8);
    self.push((value & 0xFF) as u8);
  }

  fn pop_word(&mut self) -> u16 {
    let lo = self.pop();
    let hi = self.pop();
    (hi as u16) << 8 | lo as u16
  }
}

pub trait InterruptHandler {
  fn interrupt(&mut self, maskable: bool, set_brk: bool);
}

impl InterruptHandler for System {
  fn interrupt(&mut self, maskable: bool, break_instr: bool) {
    if maskable && !break_instr && self.registers.sr.read(flags::INTERRUPT) {
      return;
    }

    self.push_word(self.registers.pc.address());

    if break_instr {
      self.push(self.registers.sr.get() | flags::BREAK);
    } else {
      self.push(self.registers.sr.get() & !flags::BREAK);
    }

    self.registers.sr.set(flags::INTERRUPT);

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
      cycle_count: 0,
    }
  }

  pub fn reset(&mut self) {
    self.memory.reset();
    self.registers.reset();
    let pc_address = self.read_word(0xFFFC);
    self.registers.pc.load(pc_address);
  }

  pub fn get_info(&self) -> SystemInfo {
    SystemInfo {
      cycles_per_second: self.cycles_per_second,
      cycle_count: self.cycle_count,
    }
  }

  pub fn tick(&mut self) -> f64 {
    let opcode = self.fetch();

    match self.execute(opcode) {
      Ok(cycles) => {
        self.cycle_count += cycles as u64;

        let info = self.get_info();

        match self.memory.poll(&info) {
          ActiveInterrupt::None => (),
          ActiveInterrupt::NMI => self.interrupt(false, false),
          ActiveInterrupt::IRQ => self.interrupt(true, false),
        }

        if self.cycles_per_second == 0 {
          0.0
        } else {
          cycles as f64 / self.cycles_per_second as f64
        }
      }
      Err(_) => {
        panic!(
          "Failed to execute instruction {:02x} at {:04x}",
          opcode,
          self.registers.pc.address()
        );
      }
    }
  }
}
