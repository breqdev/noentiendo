use crate::cpu::{MemoryIO, Mos6502};
use crate::keyboard::{KeyAdapter, KeyMappingStrategy, SymbolAdapter};
use crate::memory::mos::{ControlLines, ControlLinesPort, Pia, Via};
use crate::memory::{
  mos::{NullPort, Port},
  BlockMemory, BranchMemory, NullMemory, SystemInfo,
};
use crate::peripherals::Datasette;
use crate::platform::{Color, PlatformProvider, WindowConfig};
use crate::systems::{System, SystemBuilder};
use instant::Instant;
use std::cell::{Cell, RefCell};
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
/// This is used for setting the active row of the keyboard matrix.
pub struct PetPia1PortA {
  keyboard_row: Rc<Cell<u8>>,
  datasette_1: Rc<RefCell<Datasette>>,
  datasette_2: Rc<RefCell<Datasette>>,
}

impl PetPia1PortA {
  pub fn new(datasette_1: Rc<RefCell<Datasette>>, datasette_2: Rc<RefCell<Datasette>>) -> Self {
    Self {
      keyboard_row: Rc::new(Cell::new(0)),
      datasette_1,
      datasette_2,
    }
  }

  pub fn get_keyboard_row(&self) -> Rc<Cell<u8>> {
    self.keyboard_row.clone()
  }
}

impl Port for PetPia1PortA {
  fn read(&mut self) -> u8 {
    (1 << 7) // diagnostic mode off
      | (0 << 6) // IEEE488 (not implemented)
      | ((self.datasette_2.borrow_mut().sense() as u8) << 5) // cassette sense #2
      | ((self.datasette_1.borrow_mut().sense() as u8) << 4) // cassette sense #1
      | self.keyboard_row.get() // keyboard row select
  }

  fn write(&mut self, value: u8) {
    self.keyboard_row.set(value & 0b1111);
  }

  fn reset(&mut self) {
    self.keyboard_row.set(0);
  }
}

impl ControlLinesPort for PetPia1PortA {
  fn poll(&mut self, _cycles: u32, _info: &SystemInfo) -> ControlLines {
    // CA1: Cassette #1 Read
    // CA2: blank screen (not implemented)

    ControlLines::new()
  }

  fn write_c2(&mut self, _value: bool) {}
}

/// Port B on the first PIA.
/// This is used for reading the keyboard matrix,
/// and for generating the 60Hz screen interrupt.
pub struct PetPia1PortB {
  keyboard_row: Rc<Cell<u8>>,
  mapping_strategy: KeyMappingStrategy,
  platform: Arc<dyn PlatformProvider>,
  last_draw_instant: Option<Instant>,
  last_draw_cycle: u64,
  datasette_1: Rc<RefCell<Datasette>>,
}

