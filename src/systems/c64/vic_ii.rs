use crate::memory::{ActiveInterrupt, Memory};
use crate::platform::{Color, WindowConfig};
use std::cell::RefCell;
use std::rc::Rc;

const WIDTH: u32 = 40;
const HEIGHT: u32 = 25;
const CHAR_WIDTH: u32 = 8;
const CHAR_HEIGHT: u32 = 8;
const SPRITE_WIDTH: u32 = 24;
const SPRITE_HEIGHT: u32 = 21;
const BORDER_WIDTH: u32 = 24;
const BORDER_HEIGHT: u32 = 29;
pub const FULL_WIDTH: u32 = WIDTH * CHAR_WIDTH + BORDER_WIDTH * 2;
pub const FULL_HEIGHT: u32 = HEIGHT * CHAR_HEIGHT + BORDER_HEIGHT * 2;
const SPRITE_MEMORY_SIZE: u16 = (SPRITE_WIDTH * SPRITE_HEIGHT / 8) as u16;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Sprite {
  pub x: u16,
  pub y: u8,
  pub color: u8,
  pub enabled: bool,
  pub x_expansion: bool,
  pub y_expansion: bool,
  pub multicolor: bool,
  pub data_priority: bool,
}

impl Sprite {
  pub fn new() -> Self {
    Self {
      x: 0,
      y: 0,
      color: 0,
      enabled: false,
      x_expansion: false,
      y_expansion: false,
      multicolor: false,
      data_priority: false,
    }
  }
}

#[allow(dead_code)]
mod interrupt_bits {
  pub const RASTER: u8 = 1 << 0;
  pub const SPRITE_SPRITE_COLLISION: u8 = 1 << 1;
  pub const SPRITE_BACKGROUND_COLLISION: u8 = 1 << 2;
  pub const LIGHT_PEN: u8 = 1 << 3;
}

pub struct VicIIChip {
  character_rom: Box<dyn Memory>,

  sprites: [Sprite; 8],

  background_color: [u8; 4],

  sprite_multicolor: [u8; 2],

  border_color: u8,

  light_pen: (u8, u8), // (x, y)

  raster_counter: u16,

  interrupts_enabled: u8,

  extended_color_mode: bool,
  bit_map_mode: bool,
  multi_color_mode: bool,
  display_enable: bool,
  res_bit: bool,

  row_select: bool,
  column_select: bool,
  x_scroll: u8,
  y_scroll: u8,
}

impl VicIIChip {
  pub fn new(character_rom: Box<dyn Memory>) -> Self {
    Self {
      character_rom,
      sprites: [Sprite::new(); 8],
      background_color: [0; 4],
      sprite_multicolor: [0; 2],
      border_color: 0,
      light_pen: (0, 0),
      raster_counter: 0,
      interrupts_enabled: 0,
      extended_color_mode: false,
      bit_map_mode: false,
      multi_color_mode: false,
      display_enable: false,
      res_bit: false,
      row_select: false,
      column_select: false,
      x_scroll: 0,
      y_scroll: 0,
    }
  }

  pub fn reset(&mut self) {
    self.character_rom.reset();
    self.sprites = [Sprite::new(); 8];
    self.background_color = [0; 4];
    self.sprite_multicolor = [0; 2];
    self.border_color = 0;
    self.light_pen = (0, 0);
    self.raster_counter = 0;
    self.interrupts_enabled = 0;
    self.extended_color_mode = false;
    self.bit_map_mode = false;
    self.multi_color_mode = false;
    self.display_enable = false;
    self.res_bit = false;
    self.row_select = false;
    self.column_select = false;
    self.x_scroll = 0;
    self.y_scroll = 0;
  }

  /// Read the value of the screen memory at the given address,
  /// respecting the mapping defined in the VIC registers.
  fn read_vram(&self, address: u16, memory: &mut Box<dyn Memory>) -> u8 {
    let offset = 0x0400;

    memory.read(address + offset)
  }

  /// Read the value of the color memory at the given address,
  /// respecting the mapping defined in the VIC registers.
  fn read_color(&self, address: u16, memory: &mut Box<dyn Memory>) -> u8 {
    let offset = 0xD800;

    memory.read(address + offset)
  }

