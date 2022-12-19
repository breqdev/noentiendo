use std::{
  cell::{Cell, RefCell},
  rc::Rc,
};

use crate::memory::{ActiveInterrupt, Memory, SystemInfo};
use crate::platform::{Color, PlatformProvider, WindowConfig};

const WIDTH: u32 = 22;
const HEIGHT: u32 = 23;
const CHAR_WIDTH: u32 = 8;
const CHAR_HEIGHT: u32 = 8;
const VRAM_SIZE: usize = 512; // 6 extra bytes to make mapping easier

/// One of the speakers available on the MOS 6560 VIC.
struct VicChipSpeaker {
  on: Cell<bool>,
  note: Cell<u8>,
}

impl VicChipSpeaker {
  fn new() -> Self {
    Self {
      on: Cell::new(false),
      note: Cell::new(0),
    }
  }

  fn read(&self) -> u8 {
    self.note.get() | (self.on.get() as u8) << 7
  }

  fn write(&self, value: u8) {
    self.on.set((value & 0x80) != 0);
    self.note.set(value & 0x7f);
  }

  fn reset(&self) {
    self.on.set(false);
    self.note.set(0);
  }
}

/// The set of speakers included on the MOS 6560 VIC.
struct VicChipSpeakerSet {
  pub alto: VicChipSpeaker,
  pub tenor: VicChipSpeaker,
  pub soprano: VicChipSpeaker,
  pub noise: VicChipSpeaker,
  pub volume: Cell<u8>,
}

impl VicChipSpeakerSet {
  pub fn new() -> Self {
    Self {
      alto: VicChipSpeaker::new(),
      tenor: VicChipSpeaker::new(),
      soprano: VicChipSpeaker::new(),
      noise: VicChipSpeaker::new(),
      volume: Cell::new(0),
    }
  }

  pub fn reset(&self) {
    self.alto.reset();
    self.tenor.reset();
    self.soprano.reset();
    self.noise.reset();
    self.volume.set(0);
  }
}

/// The light pen input available on the MOS 6560 VIC.
struct VicChipLightPen {
  x: Cell<u8>,
  y: Cell<u8>,
}

impl VicChipLightPen {
  fn new() -> Self {
    Self {
      x: Cell::new(0),
      y: Cell::new(0),
    }
  }

  fn read_x(&self) -> u8 {
    self.x.get()
  }

  fn read_y(&self) -> u8 {
    self.y.get()
  }

  fn write_x(&self, value: u8) {
    self.x.set(value);
  }

  fn write_y(&self, value: u8) {
    self.y.set(value);
  }

  fn reset(&self) {
    self.x.set(0);
    self.y.set(0);
  }
}

// Source: http://tinyvga.com/6561

/// The MOS 6560 VIC (Video Interface Chip).
/// Uses VRAM memory, character memory, and color memory to draw the screen.
/// Also handles the speakers and light pen.
pub struct VicChip {
  // Registers
  /// TV scan settings
  scan_mode: Cell<bool>,

  /// Screen alignment: (left, top)
  draw_offset: Cell<(u8, u8)>,

  /// Character size: (row, column)
  character_count: Cell<(u8, u8)>,

  /// Double size characters
  double_size_chars: Cell<bool>,

  // Screen drawing
  raster_counter: Cell<u16>,

  // Memory mapping
  vram_address_top: Cell<u8>,
  character_address_top: Cell<u8>,

  // Light pen
  light_pen: VicChipLightPen,

  // Potentiometers
  potentiometers: Cell<(u8, u8)>,

  // Speakers
  speakers: VicChipSpeakerSet,

  // Colors
  aux_color: Cell<u8>,
  border_color: Cell<u8>,
  reverse_field: Cell<bool>,
  background_color: Cell<u8>,

  // Drawing
  last_draw_cycle: Cell<u64>,

  // Misc
  vram_line_9: Cell<bool>,
}

impl VicChip {
  pub fn new(platform: &Box<dyn PlatformProvider>) -> Self {
    platform.request_window(WindowConfig::new(
      WIDTH * CHAR_WIDTH,
      HEIGHT * CHAR_HEIGHT,
      2.0,
    ));

    Self {
      scan_mode: Cell::new(false),
      draw_offset: Cell::new((12, 38)),
      character_count: Cell::new((WIDTH as u8, HEIGHT as u8)),
      vram_line_9: Cell::new(true),
      raster_counter: Cell::new(0),
      double_size_chars: Cell::new(false),
      vram_address_top: Cell::new(0),
      light_pen: VicChipLightPen::new(),
      potentiometers: Cell::new((0xFF, 0xFF)),
      speakers: VicChipSpeakerSet::new(),
      aux_color: Cell::new(0),
      border_color: Cell::new(3),
      reverse_field: Cell::new(true),
      background_color: Cell::new(1),
      character_address_top: Cell::new(0),
      last_draw_cycle: Cell::new(0),
    }
  }

