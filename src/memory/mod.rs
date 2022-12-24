mod banked;
mod block;
mod branch;
pub mod interface;
mod null;
pub mod ports;

pub use banked::BankedMemory;
pub use block::BlockMemory;
pub use branch::BranchMemory;
pub use null::NullMemory;
pub use ports::{NullPort, Port};

/// Represents the state of the interrupts on the system.
#[derive(Debug, PartialEq, Eq)]
pub enum ActiveInterrupt {
  /// No interrupts are active.
  None,
  /// An NMI (non-maskable interrupt) is active.
  NMI,
  /// An IRQ (maskable interrupt request) is active.
  IRQ,
}

/// Information about the system that Memory implementations can use to
/// determine if an interrupt should be triggered.
#[derive(Debug, Default)]
pub struct SystemInfo {
  pub cycles_per_second: u64,
  pub cycle_count: u64,
}

/// Represents a contiguous block of memory which can be read, written,
/// reset, and polled to see if an interrupt has been triggered.
pub trait Memory {
  /// Read a byte from this memory at the given address.
  /// Implementations may trigger side effects as a result of this read.
  fn read(&mut self, address: u16) -> u8;

  /// Write a byte to this memory at the given address.
  fn write(&mut self, address: u16, value: u8);

  /// Reset this memory to its initial state, e.g. after a system reboot.
  /// Sometimes this will clear the contents of the memory, like with RAM.
  /// Other times this is a no-op, e.g. for ROM.
  fn reset(&mut self);

  /// Poll this memory to see if an interrupt has been triggered.
  /// Implementations may trigger an NMI or IRQ for any
  /// implementation-dependent reason.
  fn poll(&mut self, info: &SystemInfo) -> ActiveInterrupt;
}

/// Represents a system component which may access the memory in between
/// cycles. This is used to implement the VIC chip's alternate-clock memory
/// access.
pub trait DMA {
  /// Handle DMA for the current instruction tick.
  /// Implementations may perform whatever operations on the memory that they
  /// would like.
  fn dma(&mut self, memory: &mut Box<dyn Memory>, info: &SystemInfo);
}
