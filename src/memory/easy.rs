use crate::graphics::{Color, GraphicsProvider};
use crate::memory::Memory;
use rand::random;

// Easy6502 bitmap screen memory
// https://skilldrick.github.io/easy6502/

pub struct EasyVram {
  width: u32,
  height: u32,
  data: Vec<u8>,
  graphics: Box<dyn GraphicsProvider>,
  palette: Vec<Color>,
}

const SCALE: u32 = 8;

impl EasyVram {
  pub fn new(width: u32, height: u32, graphics: Box<dyn GraphicsProvider>) -> Self {
    let mut graphics = graphics;
    graphics.create_window(width * SCALE, height * SCALE);

    let palette = [
      0x000000, 0xffffff, 0x880000, 0xaaffee, 0xcc44cc, 0x00cc55, 0x0000aa, 0xeeee77, 0xdd8855,
      0x664400, 0xff7777, 0x333333, 0x777777, 0xaaff66, 0x0088ff, 0xbbbbbb,
    ];

    let palette = palette
      .iter()
      .map(|&c| Color::new((c >> 16) as u8, (c >> 8) as u8, c as u8))
      .collect();

    Self {
      width,
      height,
      data: vec![0; (width * height) as usize],
      graphics,
      palette,
    }
  }
}

impl Memory for EasyVram {
  fn read(&self, address: u16) -> u8 {
    self.data[((address as u32) % (self.width * self.height)) as usize]
  }

  fn write(&mut self, address: u16, value: u8) {
    self.data[((address as u32) % (self.width * self.height)) as usize] = value;
  }

  fn tick(&mut self) {
    for i in 0..self.data.len() {
      let x_base = (i % self.width as usize) as u32 * SCALE;
      let y_base = (i / self.width as usize) as u32 * SCALE;
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

pub struct EasyIO {
  last_key: u8,
}

impl EasyIO {
  pub fn new() -> Self {
    Self { last_key: 0 }
  }
}

impl Memory for EasyIO {
  fn read(&self, address: u16) -> u8 {
    match address % 2 {
      0 => random::<u8>(),
      _ => self.last_key,
    }
  }

  fn write(&mut self, _address: u16, _value: u8) {}

  fn tick(&mut self) {}

  fn reset(&mut self) {}
}
