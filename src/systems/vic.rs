use crate::memory::{
  ActiveInterrupt, BlockMemory, BranchMemory, Memory, NullMemory, RomFile, SharedMemory, SystemInfo,
};
use crate::platform::{Color, PlatformProvider, WindowConfig};
use crate::system::System;
use crate::systems::SystemFactory;
use std::sync::{Arc, Mutex};

const WIDTH: u32 = 22;
const HEIGHT: u32 = 23;
const CHAR_WIDTH: u32 = 8;
const CHAR_HEIGHT: u32 = 8;
const VRAM_SIZE: usize = 512; // 6 extra bytes to make mapping easier

pub struct Vic20SystemRoms {
  pub character: RomFile,
  pub basic: RomFile,
  pub kernal: RomFile,
}

impl Vic20SystemRoms {
  #[cfg(not(target_arch = "wasm32"))]
  pub fn from_disk() -> Self {
    let character = RomFile::from_file("vic/char.bin");
    let basic = RomFile::from_file("vic/basic.bin");
    let kernal = RomFile::from_file("vic/kernal.bin");

    Self {
      character,
      basic,
      kernal,
    }
  }
}

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
struct VicChip {
  // Associated Memory
  characters: SharedMemory,
  colors: SharedMemory,

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
  fn new(characters: RomFile) -> Self {
    Self {
      characters: SharedMemory::new(Box::new(BlockMemory::from_file(0x1000, characters))),
      colors: SharedMemory::new(Box::new(BlockMemory::ram(0x0200))),

      scan_mode: false,
      left_draw_offset: 12,
      top_draw_offset: 38,
      column_count: WIDTH as u8,
      vram_line_9: true,
      raster_counter: 0,
      row_count: HEIGHT as u8,
      double_size_chars: false,
      vram_address_top: 15,
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

  fn characters(&self) -> Box<dyn Memory> {
    Box::new(self.characters.clone())
  }

  fn get_character(&mut self, value: u8) -> Vec<u8> {
    let character_index = (value as u16) * 8;

    let mut character = vec![0; 8];
    for i in 0..8 {
      character[i] = self.characters.read(character_index + i as u16);
    }

    if value & 0x80 != 0 {
      character = character.iter().map(|&x| !x).collect();
    }

    character
  }

  fn colors(&self) -> Box<dyn Memory> {
    Box::new(self.colors.clone())
  }

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
}

impl Memory for VicChip {
  fn read(&mut self, address: u16) -> u8 {
    match address % 0xF {
      0x0 => self.left_draw_offset | (self.scan_mode as u8) << 7,
      0x1 => self.top_draw_offset,
      0x2 => self.column_count | (self.vram_line_9 as u8) << 7,
      0x3 => {
        (self.double_size_chars as u8)
          | (self.row_count << 1)
          | ((self.raster_counter & 0b1) as u8) << 7
      }
      0x4 => (self.raster_counter >> 1) as u8,
      0x5 => self.character_table_values | (self.vram_address_top << 4),
      0x6 => self.light_pen.read_x(),
      0x7 => self.light_pen.read_y(),
      0x8 => self.potentiometer_1,
      0x9 => self.potentiometer_2,
      0xA => self.speaker_alto.read(),
      0xB => self.speaker_tenor.read(),
      0xC => self.speaker_soprano.read(),
      0xD => self.speaker_noise.read(),
      0xE => self.speaker_volume | (self.aux_color << 4),
      0xF => self.border_color | (self.reverse_field as u8) << 3 | (self.background_color << 4),
      _ => unreachable!(),
    }
  }

  fn write(&mut self, address: u16, value: u8) {
    match address & 0xF {
      0x0 => {
        self.scan_mode = (value & 0x80) != 0;
        self.left_draw_offset = value & 0x7F;
      }
      0x1 => self.top_draw_offset = value,
      0x2 => {
        self.vram_line_9 = (value & 0x80) != 0;
        self.column_count = value & 0x7F;
      }
      0x3 => {
        self.raster_counter = (self.raster_counter & 0x1FE) | ((value & 0x80) as u16) >> 7;
        self.row_count = (value >> 1) & 0x3F;
        self.double_size_chars = (value & 0x01) != 0;
      }
      0x4 => self.raster_counter = (self.raster_counter & 0x1) | ((value as u16) << 1),
      0x5 => {
        self.vram_address_top = (value >> 4) & 0x0F;
        self.character_table_values = value & 0x0F;
      }
      0x6 => self.light_pen.write_x(value),
      0x7 => self.light_pen.write_y(value),
      0x8 => self.potentiometer_1 = value,
      0x9 => self.potentiometer_2 = value,
      0xA => self.speaker_alto.write(value),
      0xB => self.speaker_tenor.write(value),
      0xC => self.speaker_soprano.write(value),
      0xD => self.speaker_noise.write(value),
      0xE => {
        self.speaker_volume = value & 0x0F;
        self.aux_color = (value >> 4) & 0x0F;
      }
      0xF => {
        self.border_color = value & 0x0F;
        self.reverse_field = (value & 0x08) != 0;
        self.background_color = (value >> 4) & 0x0F;
      }
      _ => unreachable!(),
    }
  }

  fn reset(&mut self) {
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

  fn poll(&mut self, _info: &SystemInfo) -> ActiveInterrupt {
    ActiveInterrupt::None
  }
}

struct VicVram {
  data: Vec<u8>,
  platform: Arc<dyn PlatformProvider>,
  chip: Arc<Mutex<VicChip>>,
}

impl VicVram {
  fn new(platform: Arc<dyn PlatformProvider>, chip: Arc<Mutex<VicChip>>) -> VicVram {
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

pub struct Vic20SystemFactory {}

impl SystemFactory<Vic20SystemRoms> for Vic20SystemFactory {
  fn create(roms: Vic20SystemRoms, platform: Arc<dyn PlatformProvider>) -> System {
    let low_ram = BlockMemory::ram(0x0400);
    let main_ram = BlockMemory::ram(0x0E00);

    let vic_chip = Arc::new(Mutex::new(VicChip::new(roms.character)));

    let basic_rom = BlockMemory::from_file(0x2000, roms.basic);
    let kernel_rom = BlockMemory::from_file(0x2000, roms.kernal);

    let vram = VicVram::new(platform.clone(), vic_chip.clone());
    let characters = { vic_chip.lock().unwrap().characters() };
    let colors = { vic_chip.lock().unwrap().colors() };

    let memory = BranchMemory::new()
      .map(0x0000, Box::new(low_ram))
      .map(0x0400, Box::new(NullMemory::new()))
      .map(0x1000, Box::new(main_ram))
      .map(0x1E00, Box::new(vram))
      .map(0x2000, Box::new(NullMemory::new()))
      // .map(0x2000, Box::new(expansion_ram))
      .map(0x8000, characters)
      // .map(0x9000, SharedMemory::new(Box::new(vic_chip)))
      .map(0x9000, Box::new(NullMemory::new()))
      .map(0x9600, colors)
      .map(0xC000, Box::new(basic_rom))
      .map(0xE000, Box::new(kernel_rom));

    System::new(Box::new(memory), 1_000_000)
  }
}
