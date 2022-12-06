use crate::memory::{ActiveInterrupt, BlockMemory, Memory, RomFile, SystemInfo};
use crate::platform::{Color, PlatformProvider, WindowConfig};
use std::sync::{Arc, Mutex};

const WIDTH: u32 = 22;
const HEIGHT: u32 = 23;
const CHAR_WIDTH: u32 = 8;
const CHAR_HEIGHT: u32 = 8;
const VRAM_SIZE: usize = 512; // 6 extra bytes to make mapping easier

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

// Source: http://tinyvga.com/6561

/// The MOS 6560 VIC (Video Interface Chip).
/// Uses VRAM memory, character memory, and color memory to draw the screen.
/// Also handles the speakers and light pen.
pub struct VicChip {
  platform: Arc<dyn PlatformProvider>,

  // Associated Memory
  vram: BlockMemory,
  characters: BlockMemory,
  colors: BlockMemory,

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
  vram_line_9: bool,

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

  // Misc
  character_table_values: u8, // what is this?
}

impl VicChip {
  pub fn new(platform: Arc<dyn PlatformProvider>, characters: RomFile) -> Self {
    platform.request_window(WindowConfig::new(
      WIDTH * CHAR_WIDTH,
      HEIGHT * CHAR_HEIGHT,
      2.0,
    ));

    Self {
      platform,

      // Associated Memory
      vram: BlockMemory::ram(VRAM_SIZE),
      characters: BlockMemory::from_file(0x1000, characters),
      colors: BlockMemory::ram(0x0200),

      scan_mode: false,
      left_draw_offset: 12,
      top_draw_offset: 38,
      column_count: WIDTH as u8,
      vram_line_9: true,
      raster_counter: 0,
      row_count: HEIGHT as u8,
      double_size_chars: false,
      vram_address_top: 0,
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
      character_table_values: 0,
    }
  }

  pub fn reset(&mut self) {
    self.scan_mode = false;
    self.left_draw_offset = 12;
    self.top_draw_offset = 38;
    self.column_count = 22;
    self.vram_line_9 = true;
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
    self.character_table_values = 0;
  }

  /// Get the bits in the character at the given value.
  /// The character is 8 bits wide and 8 bits tall, so this returns a vec![0; 8].
  /// Each byte is a horizontal row, which are ordered from top to bottom.
  /// Bits are ordered with the MSB at the left and the LSB at the right.
  fn get_character(&mut self, value: u8) -> Vec<u8> {
    let character_index = (value as u16) * 8;

    let mut character = vec![0; 8];
    for i in 0..8 {
      character[i] = self.characters.read(character_index + i as u16);
    }

    character
  }

  /// Get the foreground color to be shown at the given character position.
  fn get_foreground(&mut self, address: u16) -> Color {
    let value = self.colors.read(address);
    match value {
      0b000 => Color::new(0, 0, 0),
      0b001 => Color::new(255, 255, 255),
      0b010 => Color::new(255, 0, 0),
      0b011 => Color::new(0, 255, 255),
      0b100 => Color::new(255, 0, 255),
      0b101 => Color::new(0, 255, 0),
      0b110 => Color::new(0, 0, 255),
      0b111 => Color::new(255, 255, 0),
      _ => panic!("Invalid color value: {}", value),
    }
  }

  /// Get the current background color being shown.
  fn get_background(&self) -> Color {
    match self.background_color {
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
      _ => panic!("Invalid color value: {}", self.background_color),
    }
  }

  /// Redraw the character at the specified address.
  fn redraw(&mut self, address: u16) {
    if address >= (HEIGHT * WIDTH) as u16 {
      return; // ignore writes to the extra bytes
    }

    let column = (address % WIDTH as u16) as u32;
    let row = (address / WIDTH as u16) as u32;

    let value = self.read_vram(address);
    let character = self.get_character(value);

    for line in 0..CHAR_HEIGHT {
      let line_data = character[line as usize];
      for pixel in 0..CHAR_WIDTH {
        let color = if line_data & (1 << (CHAR_WIDTH - 1 - pixel)) != 0 {
          self.get_foreground(address)
        } else {
          self.get_background()
        };

        self
          .platform
          .set_pixel(column * CHAR_WIDTH + pixel, row * CHAR_HEIGHT + line, color);
      }
    }
  }

  pub fn read_vram(&mut self, address: u16) -> u8 {
    self.vram.read(address)
  }

  pub fn write_vram(&mut self, address: u16, value: u8) {
    self.vram.write(address, value);
    self.redraw(address);
  }

  pub fn read_character(&mut self, address: u16) -> u8 {
    self.characters.read(address)
  }

  pub fn write_character(&mut self, address: u16, value: u8) {
    self.characters.write(address, value);

    // Redraw the entire screen
    for i in 0..(WIDTH * HEIGHT) {
      self.redraw(i as u16);
    }
  }

  pub fn read_color(&mut self, address: u16) -> u8 {
    self.colors.read(address)
  }

  pub fn write_color(&mut self, address: u16, value: u8) {
    self.colors.write(address, value);
    self.redraw(address);
  }
}

/// Represents the I/O mapping for the MOS 6560 VIC.
pub struct VicChipIO {
  chip: Arc<Mutex<VicChip>>,
}

impl VicChipIO {
  pub fn new(chip: Arc<Mutex<VicChip>>) -> Self {
    Self { chip }
  }
}

impl Memory for VicChipIO {
  fn read(&mut self, address: u16) -> u8 {
    let chip = self.chip.lock().unwrap();

    match address % 0xF {
      0x0 => chip.left_draw_offset | (chip.scan_mode as u8) << 7,
      0x1 => chip.top_draw_offset,
      0x2 => chip.column_count | (chip.vram_line_9 as u8) << 7,
      0x3 => {
        (chip.double_size_chars as u8)
          | (chip.row_count << 1)
          | ((chip.raster_counter & 0b1) as u8) << 7
      }
      0x4 => (chip.raster_counter >> 1) as u8,
      0x5 => chip.character_table_values | (chip.vram_address_top << 4),
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
    let mut chip = self.chip.lock().unwrap();
    match address & 0xF {
      0x0 => {
        chip.scan_mode = (value & 0x80) != 0;
        chip.left_draw_offset = value & 0x7F;
      }
      0x1 => chip.top_draw_offset = value,
      0x2 => {
        chip.vram_line_9 = (value & 0x80) != 0;
        chip.column_count = value & 0x7F;
      }
      0x3 => {
        chip.raster_counter = (chip.raster_counter & 0x1FE) | ((value & 0x80) as u16) >> 7;
        chip.row_count = (value >> 1) & 0x3F;
        chip.double_size_chars = (value & 0x01) != 0;
      }
      0x4 => chip.raster_counter = (chip.raster_counter & 0x1) | ((value as u16) << 1),
      0x5 => {
        chip.vram_address_top = (value >> 4) & 0x0F;
        chip.character_table_values = value & 0x0F;
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
    let mut chip = self.chip.lock().unwrap();
    chip.reset();
  }

  fn poll(&mut self, _info: &SystemInfo) -> ActiveInterrupt {
    ActiveInterrupt::None
  }
}