  /// Get the bits in the character at the given value.
  /// The character is 8 bits wide and 8 bits tall, so this returns a vec![0; 8].
  /// Each byte is a horizontal row, which are ordered from top to bottom.
  /// Bits are ordered with the MSB at the left and the LSB at the right.
  fn get_character(&mut self, value: u8, _memory: &mut Box<dyn Memory>) -> Vec<u8> {
    let character_index = (value as u16) * 8;

    let mut character = vec![0; 8];
    for (i, row) in character.iter_mut().enumerate() {
      *row = self.character_rom.read(character_index + i as u16);
    }

    character
  }

  fn get_color(value: u8) -> Color {
    match value & 0xF {
      0x0 => Color::new(0x00, 0x00, 0x00),
      0x1 => Color::new(0xFF, 0xFF, 0xFF),
      0x2 => Color::new(0x88, 0x00, 0x00),
      0x3 => Color::new(0xAA, 0xFF, 0xEE),
      0x4 => Color::new(0xCC, 0x44, 0xCC),
      0x5 => Color::new(0x00, 0xCC, 0x55),
      0x6 => Color::new(0x00, 0x00, 0xAA),
      0x7 => Color::new(0xEE, 0xEE, 0x77),
      0x8 => Color::new(0xDD, 0x88, 0x55),
      0x9 => Color::new(0x66, 0x44, 0x00),
      0xA => Color::new(0xFF, 0x77, 0x77),
      0xB => Color::new(0x33, 0x33, 0x33),
      0xC => Color::new(0x77, 0x77, 0x77),
      0xD => Color::new(0xAA, 0xFF, 0x66),
      0xE => Color::new(0x00, 0x88, 0xFF),
      0xF => Color::new(0xBB, 0xBB, 0xBB),
      _ => unreachable!(),
    }
  }

  /// Get the foreground color to be shown at the given character position.
  fn get_foreground(&mut self, address: u16, memory: &mut Box<dyn Memory>) -> Color {
    VicIIChip::get_color(self.read_color(address, memory))
  }

  /// Draw the given sprite.
  /// Sources: <https://retro64.altervista.org/blog/programming-sprites-the-commodore-64-simple-tutorial-using-basic-v2/> and <https://dustlayer.com/vic-ii/2013/4/28/vic-ii-for-beginners-part-5-bringing-sprites-in-shape>
  fn draw_sprite(
    &mut self,
    index: usize,
    memory: &mut Box<dyn Memory>,
    framebuffer: &mut [u8],
    _config: WindowConfig,
  ) {
    let sprite = &self.sprites[index];

    if !sprite.enabled {
      return;
    }

    let data_pointer = 0x07F8 + index as u16;
    let data_address = memory.read(data_pointer) as u16 * 64;

    for byte_index in 0..SPRITE_MEMORY_SIZE {
      let data_byte = memory.read(data_address + byte_index);

      for bit_index in 0..8 {
        let color = if data_byte & (1 << (7 - bit_index)) != 0 {
          sprite.color
        } else {
          self.background_color[0]
        };

        let position = (byte_index * 8 + bit_index) as u32;
        let y = position / SPRITE_WIDTH;
        let x = position % SPRITE_WIDTH;

        let x = x + sprite.x as u32;
        let y = y + sprite.y as u32;

        let index = (y * WIDTH + x) as usize * 4;
        let pixel = &mut framebuffer[index..index + 4];
        let color = VicIIChip::get_color(color);
        pixel.copy_from_slice(&color.to_rgba());
      }
    }
  }

