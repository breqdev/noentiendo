mod memory;
mod registers;
mod system;

fn main() {
  let memory = Box::new(memory::BlockMemory::new());
  let mut system = system::System::new(memory);

  // Set reset vector (0xFFFC) to program start (0x8000)
  system.write(0xFFFC, 0x00).unwrap();
  system.write(0xFFFD, 0x80).unwrap();

  // 6502 machine code
  let program = vec![
    /* 0x8000 */ 0xA9, 0x42, // LDA #$42
    /* 0x8002 */ 0x8D, 0x00, 0x02, // STA $0200
    /* 0x8005 */ 0xA9, 0x52, // LDA #$52
    /* 0x8007 */ 0x8D, 0x01, 0x02, // STA $0201
    /* 0x800A */ 0xA9, 0x45, // LDA #$45
    /* 0x800C */ 0x8D, 0x02, 0x02, // STA $0202
    /* 0x800F */ 0xA9, 0x51, // LDA #$51
    /* 0x8011 */ 0x8D, 0x03, 0x02, // STA $0203
    /* 0x8014 */ 0x4c, 0x14, 0x80, // JMP $8014
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

  // Dump contents of memory from 0x0200 to 0x0204
  for i in 0x0200..0x0204 {
    print!("{}", system.read(i).unwrap() as char);
  }
  println!();
}
