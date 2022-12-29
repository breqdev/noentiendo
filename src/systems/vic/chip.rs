use crate::memory::{ActiveInterrupt, Memory, SystemInfo};
use crate::platform::{Color, PlatformProvider, WindowConfig};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

/// One of the speakers available on the MOS 6560 VIC.
struct VicChipSpeaker {
  on: bool,
  note: u8,
}

impl VicChipSpeaker {
  fn new() -> Self {
    Self { on: false, note: 0 }
  }

  fn read(&self) -> u8 {
    self.note | (self.on as u8) << 7
  }

  fn write(&mut self, value: u8) {
    self.on = (value & 0x80) != 0;
    self.note = value & 0x7f;
  }

  fn reset(&mut self) {
    self.on = false;
    self.note = 0;
  }
}

/// The light pen input available on the MOS 6560 VIC.
struct VicChipLightPen {
  x: u8,
  y: u8,
}

impl VicChipLightPen {
  fn new() -> Self {
    Self { x: 0, y: 0 }
  }

  fn read_x(&self) -> u8 {
    self.x
  }

  fn read_y(&self) -> u8 {
    self.y
  }

  fn write_x(&mut self, value: u8) {
    self.x = value;
  }

  fn write_y(&mut self, value: u8) {
    self.y = value;
  }

  fn reset(&mut self) {
    self.x = 0;
    self.y = 0;
  }
}

/// The MOS 6560 VIC (Video Interface Chip).
/// Uses VRAM memory, character memory, and color memory to draw the screen.
/// Also handles the speakers and light pen.
/// Source: <http://tinyvga.com/6561>
pub struct VicChip {
  platform: Arc<dyn PlatformProvider>,
  // Registers

  // TV scan settings
  scan_mode: bool,

  // Screen alignment
  left_draw_offset: u8,
  top_draw_offset: u8,

  // Character size
  row_count: u8,
  column_count: u8,
  double_size_chars: bool,

  // Screen drawing
  raster_counter: u16,

  // Memory mapping
  vram_address_top: u8,
  character_address_top: u8,
  color_ram_mapping: bool,

  // Light pen
  light_pen: VicChipLightPen,

  // Potentiometers
  potentiometer_1: u8,
  potentiometer_2: u8,

  // Speakers
  speaker_alto: VicChipSpeaker,
  speaker_tenor: VicChipSpeaker,
  speaker_soprano: VicChipSpeaker,
  speaker_noise: VicChipSpeaker,
  speaker_volume: u8,

  // Colors
  aux_color: u8,
  border_color: u8,
  reverse_field: bool,
  background_color: u8,
}

impl VicChip {
  pub fn new(platform: Arc<dyn PlatformProvider>) -> Self {
    let width: u8 = 22;
    let height: u8 = 23;

    platform.request_window(WindowConfig::new(width as u32 * 8, height as u32 * 8, 2.0));

    Self {
      platform,

      scan_mode: false,
      left_draw_offset: 12,
      top_draw_offset: 38,
      column_count: width,
      color_ram_mapping: true,
      raster_counter: 0,
      row_count: height,
      double_size_chars: false,
      vram_address_top: 0,
      character_address_top: 0,
      light_pen: VicChipLightPen::new(),
      potentiometer_1: 0xFF,
      potentiometer_2: 0xFF,
      speaker_alto: VicChipSpeaker::new(),
      speaker_tenor: VicChipSpeaker::new(),
      speaker_soprano: VicChipSpeaker::new(),
      speaker_noise: VicChipSpeaker::new(),
      speaker_volume: 0,
      aux_color: 0,
      border_color: 3,
      reverse_field: true,
      background_color: 1,
    }
  }

  pub fn reset(&mut self) {
    self.scan_mode = false;
    self.left_draw_offset = 12;
    self.top_draw_offset = 38;
    self.column_count = 22;
    self.color_ram_mapping = true;
    self.raster_counter = 0;
    self.row_count = 23;
    self.double_size_chars = false;
    self.vram_address_top = 15;
    self.light_pen.reset();
    self.potentiometer_1 = 0xFF;
    self.potentiometer_2 = 0xFF;
    self.speaker_alto.reset();
    self.speaker_tenor.reset();
    self.speaker_soprano.reset();
    self.speaker_noise.reset();
    self.speaker_volume = 0;
    self.aux_color = 0;
    self.border_color = 3;
    self.reverse_field = true;
    self.background_color = 1;
    self.character_address_top = 0;
  }

