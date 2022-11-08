use crate::memory::{ActiveInterrupt, Memory, SystemInfo};
use crate::platform::{Color, PlatformProvider, WindowConfig};
use crate::systems::vic::VicChip;
use std::sync::{Arc, Mutex};

const WIDTH: u32 = 22;
const HEIGHT: u32 = 23;
const CHAR_WIDTH: u32 = 8;
const CHAR_HEIGHT: u32 = 8;
const VRAM_SIZE: usize = 512; // 6 extra bytes to make mapping easier

pub struct VicVram {
  data: Vec<u8>,
  platform: Arc<dyn PlatformProvider>,
  chip: Arc<Mutex<VicChip>>,
}

impl VicVram {
  pub fn new(platform: Arc<dyn PlatformProvider>, chip: Arc<Mutex<VicChip>>) -> VicVram {
    platform.request_window(WindowConfig::new(
      WIDTH * CHAR_WIDTH,
      HEIGHT * CHAR_HEIGHT,
      2.0,
    ));

    VicVram {
      platform,
      data: vec![0; 0x0200],
      chip,
    }
  }
}

impl Memory for VicVram {
  fn read(&mut self, address: u16) -> u8 {
    self.data[address as usize]
  }

  fn write(&mut self, address: u16, value: u8) {
    println!("written to vram");
    self.data[address as usize] = value;

    if address >= (HEIGHT * WIDTH) as u16 {
      return; // ignore writes to the extra bytes
    }

    let column = (address % WIDTH as u16) as u32;
    let row = (address / WIDTH as u16) as u32;

    let character = self.chip.lock().unwrap().get_character(value);

    for line in 0..CHAR_HEIGHT {
      let line_data = character[line as usize];
      for pixel in 0..CHAR_WIDTH {
        let color = if line_data & (1 << (CHAR_WIDTH - 1 - pixel)) != 0 {
          // self.chip.lock().unwrap().get_foreground(address)
          Color::new(0, 0, 0)
        } else {
          self.chip.lock().unwrap().get_background()
        };

        self
          .platform
          .set_pixel(column * CHAR_WIDTH + pixel, row * CHAR_HEIGHT + line, color);
      }
    }
  }

  fn reset(&mut self) {
    self.data = vec![0; 0x0200];
  }

  fn poll(&mut self, _info: &SystemInfo) -> ActiveInterrupt {
    ActiveInterrupt::None
  }
}