  /// Redraw the character at the specified address.
  fn redraw(
    &mut self,
    address: u16,
    memory: &mut Box<dyn Memory>,
    framebuffer: &mut [u8],
    _config: WindowConfig,
  ) {
    if address >= (WIDTH * HEIGHT) as u16 {
      return; // ignore writes to the extra bytes
    }

    let column = (address % WIDTH as u16) as u32;
    let row = (address / WIDTH as u16) as u32;

    let value = self.read_vram(address, memory);
    let character = self.get_character(value, memory);

    for line in 0..CHAR_HEIGHT {
      let line_data = character[line as usize];
      for pixel in 0..CHAR_WIDTH {
        let color = if line_data & (1 << (CHAR_WIDTH - 1 - pixel)) != 0 {
          self.get_foreground(address, memory)
        } else {
          VicIIChip::get_color(self.background_color[0])
        };

        let x = BORDER_WIDTH + column * CHAR_WIDTH + pixel;
        let y = BORDER_HEIGHT + row * CHAR_HEIGHT + line;
        let index = (y * FULL_WIDTH + x) as usize * 4;
        let pixel = &mut framebuffer[index..index + 4];
        pixel.copy_from_slice(&color.to_rgba());
      }
    }
  }

  pub fn draw_screen(
    &mut self,
    memory: &mut Box<dyn Memory>,
    framebuffer: &mut [u8],
    config: WindowConfig,
  ) {
    // draw the border
    for x in 0..FULL_WIDTH {
      for y in 0..FULL_HEIGHT {
        let index = (y * FULL_WIDTH + x) as usize * 4;
        let pixel = &mut framebuffer[index..(index + 4)];
        let color = VicIIChip::get_color(self.border_color);
        pixel.copy_from_slice(&color.to_rgba());
      }
    }

    for i in 0..((WIDTH * HEIGHT) as u16) {
      self.redraw(i, memory, framebuffer, config);
    }

    for i in 0..8 {
      self.draw_sprite(i, memory, framebuffer, config);
    }
  }
}

/// Represents the I/O mapping for the MOS 6560 VIC.
/// Sources: <https://www.c64-wiki.com/wiki/Page_208-211>
/// and <http://www.zimmers.net/cbmpics/cbm/c64/vic-ii.txt>.
pub struct VicIIChipIO {
  chip: Rc<RefCell<VicIIChip>>,
}

impl VicIIChipIO {
  pub fn new(chip: Rc<RefCell<VicIIChip>>) -> Self {
    Self { chip }
  }
}

impl Memory for VicIIChipIO {
  fn read(&mut self, address: u16) -> u8 {
    let chip = self.chip.borrow();

    match address % 0x40 {
      0x00..=0x0F => {
        let sprite_index = (address % 0x40 / 2) as usize;

        match sprite_index % 2 {
          0 => chip.sprites[sprite_index].x as u8,
          1 => chip.sprites[sprite_index].y,
          _ => unreachable!(),
        }
      }
      0x10 => chip.sprites.iter().enumerate().fold(0, |acc, (i, sprite)| {
        let sprite_msb = ((sprite.x & 0b1_0000_0000) >> 8) as u8;
        let shifted = sprite_msb << i;
        acc | shifted
      }),
      0x11 => {
        ((chip.raster_counter & 0x100 >> 8) as u8) << 7
          | (chip.extended_color_mode as u8) << 6
          | (chip.bit_map_mode as u8) << 5
          | (chip.display_enable as u8) << 4
          | (chip.row_select as u8) << 3
          | chip.y_scroll & 0b111
      }
      0x12 => chip.raster_counter as u8,
      0x13 => chip.light_pen.0,
      0x14 => chip.light_pen.1,
      0x15 => chip
        .sprites
        .iter()
        .enumerate()
        .fold(0, |acc, (i, sprite)| acc | ((sprite.enabled as u8) << i)),
      0x16 => {
        0b1100_0000
          | (chip.res_bit as u8) << 5
          | (chip.multi_color_mode as u8) << 4
          | (chip.column_select as u8) << 3
          | chip.x_scroll & 0b111
      }
      0x17 => chip.sprites.iter().enumerate().fold(0, |acc, (i, sprite)| {
        acc | ((sprite.y_expansion as u8) << i)
      }),
      0x18 => 0, // TODO: memory expansion
      0x19 => 0, // TODO: interrupt flags
      0x1A => 0xF0 | chip.interrupts_enabled,
      0x1B => chip.sprites.iter().enumerate().fold(0, |acc, (i, sprite)| {
        acc | ((sprite.data_priority as u8) << i)
      }),
      0x1C => chip
        .sprites
        .iter()
        .enumerate()
        .fold(0, |acc, (i, sprite)| acc | ((sprite.multicolor as u8) << i)),
      0x1D => chip.sprites.iter().enumerate().fold(0, |acc, (i, sprite)| {
        acc | ((sprite.x_expansion as u8) << i)
      }),
      0x1E => 0, // TODO: sprite-sprite collision
      0x1F => 0, // TODO: sprite-data collision
      0x20 => 0xF0 | chip.border_color,
      0x21..=0x24 => 0xF0 | chip.background_color[(address % 0x40) as usize - 0x21],
      0x25..=0x26 => 0xF0 | chip.sprite_multicolor[(address % 0x40) as usize - 0x25],
      0x27..=0x2E => 0xF0 | chip.sprites[(address % 0x40) as usize - 0x27].color,
      0x2F..=0x3F => 0xFF,
      _ => unreachable!(),
    }
  }