  /// The Vic-20 only has 14 address lines, see:
  /// <http://sleepingelephant.com/~sleeping/ipw-web/bulletin/bb/viewtopic.php?t=9928#p111327>
  fn vic_to_cpu_address(address: u16) -> u16 {
    // 0x0000 -> 0x8000
    // 0x0FFF -> 0x8FFF
    // 0x1000 -> 0x9000
    // 0x1FFF -> 0x9FFF
    // 0x2000 -> 0x0000
    // 0x2FFF -> 0x0FFF

    #[allow(clippy::identity_op)]
    match address & (1 << 13) {
      0 => 0x8000 + (address & 0x1fff),
      _ => 0x0000 + (address & 0x1fff),
    }
  }

  /// Read the value of the screen memory at the given address,
  /// respecting the mapping defined in the VIC registers.
  fn read_vram(&self, address: u16, memory: &mut Box<dyn Memory>) -> u8 {
    let mut offset = (self.vram_address_top as u16) << 10;
    offset += (self.color_ram_mapping as u16) << 9;

    memory.read(VicChip::vic_to_cpu_address(address + offset))
  }

  /// Read the value of the color memory at the given address,
  /// respecting the mapping defined in the VIC registers.
  fn read_color(&self, address: u16, memory: &mut Box<dyn Memory>) -> u8 {
    let offset = if self.color_ram_mapping {
      0x1600
    } else {
      0x1400
    };

    memory.read(VicChip::vic_to_cpu_address(address + offset))
  }

  /// Read the value of the character memory at the given address,
  /// respecting the mapping defined in the VIC registers.
  fn read_character(&self, address: u16, memory: &mut Box<dyn Memory>) -> u8 {
    let offset = (self.character_address_top as u16) << 10;

    memory.read(VicChip::vic_to_cpu_address(address + offset))
  }

  /// Get the bits in the character at the given value.
  /// The character is 8 bits wide and 8 bits tall, so this returns a vec![0; 8].
  /// Each byte is a horizontal row, which are ordered from top to bottom.
  /// Bits are ordered with the MSB at the left and the LSB at the right.
  fn get_character(&mut self, value: u8, memory: &mut Box<dyn Memory>) -> Vec<u8> {
    let character_index = (value as u16) * 8;

    let mut character = vec![0; 8];
    for (i, row) in character.iter_mut().enumerate() {
      *row = self.read_character(character_index + i as u16, memory);
    }

    character
  }

  /// Get the foreground color to be shown at the given character position.
  fn get_foreground(&mut self, address: u16, memory: &mut Box<dyn Memory>) -> Color {
    let value = self.read_color(address, memory);
    match value & 0b111 {
      0b000 => Color::new(0, 0, 0),
      0b001 => Color::new(255, 255, 255),
      0b010 => Color::new(255, 0, 0),
      0b011 => Color::new(0, 255, 255),
      0b100 => Color::new(255, 0, 255),
      0b101 => Color::new(0, 255, 0),
      0b110 => Color::new(0, 0, 255),
      0b111 => Color::new(255, 255, 0),
      _ => unreachable!(),
    }
  }

  /// Get the current background color being shown.
  fn get_background(&self) -> Color {
    match self.background_color & 0b1111 {
      0b0000 => Color::new(0, 0, 0),
      0b0001 => Color::new(255, 255, 255),
      0b0010 => Color::new(255, 0, 0),
      0b0011 => Color::new(0, 255, 255),
      0b0100 => Color::new(255, 0, 255),
      0b0101 => Color::new(0, 255, 0),
      0b0110 => Color::new(0, 0, 255),
      0b0111 => Color::new(255, 255, 0),
      0b1000 => Color::new(255, 127, 0),
      0b1001 => Color::new(255, 192, 128),
      0b1010 => Color::new(255, 128, 128),
      0b1011 => Color::new(128, 255, 255),
      0b1100 => Color::new(255, 128, 255),
      0b1101 => Color::new(128, 255, 128),
      0b1110 => Color::new(128, 128, 255),
      0b1111 => Color::new(255, 255, 128),
      _ => unreachable!(),
    }
  }

  /// Get the color of the screen border.
  fn get_border_color(&self) -> Color {
    match self.border_color & 0b111 {
      0b000 => Color::new(0, 0, 0),
      0b001 => Color::new(255, 255, 255),
      0b010 => Color::new(255, 0, 0),
      0b011 => Color::new(0, 255, 255),
      0b100 => Color::new(255, 0, 255),
      0b101 => Color::new(0, 255, 0),
      0b110 => Color::new(0, 0, 255),
      0b111 => Color::new(255, 255, 0),
      _ => unreachable!(),
    }
  }

