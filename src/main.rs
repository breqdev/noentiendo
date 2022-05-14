mod execute;
mod memory;
mod registers;
mod system;

use crate::system::MemoryIO;

fn main() {
  let memory = Box::new(memory::BlockMemory::new());
  let mut system = system::System::new(memory);

  // Set reset vector (0xFFFC) to program start (0x8000)
  system.write(0xFFFC, 0x00).unwrap();
  system.write(0xFFFD, 0x80).unwrap();

  // 6502 machine code
  let program = vec![
    /* 0x8000 */ 0xA9, 0xC0, // LDA #$C0
    /* 0x8002 */ 0xAA, // TAX
    /* 0x8003 */ 0xE8, // INX
    /* 0x8004 */ 0x69, 0xC4, // ADC #$C4
    /* 0x8006 */ 0x4C, 0x06, 0x80, // JMP $8006
  ];

  // Load program into memory at 0x8000
  for (i, &byte) in program.iter().enumerate() {
    system.write(0x8000 + i as u16, byte).unwrap();
  }

  // Reset system (set PC to reset vector)
  system.reset();

  // Run program for a while (contains an infinite loop)
  for _ in 0..100 {
    system.tick();
  }

  println!("A: {:02X}", system.registers.accumulator);
  println!("X: {:02X}", system.registers.x_index);
  println!("SR: {:08b}", system.registers.status_register);
  println!();
}
