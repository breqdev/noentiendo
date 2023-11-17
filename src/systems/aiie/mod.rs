use std::{io::Write, sync::Arc};

use crate::{
  cpu::{MemoryIO, Mos6502},
  keyboard::{KeyAdapter, SymbolAdapter},
  memory::{
    ActiveInterrupt, BlockMemory, BranchMemory, LoggingMemory, Memory, NullMemory, SystemInfo,
  },
  platform::{Color, PlatformProvider, WindowConfig},
  systems::{aiie::keyboard::AppleIISymbolAdapter, System},
};

// https://mirrors.apple2.org.za/apple.cabi.net/Languages.Programming/MemoryMap.IIe.64K.128K.txt
// https://www.kreativekorp.com/miscpages/a2info/memorymap.shtml

mod keyboard;
mod roms;

pub use roms::AiieSystemRoms;

use instant::Duration;

use super::SystemBuilder;

const WIDTH: u32 = 40;
const HEIGHT: u32 = 24;
const CHAR_HEIGHT: u32 = 8;
const CHAR_WIDTH: u32 = 7;

struct AiieSoftSwitches {
  platform: Arc<dyn PlatformProvider>,
  previous_key: Option<u8>,

  pub keypress_waiting: bool,
  pub eighty_col_memory: bool,
  pub read_aux_48k: bool,
  pub write_aux_48k: bool,
  pub ext_slot_rom: bool,
  pub aux_zeropage: bool,
  pub ext_slot_c3_rom: bool,
  pub eighty_col_display: bool,
  pub alt_characters: bool,
  pub text_mode: bool,
  pub mixed_mode: bool,
  pub text_page2: bool,
  pub hi_res: bool,
  pub annunciator: (bool, bool, bool, bool),
}

impl AiieSoftSwitches {
  fn new(platform: Arc<dyn PlatformProvider>) -> Self {
    Self {
      platform,
      previous_key: None,
      keypress_waiting: false,
      eighty_col_memory: false,
      read_aux_48k: false,
      write_aux_48k: false,
      ext_slot_rom: false,
      aux_zeropage: false,
      ext_slot_c3_rom: false,
      eighty_col_display: false,
      alt_characters: false,
      text_mode: false,
      mixed_mode: false,
      text_page2: false,
      hi_res: false,
      annunciator: (false, false, false, false),
    }
  }

  /// Set or clear a softswitch value.
  fn softswitch(&mut self, address: u16) {
    let value = address & 1 != 0;

    println!("softswitch {:02X} <- {}", address & !1, value);

    match address & !1 {
      0x00 => self.eighty_col_memory = value,
      0x02 => self.read_aux_48k = value,
      0x04 => self.write_aux_48k = value,
      0x06 => self.ext_slot_rom = value,
      0x08 => self.aux_zeropage = value,
      0x0A => self.ext_slot_c3_rom = value,
      0x0C => self.eighty_col_display = value,
      0x0E => self.alt_characters = value,

      0x50 => self.text_mode = value,
      0x52 => self.mixed_mode = value,
      0x54 => self.text_page2 = value,
      0x56 => self.hi_res = value,

      0x58 => self.annunciator.0 = value,
      0x5A => self.annunciator.1 = value,
      0x5C => self.annunciator.2 = value,
      0x5E => self.annunciator.3 = value,

      _ => todo!("unimplemented softswitch"),
    };
  }

  /// Read one of the "RD" locations.
  fn read_flag(&mut self, address: u16) -> u8 {
    let value = match address {
      0x11 => todo!("RDLCBNK2: reading from LC 0x $Dx 2"),
      0x12 => todo!("RDLCRAM : reading from LC RAM"),
      0x13 => self.read_aux_48k,
      0x14 => self.write_aux_48k,
      0x15 => self.ext_slot_rom,
      0x16 => self.aux_zeropage,
      0x17 => self.ext_slot_c3_rom,
      0x18 => self.eighty_col_memory,
      0x19 => todo!("RDVBLBAR: not VBL (VBL signal low)"),
      0x1A => self.text_mode,
      0x1B => self.mixed_mode,
      0x1C => self.text_page2,
      0x1D => self.hi_res,
      0x1E => self.alt_characters,
      0x1F => self.eighty_col_display,

      _ => todo!("unimplemented softswitch"),
    };

    if value {
      1
    } else {
      0
    }
  }
}

impl Memory for AiieSoftSwitches {
  fn read(&mut self, address: u16) -> u8 {
    match address % 0x100 {
      0x00 => {
        let state = AppleIISymbolAdapter::map(&SymbolAdapter::map(&self.platform.get_key_state()));
        let key = state.get_one_key();

        if key != self.previous_key {
          self.keypress_waiting = true;
        }
        self.previous_key = key.clone();

        (self.keypress_waiting as u8) << 7 | key.unwrap_or(0)
      }
      0x01..=0x0F | 0x50..=0x5F => {
        self.softswitch(address);
        0
      }
      0x10 => {
        self.keypress_waiting = false;

        let state = AppleIISymbolAdapter::map(&SymbolAdapter::map(&self.platform.get_key_state()));
        let key = state.get_one_key();

        (self.keypress_waiting as u8) << 7 | key.unwrap_or(0)
      }
      0x11..=0x1F => self.read_flag(address),
      0x30 => {
        print!("🔈");
        std::io::stdout().flush().unwrap();
        0
      }
      0x61 => todo!("OPNAPPLE: open apple (command) key data"),
      0x62 => todo!("CLSAPPLE: closed apple (option) key data"),
      0x70 => todo!("PDLTRIG : trigger paddles"),

      _ => unimplemented!(),
    }
  }
  fn write(&mut self, address: u16, _value: u8) {
    match address % 0x100 {
      0x00..=0x0F | 0x50..=0x5F => self.softswitch(address),
      0x10 => {
        self.keypress_waiting = false;
      }
      0x11..=0x1F => (),
      0x30 => println!("SPEAKER : toggle speaker diaphragm"),
      0x61 => todo!("OPNAPPLE: open apple (command) key data"),
      0x62 => todo!("CLSAPPLE: closed apple (option) key data"),
      0x70 => todo!("PDLTRIG : trigger paddles"),

      _ => unimplemented!(),
    }
  }