  /// Get the auxiliary color used for multicolor characters.
  fn get_aux_color(&self) -> Color {
    match self.aux_color & 0b1111 {
      0b0000 => Color::new(0, 0, 0),
      0b0001 => Color::new(255, 255, 255),
      0b0010 => Color::new(255, 0, 0),
      0b0011 => Color::new(0, 255, 255),
      0b0100 => Color::new(255, 0, 255),
      0b0101 => Color::new(0, 255, 0),
      0b0110 => Color::new(0, 0, 255),
      0b0111 => Color::new(255, 255, 0),
      0b1000 => Color::new(255, 127, 0),
      0b1001 => Color::new(255, 192, 128),
      0b1010 => Color::new(255, 128, 128),
      0b1011 => Color::new(128, 255, 255),
      0b1100 => Color::new(255, 128, 255),
      0b1101 => Color::new(128, 255, 128),
      0b1110 => Color::new(128, 128, 255),
      0b1111 => Color::new(255, 255, 128),
      _ => unreachable!(),
    }
  }

  /// Redraw the character at the specified address.
  fn redraw(&mut self, address: u16, memory: &mut Box<dyn Memory>, framebuffer: &mut [u8]) {
    if address >= (self.row_count as u16 * self.column_count as u16) {
      return; // ignore writes to the extra bytes
    }

    let column = (address % self.column_count as u16) as u32;
    let row = (address / self.column_count as u16) as u32;
    let base_column = column * 8;
    let base_row = row * 8;

    let value = self.read_vram(address, memory);
    let color = self.read_color(address, memory);
    let character = self.get_character(value, memory);

    let char_height = if self.double_size_chars { 16 } else { 8 };
    let char_width = if color & 0b1000 == 0 { 8 } else { 4 };

    if color & 0b1000 == 0 {
      for line in 0..char_height {
        let line_data = character[line as usize];
        for pixel in 0..char_width {
          let color = if line_data & (1 << (char_width - 1 - pixel)) != 0 {
            self.get_foreground(address, memory)
          } else {
            self.get_background()
          };

          let x = base_column + pixel;
          let y = base_row + line;
          let index = (y * (self.column_count as u32 * 8) + x) as usize * 4;
          let pixel = &mut framebuffer[index..(index + 4)];
          pixel.copy_from_slice(&color.to_rgba());
        }
      }
    } else {
      // Multicolor characters
      for line in 0..char_height {
        let line_data = character[line as usize];
        for pixel in 0..char_width {
          let color_code = line_data >> (2 * (char_width - pixel - 1)) & 0b11;

          let color = match color_code {
            0b00 => self.get_background(),
            0b01 => self.get_border_color(),
            0b10 => self.get_foreground(address, memory),
            0b11 => self.get_aux_color(),
            _ => unreachable!(),
          };

          let x = column * 8 + (pixel * 2);
          let y = row * char_height + line;
          let index = ((y * self.column_count as u32 + x) * 4) as usize;
          let pixel = &mut framebuffer[index..(index + 4)];
          pixel.copy_from_slice(&color.to_rgba());

          let index = ((y * self.column_count as u32 + x + 1) * 4) as usize;
          let pixel = &mut framebuffer[index..(index + 4)];
          pixel.copy_from_slice(&color.to_rgba());
        }
      }
    }
  }

  /// Redraw the entire screen.
  pub fn redraw_screen(&mut self, memory: &mut Box<dyn Memory>, framebuffer: &mut [u8]) {
    for row in 0..self.row_count {
      for column in 0..self.column_count {
        let address = (row as u16) * (self.column_count as u16) + (column as u16);
        self.redraw(address, memory, framebuffer);
      }
    }
  }
}

/// Represents the I/O mapping for the MOS 6560 VIC.
pub struct VicChipIO {
  chip: Rc<RefCell<VicChip>>,
}

impl VicChipIO {
  pub fn new(chip: Rc<RefCell<VicChip>>) -> Self {
    Self { chip }
  }
}