  /// Read the value of the specified location in the screen memory,
  /// taking into account the current memory mapping as defined by the VIC
  /// chip registers.
  fn read_vram(
    &self,
    address: u16,
    root: &Rc<dyn Memory>,
    platform: &Box<dyn PlatformProvider>,
  ) -> u8 {
    let vram_base = 0x1E00; // TODO: support remapping

    root.read(vram_base + address, root, platform)
  }

  /// Read the value of the specified location in the color memory,
  /// taking into account the current memory mapping as defined by the VIC
  /// chip registers.
  fn read_color(
    &self,
    address: u16,
    root: &Rc<dyn Memory>,
    platform: &Box<dyn PlatformProvider>,
  ) -> u8 {
    let color_base = 0x9600; // TODO: support remapping

    root.read(color_base + address, root, platform)
  }

  /// Read the value of the specified location in the character memory,
  /// taking into account the current memory mapping as defined by the VIC
  /// chip registers.
  fn read_character(
    &self,
    address: u16,
    root: &Rc<dyn Memory>,
    platform: &Box<dyn PlatformProvider>,
  ) -> u8 {
    let character_base = 0x8000; // TODO: support remapping

    root.read(character_base + address, root, platform)
  }

  /// Get the bits in the character at the given value.
  /// The character is 8 bits wide and 8 bits tall, so this returns a vec![0; 8].
  /// Each byte is a horizontal row, which are ordered from top to bottom.
  /// Bits are ordered with the MSB at the left and the LSB at the right.
  fn get_character(
    &self,
    value: u8,
    root: &Rc<dyn Memory>,
    platform: &Box<dyn PlatformProvider>,
  ) -> Vec<u8> {
    let character_index = (value as u16) * 8;

    let mut character = vec![0; 8];
    for i in 0..8 {
      character[i] = self.read_character(character_index + i as u16, root, platform);
    }

    character
  }

