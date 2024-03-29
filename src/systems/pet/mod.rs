use crate::cpu::{
  mos6502::{MemoryIO, Mos6502, Mos6502Variant},
  Cpu,
};
use crate::keyboard::{KeyAdapter, KeyMappingStrategy, SymbolAdapter};
use crate::memory::mos652x::{Pia, Via};
use crate::memory::{BlockMemory, BranchMemory, NullMemory, NullPort, Port};
use crate::platform::{Color, PlatformProvider, WindowConfig};
use crate::systems::{BuildableSystem, System};
use instant::Instant;
use std::cell::Cell;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;
mod roms;
pub use roms::PetSystemRoms;
mod keyboard;
pub use keyboard::PetKeys;
use keyboard::{PetKeyboardAdapter, PetSymbolAdapter, KEYBOARD_MAPPING};

use self::keyboard::PetVirtualAdapter;

const WIDTH: u32 = 40;
const HEIGHT: u32 = 25;
const CHAR_WIDTH: u32 = 8;
const CHAR_HEIGHT: u32 = 8;
const VRAM_SIZE: usize = 1024; // 24 extra bytes to make mapping easier

/// Port A on the first PIA.
/// This is used for generating the 60Hz interrupt (which is fired when the
/// screen drawing reaches the last line), and for setting the active
/// row of the keyboard matrix.
pub struct PetPia1PortA {
  keyboard_row: Rc<Cell<u8>>,
  last_draw_instant: Option<Instant>,
  last_draw_cycle: u64,
}

impl PetPia1PortA {
  pub fn new() -> Self {
    Self {
      keyboard_row: Rc::new(Cell::new(0)),
      last_draw_instant: None,
      last_draw_cycle: 0,
    }
  }

  pub fn get_keyboard_row(&self) -> Rc<Cell<u8>> {
    self.keyboard_row.clone()
  }
}

impl Port for PetPia1PortA {
  fn read(&mut self) -> u8 {
    0b1000_0000 | self.keyboard_row.get()
    //^         diagnostic mode off
    // ^        IEEE488 (not implemented)
    //  ^^      Cassette sense (not implemented)
    //     ^^^^ Keyboard row select
  }

  fn write(&mut self, value: u8) {
    self.keyboard_row.set(value & 0b1111);
  }

  fn poll(&mut self, _cycles_since_poll: u64, total_cycle_count: u64) -> bool {
    // let min_elapsed = ((info.cycles_per_second as f64 / 60.0) * (2.0 / 3.0)) as u64;
    let min_elapsed = 0; // TODO: fix

    match self.last_draw_instant {
      Some(last_draw) => {
        if (last_draw.elapsed() > Duration::from_millis(17))
          && (total_cycle_count > self.last_draw_cycle + min_elapsed)
        {
          self.last_draw_cycle = total_cycle_count;
          self.last_draw_instant = Some(Instant::now());
          true
          // false
        } else {
          false
        }
      }
      None => {
        self.last_draw_instant = Some(Instant::now());
        false
      }
    }
  }

  fn reset(&mut self) {
    self.keyboard_row.set(0);
  }
}

/// Port B on the first PIA.
/// This is used for reading the keyboard matrix.
pub struct PetPia1PortB {
  keyboard_row: Rc<Cell<u8>>,
  mapping_strategy: KeyMappingStrategy,
  platform: Arc<dyn PlatformProvider>,
}

impl PetPia1PortB {
  pub fn new(
    keyboard_row: Rc<Cell<u8>>,
    mapping_strategy: KeyMappingStrategy,
    platform: Arc<dyn PlatformProvider>,
  ) -> Self {
    Self {
      keyboard_row,
      mapping_strategy,
      platform,
    }
  }
}