impl Memory for VicChipIO {
  fn read(&mut self, address: u16) -> u8 {
    let chip = self.chip.borrow();

    match address & 0x0F {
      0x0 => chip.left_draw_offset | (chip.scan_mode as u8) << 7,
      0x1 => chip.top_draw_offset,
      0x2 => chip.column_count | (chip.color_ram_mapping as u8) << 7,
      0x3 => {
        (chip.double_size_chars as u8)
          | (chip.row_count << 1)
          | ((chip.raster_counter & 0b1) as u8) << 7
      }
      0x4 => (chip.raster_counter >> 1) as u8,
      0x5 => chip.character_address_top | (chip.vram_address_top << 4),
      0x6 => chip.light_pen.read_x(),
      0x7 => chip.light_pen.read_y(),
      0x8 => chip.potentiometer_1,
      0x9 => chip.potentiometer_2,
      0xA => chip.speaker_alto.read(),
      0xB => chip.speaker_tenor.read(),
      0xC => chip.speaker_soprano.read(),
      0xD => chip.speaker_noise.read(),
      0xE => chip.speaker_volume | (chip.aux_color << 4),
      0xF => chip.border_color | (chip.reverse_field as u8) << 3 | (chip.background_color << 4),
      _ => unreachable!(),
    }
  }

  fn write(&mut self, address: u16, value: u8) {
    let mut chip = self.chip.borrow_mut();
    match address & 0x0F {
      0x0 => {
        chip.scan_mode = (value & 0x80) != 0;
        chip.left_draw_offset = value & 0x7F;
      }
      0x1 => chip.top_draw_offset = value,
      0x2 => {
        if value & 0x7F != chip.column_count {
          chip.platform.request_window(WindowConfig::new(
            (value & 0x7F) as u32 * 8,
            chip.row_count as u32 * 8,
            2.0,
          ));
        }

        chip.color_ram_mapping = (value & 0x80) != 0;
        chip.column_count = value & 0x7F;
      }
      0x3 => {
        chip.raster_counter = (chip.raster_counter & 0x1FE) | ((value & 0x80) as u16) >> 7;

        if ((value >> 1) & 0x3F) != chip.row_count {
          chip.platform.request_window(WindowConfig::new(
            chip.column_count as u32 * 8,
            ((value >> 1) & 0x3F) as u32 * 8,
            2.0,
          ));
        }

        chip.row_count = (value >> 1) & 0x3F;
        chip.double_size_chars = (value & 0x01) != 0;

        if chip.double_size_chars {
          panic!("Double size characters not supported yet");
        }
      }
      0x4 => chip.raster_counter = (chip.raster_counter & 0x1) | ((value as u16) << 1),
      0x5 => {
        chip.vram_address_top = (value >> 4) & 0x0F;
        chip.character_address_top = value & 0x0F;
      }
      0x6 => chip.light_pen.write_x(value),
      0x7 => chip.light_pen.write_y(value),
      0x8 => chip.potentiometer_1 = value,
      0x9 => chip.potentiometer_2 = value,
      0xA => chip.speaker_alto.write(value),
      0xB => chip.speaker_tenor.write(value),
      0xC => chip.speaker_soprano.write(value),
      0xD => chip.speaker_noise.write(value),
      0xE => {
        chip.speaker_volume = value & 0x0F;
        chip.aux_color = (value >> 4) & 0x0F;
      }
      0xF => {
        chip.border_color = value & 0x0F;
        chip.reverse_field = (value & 0x08) != 0;
        chip.background_color = (value >> 4) & 0x0F;
      }
      _ => unreachable!(),
    }
  }

  fn reset(&mut self) {
    self.chip.borrow_mut().reset();
  }

  fn poll(&mut self, _cycles: u32, _info: &SystemInfo) -> ActiveInterrupt {
    ActiveInterrupt::None
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_vic_to_cpu_address() {
    assert_eq!(0x8000, VicChip::vic_to_cpu_address(0x0000));
    assert_eq!(0x8FFF, VicChip::vic_to_cpu_address(0x0FFF));
    assert_eq!(0x9000, VicChip::vic_to_cpu_address(0x1000));
    assert_eq!(0x9FFF, VicChip::vic_to_cpu_address(0x1FFF));
    assert_eq!(0x0000, VicChip::vic_to_cpu_address(0x2000));
    assert_eq!(0x0FFF, VicChip::vic_to_cpu_address(0x2FFF));
    assert_eq!(0x1000, VicChip::vic_to_cpu_address(0x3000));
    assert_eq!(0x1FFF, VicChip::vic_to_cpu_address(0x3FFF));
  }
}
