use crate::cpu::System;
use crate::keyboard::{KeyAdapter, KeyMappingStrategy, SymbolAdapter};
use crate::memory::mos652x::{Pia, Via};
use crate::memory::{BlockMemory, BranchMemory, NullMemory, NullPort, Port, SystemInfo};
use crate::platform::PlatformProvider;
use crate::systems::SystemFactory;
use instant::Instant;
use std::cell::Cell;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;
mod vram;
use vram::PetVram;
mod roms;
pub use roms::PetSystemRoms;
mod keyboard;
use keyboard::{PetKeyboardAdapter, PetSymbolAdapter, KEYBOARD_MAPPING};

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

  fn poll(&mut self, info: &SystemInfo) -> bool {
    let min_elapsed = ((info.cycles_per_second as f64 / 60.0) * (2.0 / 3.0)) as u64;

    match self.last_draw_instant {
      Some(last_draw) => {
        if (last_draw.elapsed() > Duration::from_millis(17))
          && (info.cycle_count > self.last_draw_cycle + min_elapsed)
        {
          self.last_draw_cycle = info.cycle_count;
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

    for (i, key) in row.iter().enumerate() {
      if state.is_pressed(*key) {
        value &= !(1 << i);
      }
    }
    value
  }

  fn write(&mut self, _value: u8) {}

  fn poll(&mut self, _info: &SystemInfo) -> bool {
    false
  }

  fn reset(&mut self) {}
}

/// Configuration for a Commodore PET system.
pub struct PetSystemConfig {
  pub mapping: KeyMappingStrategy,
}

/// The Commodore PET system.
pub struct PetSystemFactory;

impl SystemFactory<PetSystemRoms, PetSystemConfig> for PetSystemFactory {
  fn create(
    roms: PetSystemRoms,
    config: PetSystemConfig,
    platform: Arc<dyn PlatformProvider>,
  ) -> System {
    let ram = BlockMemory::ram(0x8000);
    let vram = PetVram::new(roms.character, platform.clone());

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
      .map(0x0000, Box::new(ram))
      .map(0x8000, Box::new(vram))
      .map(0x9000, Box::new(expansion_rom_9))
      .map(0xA000, Box::new(expansion_rom_a))
      .map(0xB000, Box::new(expansion_rom_b))
      .map(0xC000, Box::new(basic_rom))
      .map(0xE000, Box::new(editor_rom))
      .map(0xE810, Box::new(pia1))
      .map(0xE820, Box::new(pia2))
      .map(0xE840, Box::new(via))
      .map(0xF000, Box::new(kernel_rom));

    System::new(Box::new(memory), 1_000_000)
  }
}