impl Port for PetPia1PortB {
  fn read(&mut self) -> u8 {
    let row = self.keyboard_row.get();
    let row = KEYBOARD_MAPPING[row as usize % 10];
    let mut value = 0b1111_1111;

    let state = match &self.mapping_strategy {
      KeyMappingStrategy::Physical => PetKeyboardAdapter::map(&self.platform.get_key_state()),
      KeyMappingStrategy::Symbolic => {
        PetSymbolAdapter::map(&SymbolAdapter::map(&self.platform.get_key_state()))
      }
    };

    let state = state | PetVirtualAdapter::map(&self.platform.get_virtual_key_state());

    for (i, key) in row.iter().enumerate() {
      if state.is_pressed(*key) {
        value &= !(1 << i);
      }
    }
    value
  }

  fn write(&mut self, _value: u8) {}

  fn poll(&mut self, _cycles_since_poll: u64, _total_cycle_count: u64) -> bool {
    false
  }

  fn reset(&mut self) {}
}

/// Configuration for a Commodore PET system.
pub struct PetSystemConfig {
  pub mapping: KeyMappingStrategy,
}

impl BuildableSystem<PetSystemRoms, PetSystemConfig> for PetSystem {
  fn build(
    roms: PetSystemRoms,
    config: PetSystemConfig,
    platform: Arc<dyn PlatformProvider>,
  ) -> Box<dyn System> {
    platform.request_window(WindowConfig::new(
      WIDTH * CHAR_WIDTH,
      HEIGHT * CHAR_HEIGHT,
      2.0,
    ));

    let ram = BlockMemory::ram(0x8000);
    let vram = BlockMemory::ram(VRAM_SIZE);

    let expansion_rom_9 = NullMemory::new();
    let expansion_rom_a = NullMemory::new();
    let expansion_rom_b = NullMemory::new();

    let basic_rom = BlockMemory::from_file(0x2000, roms.basic);
    let editor_rom = BlockMemory::from_file(0x1000, roms.editor);

    let port_a = PetPia1PortA::new();
    let port_b = PetPia1PortB::new(port_a.get_keyboard_row(), config.mapping, platform);
    let pia1 = Pia::new(Box::new(port_a), Box::new(port_b));
    let pia2 = Pia::new(Box::new(NullPort::new()), Box::new(NullPort::new()));
    let via = Via::new(Box::new(NullPort::new()), Box::new(NullPort::new()));

    let kernel_rom = BlockMemory::from_file(0x1000, roms.kernal);

    let memory = BranchMemory::new()
      .map(0x0000, ram)
      .map(0x8000, vram)
      .map(0x9000, expansion_rom_9)
      .map(0xA000, expansion_rom_a)
      .map(0xB000, expansion_rom_b)
      .map(0xC000, basic_rom)
      .map(0xE000, editor_rom)
      .map(0xE810, pia1)
      .map(0xE820, pia2)
      .map(0xE840, via)
      .map(0xF000, kernel_rom);

    let cpu = Mos6502::new(memory, Mos6502Variant::NMOS);

    Box::new(PetSystem {
      cpu,
      characters: roms.character.get_data(),
    })
  }
}

/// The Commodore PET system.
pub struct PetSystem {
  cpu: Mos6502,
  characters: Vec<u8>,
}

impl System for PetSystem {
  fn get_cpu_mut(&mut self) -> Box<&mut dyn Cpu> {
    Box::new(&mut self.cpu)
  }

  fn tick(&mut self) -> Duration {
    Duration::from_secs_f64(1.0 / 1_000_000.0) * self.cpu.tick() as u32
  }

  fn reset(&mut self) {
    self.cpu.reset();
  }

  fn render(&mut self, framebuffer: &mut [u8], config: WindowConfig) {
    for y in 0..HEIGHT {
      for x in 0..WIDTH {
        let index = (y * WIDTH + x) as u16;
        let value = self.cpu.read(0x8000 + index);

        let character_index = (value as usize) * 8;

        let mut character = self.characters[character_index..(character_index + 8)].to_vec();

        if value & 0x80 != 0 {
          character = character.iter().map(|&x| !x).collect();
        }

        for line in 0..CHAR_HEIGHT {
          let line_data = character[line as usize];
          for pixel in 0..CHAR_WIDTH {
            let color = if line_data & (1 << (CHAR_WIDTH - 1 - pixel)) != 0 {
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
