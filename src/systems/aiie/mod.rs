use std::sync::Arc;

use crate::{
  cpu::Mos6502,
  keyboard::KeyMappingStrategy,
  memory::{ActiveInterrupt, BlockMemory, BranchMemory, Memory, NullMemory, SystemInfo},
  platform::{PlatformProvider, WindowConfig},
  systems::System,
};

// https://mirrors.apple2.org.za/apple.cabi.net/Languages.Programming/MemoryMap.IIe.64K.128K.txt
// https://www.kreativekorp.com/miscpages/a2info/memorymap.shtml

mod keyboard;
mod roms;

pub use roms::AiieSystemRoms;

use instant::Duration;

use super::SystemBuilder;

struct AiieSoftSwitches {
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
  fn new() -> Self {
    Self {
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

    match (address & !1) % 0x100 {
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
    let value = match address % 0x100 {
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

      _ => unimplemented!(),
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
        // println!("KEYBOARD: read data");
        1
      }
      0x01..=0x0F | 0x50..=0x5F => {
        self.softswitch(address);
        0
      }
      0x10..=0x1F => self.read_flag(address),
      0x30 => {
        println!("SPEAKER : toggle speaker diaphragm");
        0
      }
      0x61 => todo!("OPNAPPLE: open apple (command) key data"),
      0x62 => todo!("CLSAPPLE: closed apple (option) key data"),
      0x70 => todo!("PDLTRIG : trigger paddles"),

      _ => unimplemented!(),
    }
  }
  fn write(&mut self, address: u16, value: u8) {
    match address % 0x100 {
      0x00..=0x0F | 0x50..=0x5F => self.softswitch(address),
      0x10..=0x1F => (),
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
pub struct AiieSystemConfig {
  pub mapping: KeyMappingStrategy,
}

/// A factory for creating a Commodore 64 system.
pub struct AiieSystemBuilder;

impl SystemBuilder<AiieSystem, AiieSystemRoms, AiieSystemConfig> for AiieSystemBuilder {
  fn build(
    roms: AiieSystemRoms,
    config: AiieSystemConfig,
    platform: Arc<dyn PlatformProvider>,
  ) -> Box<dyn System> {
    // platform.request_window(WindowConfig::new(
    //   FULL_WIDTH,
    //   FULL_HEIGHT,
    //   2.0,
    // ));

    let ram = BlockMemory::ram(0xC000);
    let io = AiieSoftSwitches::new();
    let peripheral_card = NullMemory::new();
    let applesoft_interpreter = BlockMemory::from_file(0x2800, roms.applesoft);
    let monitor = BlockMemory::from_file(0x800, roms.monitor);

    let memory = BranchMemory::new()
      .map(0x0000, Box::new(ram))
      .map(0xC000, Box::new(io))
      .map(0xC100, Box::new(peripheral_card))
      .map(0xD000, Box::new(applesoft_interpreter))
      .map(0xF800, Box::new(monitor));

    let cpu = Mos6502::new(Box::new(memory));

    Box::new(AiieSystem { cpu })
  }
}

/// The Apple IIe system.
pub struct AiieSystem {
  cpu: Mos6502,
}

impl System for AiieSystem {
  fn tick(&mut self) -> Duration {
    Duration::from_secs_f64(1.0 / 1_000_000.0) * self.cpu.tick() as u32
  }

  fn reset(&mut self) {
    self.cpu.reset();
  }

  fn render(&mut self, framebuffer: &mut [u8], config: WindowConfig) {
    // TODO
  }
}
