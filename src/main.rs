mod execute;
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
    /* 0x8005 */ 0x18, // CLC
    /* 0x8006 */ 0x69, 0x10, // ADC #$10
    /* 0x8008 */ 0x8D, 0x01, 0x02, // STA $0201
    /* 0x800B */ 0x38, // SEC
    /* 0x800C */ 0xE9, 0x0D, // SBC #$0D
    /* 0x800E */ 0x8D, 0x02, 0x02, // STA $0202
    /* 0x8011 */ 0xA9, 0x51, // LDA #$51
    /* 0x8013 */ 0x8D, 0x03, 0x02, // STA $0203
    /* 0x8016 */ 0x4C, 0x16, 0x80, // JMP $8016
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
