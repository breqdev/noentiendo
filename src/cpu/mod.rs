mod execute;
mod fetch;
mod registers;
use crate::memory::{ActiveInterrupt, Memory, SystemInfo, DMA};
use execute::Execute;
use fetch::Fetch;
use registers::{flags, Registers};

/// The MOS 6502 CPU and its associated memory.
pub struct System {
  pub registers: Registers,
  memory: Box<dyn Memory>,
  cycles_per_second: u64,
  cycle_count: u64,
  dma: Vec<Box<dyn DMA>>,
}

/// Read and write from the system's memory.
pub trait MemoryIO {
  /// Read a byte from the given address in memory.
  fn read(&mut self, address: u16) -> u8;

  /// Write a byte to the given address in memory.
  fn write(&mut self, address: u16, value: u8);

  /// Read a word (little-endian) from the given address in memory.
  fn read_word(&mut self, address: u16) -> u16;

  /// Write a word (little-endian) to the given address in memory.
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

/// Push and pop values from the stack.
pub trait Stack {
  /// Push a byte onto the stack.
  fn push(&mut self, value: u8);

  /// Pop a byte from the stack.
  fn pop(&mut self) -> u8;

  /// Push a word (little-endian) onto the stack.
  fn push_word(&mut self, value: u16);

  /// Pop a word (little-endian) from the stack.
  fn pop_word(&mut self) -> u16;
}

impl Stack for System {
  fn push(&mut self, value: u8) {
    self.write(self.registers.sp.address(), value);
    self.registers.sp.push();
  }

  fn pop(&mut self) -> u8 {
    self.registers.sp.pop();
    self.read(self.registers.sp.address())
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

/// Handle interrupts by setting the applicable flags, pushing the program counter
/// onto the stack, and loading the interrupt vector into the program counter.
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
      dma: Vec::new(),
    }
  }

  /// Attach a device which can perform DMA access.
  pub fn attach_dma(&mut self, dma: Box<dyn DMA>) {
    self.dma.push(dma);
  }

  pub fn reset(&mut self) {
    self.memory.reset();
    self.registers.reset();
    let pc_address = self.read_word(0xFFFC);
    self.registers.pc.load(pc_address);
  }

  /// Return a SystemInfo struct containing the current system status.
  pub fn get_info(&self) -> SystemInfo {
    SystemInfo {
      cycles_per_second: self.cycles_per_second,
      cycle_count: self.cycle_count,
    }
  }

  /// Execute a single instruction.
  pub fn tick(&mut self) -> f64 {
    let opcode = self.fetch();
    let elapsed;

    match self.execute(opcode) {
      Ok(cycles) => {
        self.cycle_count += cycles as u64;

        let info = self.get_info();

        for _ in 0..cycles {
          match self.memory.poll(&info) {
            ActiveInterrupt::None => (),
            ActiveInterrupt::NMI => self.interrupt(false, false),
            ActiveInterrupt::IRQ => self.interrupt(true, false),
          }
        }

        if self.cycles_per_second == 0 {
          elapsed = 0.0
        } else {
          elapsed = cycles as f64 / self.cycles_per_second as f64
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

    let info = self.get_info();

    for dma in &mut self.dma {
      dma.dma(&mut self.memory, &info);
    }

    elapsed
  }
}
