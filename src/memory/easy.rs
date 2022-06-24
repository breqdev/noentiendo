use crate::graphics::{Color, GraphicsProvider};
use crate::memory::Memory;

// Easy6502 bitmap screen memory
// https://skilldrick.github.io/easy6502/

pub struct EasyMemory {
  width_bits: u8,
  height_bits: u8,
  mask: u16,
  data: Vec<u8>,
  graphics: Box<dyn GraphicsProvider>,
  palette: Vec<Color>,
}

const SCALE: u32 = 8;

impl EasyMemory {
  pub fn new(width_bits: u8, height_bits: u8, graphics: Box<dyn GraphicsProvider>) -> Self {
    let mut graphics = graphics;
    graphics.create_window((1 << width_bits) * SCALE, (1 << height_bits) * SCALE);

    let palette = [
      0x000000, 0xffffff, 0x880000, 0xaaffee, 0xcc44cc, 0x00cc55, 0x0000aa, 0xeeee77, 0xdd8855,
      0x664400, 0xff7777, 0x333333, 0x777777, 0xaaff66, 0x0088ff, 0xbbbbbb,
    ];

    let palette = palette
      .iter()
      .map(|&c| Color::new((c >> 16) as u8, (c >> 8) as u8, c as u8))
      .collect();

    Self {
      width_bits,
      height_bits,
      mask: (1 << (width_bits + height_bits)) - 1,
      data: vec![0; ((1 << width_bits) * (1 << height_bits)) as usize],
      graphics,
      palette,
    }
  }
}

impl Memory for EasyMemory {
  fn read(&self, address: u16) -> u8 {
    self.data[(address & self.mask) as usize]
  }

  fn write(&mut self, address: u16, value: u8) {
    self.data[(address & self.mask) as usize] = value;
  }

  fn tick(&mut self) {
    for i in 0..self.data.len() {
      let x_base = (i & ((1 << self.width_bits) - 1)) as u32 * SCALE;
      let y_base = (i >> self.width_bits) as u32 * SCALE;
      let color = self.palette[(self.data[i] as usize) % self.palette.len()];

      for x in 0..SCALE {
        for y in 0..SCALE {
          self.graphics.set_pixel(x_base + x, y_base + y, color);
        }
      }
    }

    self.graphics.tick();
  }

  fn reset(&mut self) {
    for i in 0..self.data.len() {
      self.data[i] = 0;
    }
  }
}