impl PetPia1PortB {
  pub fn new(
    keyboard_row: Rc<Cell<u8>>,
    mapping_strategy: KeyMappingStrategy,
    platform: Arc<dyn PlatformProvider>,
    datasette_1: Rc<RefCell<Datasette>>,
  ) -> Self {
    Self {
      keyboard_row,
      mapping_strategy,
      platform,
      last_draw_instant: None,
      last_draw_cycle: 0,
      datasette_1,
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

  fn reset(&mut self) {}
}

impl ControlLinesPort for PetPia1PortB {
  fn poll(&mut self, _cycles: u32, info: &SystemInfo) -> ControlLines {
    // CB1: Screen Interrupt (60Hz)
    // CB2: Cassette #1 Motor (always output)

    // let min_elapsed = ((info.cycles_per_second as f64 / 60.0) * (2.0 / 3.0)) as u64;
    let min_elapsed = 0; // TODO: fix

    match self.last_draw_instant {
      Some(last_draw) => {
        if (last_draw.elapsed() > Duration::from_millis(17))
          && (info.cycle_count > self.last_draw_cycle + min_elapsed)
        {
          self.last_draw_cycle = info.cycle_count;
          self.last_draw_instant = Some(Instant::now());

          ControlLines {
            c1: true,
            c2: false,
          }
        } else {
          ControlLines {
            c1: false,
            c2: false,
          }
        }
      }
      None => {
        self.last_draw_instant = Some(Instant::now());
        ControlLines {
          c1: false,
          c2: false,
        }
      }
    }
  }

  fn write_c2(&mut self, value: bool) {
    self.datasette_1.borrow_mut().set_motor(!value);
  }
}

/// Port A on the VIA in a Commodore PET.
/// Connects to the user port and sets the character set (uppercase/lowercase).
pub struct PetViaPortA {
  charset: Rc<Cell<bool>>,
}

impl PetViaPortA {
  pub fn new(charset: Rc<Cell<bool>>) -> Self {
    Self { charset }
  }
}

impl Port for PetViaPortA {
  fn read(&mut self) -> u8 {
    // TODO: user port
    0
  }

  fn write(&mut self, _value: u8) {
    // TODO: user port
  }

  fn reset(&mut self) {
    self.charset.set(false);
  }
}

impl ControlLinesPort for PetViaPortA {
  fn poll(&mut self, _cycles: u32, _info: &SystemInfo) -> ControlLines {
    // CA1: User port
    // CA2: Screen graphics (always output)

    ControlLines::new()
  }

  fn write_c2(&mut self, value: bool) {
    self.charset.set(value);
  }
}

/// Port B on the VIA in a Commodore PET.
/// Used for some IEEE488 control lines and the cassette #2 motor and data.
pub struct PetViaPortB {
  datasette_1: Rc<RefCell<Datasette>>,
  datasette_2: Rc<RefCell<Datasette>>,
}

impl PetViaPortB {
  pub fn new(datasette_1: Rc<RefCell<Datasette>>, datasette_2: Rc<RefCell<Datasette>>) -> Self {
    Self {
      datasette_1,
      datasette_2,
    }
  }
}

impl Port for PetViaPortB {
  fn read(&mut self) -> u8 {
    0
  }

  fn write(&mut self, value: u8) {
    let cassette_motor = value & 0b00010000 == 0;
    let cassette_write = value & 0b00001000 != 0;

    self.datasette_2.borrow_mut().set_motor(cassette_motor);
    self.datasette_1.borrow_mut().write(cassette_write);
    self.datasette_2.borrow_mut().write(cassette_write);
  }

  fn reset(&mut self) {}
}

impl ControlLinesPort for PetViaPortB {
  fn poll(&mut self, _cycles: u32, _info: &SystemInfo) -> ControlLines {
    // TODO: User port
    ControlLines::new()
  }

  fn write_c2(&mut self, _value: bool) {}
}

/// Configuration for a Commodore PET system.
pub struct PetSystemConfig {
  pub mapping: KeyMappingStrategy,
}

/// A factory for the Commodore PET.
pub struct PetSystemBuilder;

impl SystemBuilder<PetSystem, PetSystemRoms, PetSystemConfig> for PetSystemBuilder {
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

    let datasette_1 = Rc::new(RefCell::new(Datasette::new()));
    let datasette_2 = Rc::new(RefCell::new(Datasette::new()));

    let pia1_port_a = PetPia1PortA::new(datasette_1.clone(), datasette_2.clone());
    let pia1_port_b = PetPia1PortB::new(
      pia1_port_a.get_keyboard_row(),
      config.mapping,
      platform,
      datasette_1.clone(),
    );
    let pia1 = Pia::new(Box::new(pia1_port_a), Box::new(pia1_port_b));

    // Used exclusively for IEEE488 port, unimplemented
    let pia2 = Pia::new(Box::new(NullPort::new()), Box::new(NullPort::new()));

    let charset = Rc::new(Cell::new(false)); // TODO: actually use
    let via_port_a = PetViaPortA::new(charset);
    let via_port_b = PetViaPortB::new(datasette_1, datasette_2);
    let via = Via::new(Box::new(via_port_a), Box::new(via_port_b));

    let io = BranchMemory::new()
      .map(0x10, Box::new(pia1))
      .map(0x20, Box::new(pia2))
      .map(0x40, Box::new(via));

    let kernel_rom = BlockMemory::from_file(0x1000, roms.kernal);

    let memory = BranchMemory::new()
      .map(0x0000, Box::new(ram))
      .map(0x8000, Box::new(vram))
      .map(0x9000, Box::new(expansion_rom_9))
      .map(0xA000, Box::new(expansion_rom_a))
      .map(0xB000, Box::new(expansion_rom_b))
      .map(0xC000, Box::new(basic_rom))
      .map(0xE000, Box::new(editor_rom))
      .map(0xE800, Box::new(io))
      // .map(0xE800, Box::new(LoggingMemory::new(Box::new(io))))
      .map(0xF000, Box::new(kernel_rom));

    let cpu = Mos6502::new(Box::new(memory));

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
