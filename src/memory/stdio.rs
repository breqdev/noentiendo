use crate::memory::{ActiveInterrupt, Memory};
use std::io::Write;

pub struct MappedStdIO {}

impl MappedStdIO {
  pub fn new() -> Self {
    Self {}
  }
}

impl Memory for MappedStdIO {
  // 0x00: u8 as dec
  // 0x01: char
  // 0x02: u8 as hex
  fn read(&mut self, address: u16) -> u8 {
    let mut input = String::new();
    print!("> ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).unwrap();

    match address & 0x03 {
      0x00 => input.trim().parse().expect("Invalid input for u8"),
      0x01 => {
        let char = input.chars().next().expect("String is empty");
        ((char as u32) & 0xFF) as u8
      }
      0x02 => u8::from_str_radix(&input.trim(), 16).expect("Invalid input for u8"),
      _ => unreachable!(),
    }
  }

  fn write(&mut self, address: u16, value: u8) {
    match address & 0x03 {
      0x00 => println!("{}", value),
      0x01 => println!("{}", value as char),
      0x02 => println!("{:02X}", value),
      0x03 => {
        print!("{}", value as char);
        std::io::stdout().flush().unwrap();
      }
      _ => unreachable!(),
    }
  }

  fn reset(&mut self) {}

  fn poll(&mut self) -> ActiveInterrupt {
    ActiveInterrupt::None
  }
}
