use crate::memory::{ActiveInterrupt, Memory};

// MOS 6520

// PORT: 8 individual lines
// DDR (Data Direction Register): each bit controls whether the line is an input (0) or output (1)
// Control Register:
//  bit 7: IRQ 1
//  bit 6: IRQ 2
//  bits 5, 4, 3: CA2 (interrupt status control)
//  bit 2: enable accessing DDR
//  bits 1, 0: CA1 (interrupt status control)

pub struct PIA {
  port_a: u8,
  ddr_a: u8,
  control_a: u8,
  port_b: u8,
  ddr_b: u8,
  control_b: u8,
}

impl PIA {
  pub fn new() -> Self {
    Self {
      port_a: 0xFF,
      ddr_a: 0,
      control_a: 0,
      port_b: 0xFF,
      ddr_b: 0,
      control_b: 0,
    }
  }
}

impl Memory for PIA {
  fn read(&self, address: u16) -> u8 {
    match address & 0b11 {
      0b00 => {
        print!("read from PORT A ");
        if self.control_a & 0b0000_0100 != 0 {
          println!("from PORT");
          self.port_a
        } else {
          println!("from DDR");
          self.ddr_a
        }
      }
      0b01 => {
        println!("read from CONTROL A");
        self.control_a
      }
      0b10 => {
        print!("read from PORT B ");
        if self.control_b & 0b0000_0100 != 0 {
          println!("from PORT");
          self.port_b
        } else {
          println!("from DDR");
          self.ddr_b
        }
      }
      0b11 => {
        println!("read from CONTROL B");
        self.control_b
      }
      _ => unreachable!(),
    }
  }

  fn write(&mut self, address: u16, value: u8) {
    match address & 0b11 {
      0b00 => {
        print!("write {} to PORT A ", value);
        if self.control_a & 0b0000_0100 != 0 {
          println!("to PORT");
          self.port_a = value & self.ddr_a;
        } else {
          println!("to DDR");
          self.ddr_a = value;
        }
      }
      0b01 => {
        println!("write {} to CONTROL A", value);
        self.control_a = value
      }
      0b10 => {
        print!("write {} to PORT B ", value);
        if self.control_b & 0b0000_0100 != 0 {
          println!("to PORT");
          self.port_b = value & self.ddr_b;
        } else {
          println!("to DDR");
          self.ddr_b = value;
        }
      }
      0b11 => {
        println!("write {} to CONTROL B", value);
        self.control_b = value
      }
      _ => unreachable!(),
    };
  }

  fn reset(&mut self) {
    self.port_a = 0xFF;
    self.ddr_a = 0;
    self.control_a = 0;
    self.port_b = 0xFF;
    self.ddr_b = 0;
    self.control_b = 0;
  }

  fn poll(&mut self) -> ActiveInterrupt {
    ActiveInterrupt::None
  }
}