  /// Get the foreground color to be shown at the given character position.
  fn get_foreground(
    &self,
    address: u16,
    root: &Rc<dyn Memory>,
    platform: &Box<dyn PlatformProvider>,
  ) -> Color {
    let value = self.read_color(address, root, platform);
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
    match self.background_color.get() & 0b1111 {
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
    match self.border_color.get() & 0b111 {
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
    match self.aux_color.get() & 0b1111 {
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
  fn redraw(&self, address: u16, root: &Rc<dyn Memory>, platform: &Box<dyn PlatformProvider>) {
    if address >= (HEIGHT * WIDTH) as u16 {
      return; // ignore writes to the extra bytes
    }

    let column = (address % WIDTH as u16) as u32;
    let row = (address / WIDTH as u16) as u32;

    let value = self.read_vram(address, root, platform);
    let color = self.read_color(address, root, platform);
    let character = self.get_character(value, root, platform);

    if color & 0b1000 == 0 {
      // Standard characters
      for line in 0..CHAR_HEIGHT {
        let line_data = character[line as usize];
        for pixel in 0..CHAR_WIDTH {
          let color = if line_data & (1 << (CHAR_WIDTH - 1 - pixel)) != 0 {
            self.get_foreground(address, root, platform)
          } else {
            self.get_background()
          };

          platform.set_pixel(column * CHAR_WIDTH + pixel, row * CHAR_HEIGHT + line, color);
        }
      }
    } else {
      // Multicolor characters
      for line in 0..CHAR_HEIGHT {
        let line_data = character[line as usize];
        for pixel in 0..(CHAR_WIDTH / 2) {
          let color_code = (line_data >> (CHAR_WIDTH - 2 - (pixel * 2))) & 0b11;

          let color = match color_code {
            0b00 => self.get_background(),
            0b01 => self.get_border_color(),
            0b10 => self.get_foreground(address, root, platform),
            0b11 => self.get_aux_color(),
            _ => unreachable!(),
          };

          platform.set_pixel(
            column * CHAR_WIDTH + (pixel * 2),
            row * CHAR_HEIGHT + line,
            color,
          );
          platform.set_pixel(
            column * CHAR_WIDTH + (pixel * 2) + 1,
            row * CHAR_HEIGHT + line,
            color,
          );
        }
      }
    }
  }
}

impl Memory for VicChip {
  fn read(
    &self,
    address: u16,
    _root: &Rc<dyn Memory>,
    _platform: &Box<dyn PlatformProvider>,
  ) -> u8 {
    match address % 0xF {
      0x0 => self.draw_offset.get().0 | (self.scan_mode.get() as u8) << 7,
      0x1 => self.draw_offset.get().1,
      0x2 => self.character_count.get().1 | (self.vram_line_9.get() as u8) << 7,
      0x3 => {
        (self.double_size_chars.get() as u8)
          | (self.character_count.get().0 << 1)
          | ((self.raster_counter.get() & 0b1) as u8) << 7
      }
      0x4 => (self.raster_counter.get() >> 1) as u8,
      0x5 => self.character_address_top.get() | (self.vram_address_top.get() << 4),
      0x6 => self.light_pen.read_x(),
      0x7 => self.light_pen.read_y(),
      0x8 => self.potentiometers.get().0,
      0x9 => self.potentiometers.get().1,
      0xA => self.speakers.alto.read(),
      0xB => self.speakers.tenor.read(),
      0xC => self.speakers.soprano.read(),
      0xD => self.speakers.noise.read(),
      0xE => self.speakers.volume.get() | (self.aux_color.get() << 4),
      0xF => {
        self.border_color.get()
          | (self.reverse_field.get() as u8) << 3
          | (self.background_color.get() << 4)
      }
      _ => unreachable!(),
    }
  }

  fn write(
    &self,
    address: u16,
    value: u8,
    _root: &Rc<dyn Memory>,
    _platform: &Box<dyn PlatformProvider>,
  ) {
    match address & 0xF {
      0x0 => {
        self.scan_mode.set((value & 0x80) != 0);
        self
          .draw_offset
          .set(((value & 0x7F), self.draw_offset.get().1));
      }
      0x1 => self.draw_offset.set((self.draw_offset.get().0, value)),
      0x2 => {
        self.vram_line_9.set((value & 0x80) != 0);
        self
          .character_count
          .set((self.character_count.get().0, value & 0x7F));
      }
      0x3 => {
        self
          .raster_counter
          .set((self.raster_counter.get() & 0x1FE) | ((value & 0x80) as u16) >> 7);
        self
          .character_count
          .set(((value >> 1) & 0x3F, self.character_count.get().1));
        self.double_size_chars.set((value & 0x01) != 0);
      }
      0x4 => self
        .raster_counter
        .set((self.raster_counter.get() & 0x1) | ((value as u16) << 1)),
      0x5 => {
        self.vram_address_top.set((value >> 4) & 0x0F);
        self.character_address_top.set(value & 0x0F);
      }
      0x6 => self.light_pen.write_x(value),
      0x7 => self.light_pen.write_y(value),
      0x8 => self
        .potentiometers
        .set((value, self.potentiometers.get().1)),
      0x9 => self
        .potentiometers
        .set((self.potentiometers.get().0, value)),
      0xA => self.speakers.alto.write(value),
      0xB => self.speakers.tenor.write(value),
      0xC => self.speakers.soprano.write(value),
      0xD => self.speakers.noise.write(value),
      0xE => {
        self.speakers.volume.set(value & 0x0F);
        self.aux_color.set((value >> 4) & 0x0F);
      }
      0xF => {
        self.border_color.set(value & 0x0F);
        self.reverse_field.set((value & 0x08) != 0);
        self.background_color.set((value >> 4) & 0x0F);
      }
      _ => unreachable!(),
    }
  }

  fn reset(&self, _root: &Rc<dyn Memory>, _platform: &Box<dyn PlatformProvider>) {
    self.scan_mode.set(false);
    self.draw_offset.set((12, 38));
    self.vram_line_9.set(true);
    self.raster_counter.set(0);
    self.character_count.set((23, 22));
    self.double_size_chars.set(false);
    self.vram_address_top.set(15);
    self.light_pen.reset();
    self.potentiometers.set((0xFF, 0xFF));
    self.speakers.reset();
    self.aux_color.set(0);
    self.border_color.set(3);
    self.reverse_field.set(true);
    self.background_color.set(1);
    self.character_address_top.set(0);
  }

  fn poll(
    &self,
    info: &SystemInfo,
    root: &Rc<dyn Memory>,
    platform: &Box<dyn PlatformProvider>,
  ) -> ActiveInterrupt {
    if (info.cycle_count - self.last_draw_cycle.get()) >= 2000 {
      self.last_draw_cycle.set(info.cycle_count);
      for i in 0..(WIDTH * HEIGHT) {
        self.redraw(i as u16, root, platform);
      }
    }

    ActiveInterrupt::None
  }
}