  fn write(&mut self, address: u16, value: u8) {
    let mut chip = self.chip.borrow_mut();

    match address % 0x40 {
      0x00..=0x0F => {
        let sprite_index = (address % 0x40 / 2) as usize;

        match sprite_index % 2 {
          0 => {
            chip.sprites[sprite_index].x = (chip.sprites[sprite_index].x & 0x100) | (value as u16)
          }
          1 => chip.sprites[sprite_index].y = value,
          _ => unreachable!(),
        }
      }
      0x10 => {
        for i in 0..8 {
          let x_msb = value & (1 << i);
          chip.sprites[i].x = (chip.sprites[i].x & 0xFF) | (x_msb as u16) << 8;
        }
      }
      0x11 => {
        chip.extended_color_mode = (value & 0b0100_0000) != 0;
        chip.bit_map_mode = (value & 0b0010_0000) != 0;
        chip.display_enable = (value & 0b0001_0000) != 0;
        chip.row_select = (value & 0b0000_1000) != 0;
        chip.y_scroll = value & 0b0000_0111;
      }
      0x12 => {
        // TODO: set raster comparator to trigger interrupt
      }
      0x13 => chip.light_pen.0 = value,
      0x14 => chip.light_pen.1 = value,
      0x15 => {
        for i in 0..8 {
          chip.sprites[i].enabled = (value & (1 << i)) != 0;
        }
      }
      0x16 => {
        chip.res_bit = (value & 0b0010_0000) != 0;
        chip.multi_color_mode = (value & 0b0001_0000) != 0;
        chip.column_select = (value & 0b0000_1000) != 0;
        chip.x_scroll = value & 0b0000_0111;
      }
      0x17 => {
        for i in 0..8 {
          chip.sprites[i].y_expansion = (value & (1 << i)) != 0;
        }
      }
      0x18 => {} // TODO: memory expansion
      0x19 => {} // TODO: interrupt flags
      0x1A => chip.interrupts_enabled = value & 0x0F,
      0x1B => {
        for i in 0..8 {
          chip.sprites[i].data_priority = (value & (1 << i)) != 0;
        }
      }
      0x1C => {
        for i in 0..8 {
          chip.sprites[i].multicolor = (value & (1 << i)) != 0;
        }
      }
      0x1D => {
        for i in 0..8 {
          chip.sprites[i].x_expansion = (value & (1 << i)) != 0;
        }
      }
      0x1E => {}
      0x1F => {}
      0x20 => chip.border_color = value & 0x0F,
      0x21..=0x24 => chip.background_color[(address % 0x40) as usize - 0x21] = value & 0x0F,
      0x25..=0x26 => chip.sprite_multicolor[(address % 0x40) as usize - 0x25] = value & 0x0F,
      0x27..=0x2E => chip.sprites[(address % 0x40) as usize - 0x27].color = value & 0x0F,
      0x2F..=0x3F => {} // no-op
      _ => unreachable!(),
    }
  }

  fn reset(&mut self) {
    self.chip.borrow_mut().reset();
  }

  fn poll(&mut self, _cycles_since_poll: u64, total_cycle_count: u64) -> ActiveInterrupt {
    self.chip.borrow_mut().raster_counter = ((total_cycle_count / 83) % 312) as u16;

    ActiveInterrupt::None
  }
}