  fn reset(&mut self) {
    // set all the flags to false
    self.eighty_col_memory = false;
    self.read_aux_48k = false;
    self.write_aux_48k = false;
    self.ext_slot_rom = false;
    self.aux_zeropage = false;
    self.ext_slot_c3_rom = false;
    self.eighty_col_display = false;
    self.alt_characters = false;
    self.text_mode = false;
    self.mixed_mode = false;
    self.text_page2 = false;
    self.hi_res = false;
    self.annunciator = (false, false, false, false);
  }

  fn poll(&mut self, _cycles: u32, _info: &SystemInfo) -> ActiveInterrupt {
    ActiveInterrupt::None
  }
}

/// Configuration for a Apple IIe system.
pub struct AiieSystemConfig {}

/// A factory for creating a Commodore 64 system.
pub struct AiieSystemBuilder;

impl SystemBuilder<AiieSystem, AiieSystemRoms, AiieSystemConfig> for AiieSystemBuilder {
  fn build(
    roms: AiieSystemRoms,
    _config: AiieSystemConfig,
    platform: Arc<dyn PlatformProvider>,
  ) -> Box<dyn System> {
    platform.request_window(WindowConfig::new(
      WIDTH * CHAR_WIDTH,
      HEIGHT * CHAR_HEIGHT,
      2.0,
    ));

    let ram = BlockMemory::ram(0xC000);
    let io = AiieSoftSwitches::new(platform);
    let peripheral_card =
      LoggingMemory::new(Box::new(NullMemory::new()), "Peripheral Card", 0xC100);
    let applesoft_interpreter = BlockMemory::from_file(0x2800, roms.applesoft);
    let monitor = BlockMemory::from_file(0x800, roms.monitor);

    let memory = BranchMemory::new()
      .map(0x0000, Box::new(ram))
      .map(0x1000, Box::new(NullMemory::new()))
      .map(
        0xC000,
        Box::new(io),
        // Box::new(LoggingMemory::new(Box::new(io), "I/O", 0xC000)),
      )
      .map(0xC100, Box::new(peripheral_card))
      .map(0xD000, Box::new(applesoft_interpreter))
      .map(0xF800, Box::new(monitor));

    let cpu = Mos6502::new(Box::new(memory));

    Box::new(AiieSystem {
      cpu,
      characters: roms.character.get_data(),
    })
  }
}

/// The Apple IIe system.
pub struct AiieSystem {
  cpu: Mos6502,
  characters: Vec<u8>,
}

impl System for AiieSystem {
  fn tick(&mut self) -> Duration {
    if self.cpu.registers.pc.address() < 0xF800 {
      // println!("{:#04x}", self.cpu.registers.pc.address());
      // println!("{}", self.cpu.memory.read(32768));
    }
    Duration::from_secs_f64(1.0 / 1_000_000.0) * self.cpu.tick() as u32
  }

  fn reset(&mut self) {
    self.cpu.reset();
  }

  fn render(&mut self, framebuffer: &mut [u8], config: WindowConfig) {
    let flash_state = ((self.cpu.get_info().cycle_count / 500_000) & 0b1) == 0;

    // https://retrocomputing.stackexchange.com/a/2541
    for index in 0x000..=0x3FF {
      let position = match index & 0x7F {
        0x00..=0x27 => Some((0, (index & 0x7F) - 0x00)),
        0x28..=0x4F => Some((1, (index & 0x7F) - 0x28)),
        0x50..=0x77 => Some((2, (index & 0x7F) - 0x50)),
        _ => None,
      };

      if let Some((third, x)) = position {
        let y = (third * 8) + (index >> 7);
        // println!("{} -> {}, {}", index, x, y);

        let value = self.cpu.read((0x0400 + index) as u16);

        let character_index = ((value & 0b0011_1111) as usize) * 8;
        let mode = (value & 0b1100_0000) >> 6;

        let inverted = match mode {
          0b00 => true,
          0b01 => flash_state,
          0b10 | 0b11 => false,
          _ => unreachable!(),
        };

        let character = self.characters[character_index..(character_index + 8)].to_vec();

        for line in 0..CHAR_HEIGHT {
          let line_data = character[line as usize];
          for pixel in 0..CHAR_WIDTH {
            let color = if (line_data & (1 << pixel) != 0) ^ inverted {
              Color::new(0, 255, 0)
            } else {
              Color::new(0, 0, 0)
            };

            let x = x * CHAR_WIDTH + pixel;
            let y = y * CHAR_HEIGHT + line;
            let index = ((y * config.width + x) * 4) as usize;
            let pixel = &mut framebuffer[index..(index + 4)];
            pixel.copy_from_slice(&color.to_rgba());
          }
        }
      }
    }
  }
}
