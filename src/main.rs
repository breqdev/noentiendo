mod execute;
mod fetch;
mod memory;
mod registers;
mod system;

use crate::system::MemoryIO;

fn main() {
  let ram = Box::new(memory::BlockMemory::new(14));
  let io = Box::new(memory::MappedIO::new());
  let rom = Box::new(memory::BlockMemory::new(15));

  let low = Box::new(memory::BranchMemory::new(ram, io, 15));
  let memory = Box::new(memory::BranchMemory::new(low, rom, 16));

  let mut system = system::System::new(memory);

  // Set reset vector (0xFFFC) to program start (0x8000)
  system.write(0xFFFC, 0x00);
  system.write(0xFFFD, 0x80);

  // 6502 machine code
  let program = vec![
    /* 0x8000 */ 0xA2, 0x01, // LDX #$01
    /* 0x8002 */ 0x86, 0x00, // STX $00
    /* 0x8004 */ 0xA9, 0x02, // LDA #$02
    /* 0x8006 */ 0x85, 0x01, // STA $01
    /* 0x8008 */ 0xA6, 0x01, // LDA $01
    /* 0x800A */ 0x65, 0x00, // ADC $00
    /* 0x800C */ 0x85, 0x01, // STA $01
    /* 0x800E */ 0x8D, 0x00, 0x40, // STA $4000
    /* 0x800E */ 0x86, 0x00, // STX $00
    /* 0x8010 */ 0x4C, 0x08, 0x80, // JMP $8008
  ];

  // Load program into memory at 0x8000
  for (i, &byte) in program.iter().enumerate() {
    system.write(0x8000 + i as u16, byte);
  }

  // Reset system (set PC to reset vector)
  system.reset();

  // Run program for a while (contains an infinite loop)
  for _ in 0..50 {
    system.tick();
  }
}
