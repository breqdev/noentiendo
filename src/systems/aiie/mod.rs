use std::sync::Arc;

use crate::{
  cpu::{MemoryIO, Mos6502},
  memory::{BankedMemory, BlockMemory, BranchMemory, LoggingMemory, NullMemory},
  platform::{Color, PlatformProvider, WindowConfig},
  systems::System,
};

// https://mirrors.apple2.org.za/apple.cabi.net/Languages.Programming/MemoryMap.IIe.64K.128K.txt
// https://www.kreativekorp.com/miscpages/a2info/memorymap.shtml

mod keyboard;
mod roms;
mod switches;

pub use roms::AiieSystemRoms;

use instant::Duration;

use self::switches::{AiieBankSelectors, AiieSoftSwitches};

use super::SystemBuilder;

const WIDTH: u32 = 40;
const HEIGHT: u32 = 24;
const CHAR_HEIGHT: u32 = 8;
const CHAR_WIDTH: u32 = 7;

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

    let selectors = AiieBankSelectors::new();
    let io = AiieSoftSwitches::new(platform, selectors.clone());

    let memory = BranchMemory::new()
      .map(
        0x0000,
        Box::new(
          BankedMemory::new(selectors.zp_stack.clone())
            .bank(Box::new(BlockMemory::ram(0x0200))) // Main memory
            .bank(Box::new(BlockMemory::ram(0x0200))), // Aux memory
        ),
      )
      .map(
        0x0200,
        Box::new(
          BankedMemory::new(selectors.low_segment.clone())
            .bank(Box::new(BlockMemory::ram(0x0200))) // Main memory
            .bank(Box::new(BlockMemory::ram(0x0200))), // Aux memory
        ),
      )
      .map(
        0x0400,
        Box::new(
          BankedMemory::new(selectors.text_page_1.clone())
            .bank(Box::new(BlockMemory::ram(0x0400))) // Text Page 1
            .bank(Box::new(BlockMemory::ram(0x0400))), // Text Page 1X
        ),
      )
      .map(
        0x0800,
        Box::new(
          BankedMemory::new(selectors.text_page_2.clone())
            .bank(Box::new(BlockMemory::ram(0x1800))) // Text Page 2
            .bank(Box::new(BlockMemory::ram(0x1800))), // Text Page 2X
        ),
      )
      .map(
        0x2000,
        Box::new(
          BankedMemory::new(selectors.hires_page_1.clone())
            .bank(Box::new(BlockMemory::ram(0x2000))) // HiRes Page 1
            .bank(Box::new(BlockMemory::ram(0x2000))), // HiRes Page 1X
        ),
      )
      .map(
        0x4000,
        Box::new(
          BankedMemory::new(selectors.hires_page_2.clone())
            .bank(Box::new(BlockMemory::ram(0x8000))) // HiRes Page 1
            .bank(Box::new(BlockMemory::ram(0x8000))), // HiRes Page 1X
        ),
      )
      .map(
        0xC000,
        Box::new(io),
        // Box::new(LoggingMemory::new(Box::new(io), "I/O", 0xC000)),
      )
      .map(
        0xC100,
        Box::new(
          BankedMemory::new(selectors.ext_slot_rom.clone())
            .bank(Box::new(LoggingMemory::new(
              Box::new(NullMemory::new()),
              "Peripheral Card",
              0xC100,
            )))
            .bank(Box::new(BlockMemory::from_file(0x0F00, roms.firmware))),
        ),
      );

    let upper_rom = BranchMemory::new()
      .map(
        0x0000,
        Box::new(BlockMemory::from_file(0x2800, roms.applesoft)),
      )
      .map(
        0x2800,
        Box::new(BlockMemory::from_file(0x0800, roms.monitor)),
      );

    let upper_main_ram = BranchMemory::new()
      .map(
        0x0000,
        Box::new(
          BankedMemory::new(selectors.ram_bank_select.clone())
            .bank(Box::new(BlockMemory::ram(0x1000)))
            .bank(Box::new(BlockMemory::ram(0x1000))),
        ),
      )
      .map(0x1000, Box::new(BlockMemory::ram(0x2000)));

    let upper_aux_ram = BranchMemory::new()
      .map(
        0x0000,
        Box::new(
          BankedMemory::new(selectors.ram_bank_select)
            .bank(Box::new(BlockMemory::ram(0x1000)))
            .bank(Box::new(BlockMemory::ram(0x1000))),
        ),
      )
      .map(0x1000, Box::new(BlockMemory::ram(0x2000)));

    let memory = memory.map(
      0xD000,
      Box::new(
        BankedMemory::new(selectors.rom_ram_select)
          .bank(Box::new(upper_rom))
          .bank(Box::new(
            BankedMemory::new(selectors.upper_ram)
              .bank(Box::new(upper_main_ram))
              .bank(Box::new(upper_aux_ram)),
          )),
      ),
    );

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

  #[allow(clippy::identity_op)]
  fn render(&mut self, framebuffer: &mut [u8], config: WindowConfig) {
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

        let character_index = ((value & 0b0111_1111) as usize) * 8;
        let inverted = (value & 0b1000_0000) == 0;
        /*let mode = (value & 0b1100_0000) >> 6;

        let inverted = match mode {
          0b00 => true,
          0b01 => flash_state,
          0b10 | 0b11 => false,
          _ => unreachable!(),
        };*/

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
