use crate::memory::{
  ActiveInterrupt, BlockMemory, BranchMemory, Memory, NullMemory, RomFile, SystemInfo,
};
use crate::platform::PlatformProvider;
use crate::system::System;
use crate::systems::SystemFactory;
use std::sync::Arc;

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
}

// Source: http://tinyvga.com/6561
struct VicChip {
  // $9000
  scan_mode: bool,
  left_draw_offset: u8,
  // $9001
  top_draw_offset: u8,
  // $9002
  column_count: u8,
  vram_line_9: bool,
  // $9003
  raster_counter_lsb: bool,
  row_count: u8,
  double_size_chars: bool,
  // $9004
  raster_counter: u8,
  // $9005
  vram_address_top: u8,
  character_table_values: u8,
  // $9006
  light_pen_horizontal: u8,
  // $9007
  light_pen_vertical: u8,
  // $9008
  potentiometer_1: u8,
  // $9009
  potentiometer_2: u8,
  // $900A
  speaker_alto: VicChipSpeaker,
  // $900B
  speaker_tenor: VicChipSpeaker,
  // $900C
  speaker_soprano: VicChipSpeaker,
  // $900D
  speaker_noise: VicChipSpeaker,
  // $900E
  speaker_volume: u8,
  aux_color: u8,
  // $900F
  border_color: u8,
  reverse_field: bool,
  background_color: u8,
}

impl VicChip {
  fn new() -> Self {
    Self {
      scan_mode: false,
      left_draw_offset: 12,
      top_draw_offset: 38,
      column_count: 22,
      vram_line_9: true,
      raster_counter_lsb: false,
      row_count: 23,
      double_size_chars: false,
      raster_counter: 0,
      vram_address_top: 15,
      character_table_values: 0,
      light_pen_horizontal: 0,
      light_pen_vertical: 1,
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
          | (self.raster_counter_lsb as u8) << 7
      }
      0x4 => self.raster_counter, // TODO: merge this with raster_counter_lsb ?
      0x5 => self.character_table_values | (self.vram_address_top << 4),
      0x6 => self.light_pen_horizontal,
      0x7 => self.light_pen_vertical,
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
        self.raster_counter_lsb = (value & 0x80) != 0;
        self.row_count = (value >> 1) & 0x3F;
        self.double_size_chars = (value & 0x01) != 0;
      }
      0x4 => self.raster_counter = value,
      0x5 => {
        self.vram_address_top = (value >> 4) & 0x0F;
        self.character_table_values = value & 0x0F;
      }
      0x6 => self.light_pen_horizontal = value,
      0x7 => self.light_pen_vertical = value,
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
    *self = Self::new();
  }

  fn poll(&mut self, _info: &SystemInfo) -> ActiveInterrupt {
    ActiveInterrupt::None
  }
}

pub struct Vic20SystemFactory {}

impl SystemFactory<Vic20SystemRoms> for Vic20SystemFactory {
  fn create(roms: Vic20SystemRoms, platform: Arc<dyn PlatformProvider>) -> System {
    let low_ram = BlockMemory::ram(0x0400);
    let main_ram = BlockMemory::ram(0x1000);
    let expansion_ram = NullMemory::new();

    let characters = BlockMemory::from_file(0x1000, roms.character);

    let io = VicChip::new();

    let basic_rom = BlockMemory::from_file(0x2000, roms.basic);
    let kernel_rom = BlockMemory::from_file(0x2000, roms.kernal);

    let memory = BranchMemory::new()
      .map(0x0000, Box::new(low_ram))
      .map(0x1000, Box::new(main_ram))
      .map(0x2000, Box::new(expansion_ram))
      .map(0x8000, Box::new(characters))
      .map(0x9000, Box::new(io))
      .map(0xC000, Box::new(basic_rom))
      .map(0xE000, Box::new(kernel_rom));

    System::new(Box::new(memory), 1_000)
  }
}
