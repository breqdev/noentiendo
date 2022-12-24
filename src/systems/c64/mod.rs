use std::{
  cell::{Cell, RefCell},
  rc::Rc,
  sync::Arc,
};

use crate::{
  keyboard::{KeyAdapter, KeyMappingStrategy, SymbolAdapter},
  memory::{
    interface::CIA, BankedMemory, BlockMemory, BranchMemory, NullMemory, NullPort, Port, SystemInfo,
  },
  platform::PlatformProvider,
  system::System,
  systems::SystemFactory,
};

mod keyboard;
mod roms;
mod vic_ii;

pub use roms::C64SystemRoms;

use self::{
  keyboard::{C64KeyboardAdapter, C64SymbolAdapter, KEYBOARD_MAPPING},
  vic_ii::{VicIIChip, VicIIChipDMA, VicIIChipIO},
};

struct C64Cia1PortA {
  keyboard_row: Rc<Cell<u8>>,
}

impl C64Cia1PortA {
  pub fn new() -> Self {
    Self {
      keyboard_row: Rc::new(Cell::new(0)),
    }
  }

  /// Return a reference to the keyboard column's current value.
  pub fn get_keyboard_col(&self) -> Rc<Cell<u8>> {
    self.keyboard_row.clone()
  }
}

impl Port for C64Cia1PortA {
  fn read(&mut self) -> u8 {
    self.keyboard_row.get()
  }

  fn write(&mut self, value: u8) {
    self.keyboard_row.set(value);
  }

  fn poll(&mut self, _info: &SystemInfo) -> bool {
    false
  }

  fn reset(&mut self) {}
}

struct C64Cia1PortB {
  keyboard_col: Rc<Cell<u8>>,
  mapping_strategy: KeyMappingStrategy,
  platform: Arc<dyn PlatformProvider>,
}

impl C64Cia1PortB {
  /// Create a new instance of the port, with the given keyboard column,
  /// reading the key status from the given platform.
  pub fn new(
    keyboard_col: Rc<Cell<u8>>,
    mapping_strategy: KeyMappingStrategy,
    platform: Arc<dyn PlatformProvider>,
  ) -> Self {
    Self {
      keyboard_col,
      mapping_strategy,
      platform,
    }
  }
}

impl Port for C64Cia1PortB {
  fn read(&mut self) -> u8 {
    let col_mask = self.keyboard_col.get();

    let mut value = 0b1111_1111;

    let state = match &self.mapping_strategy {
      KeyMappingStrategy::Physical => C64KeyboardAdapter::map(&self.platform.get_key_state()),
      KeyMappingStrategy::Symbolic => {
        C64SymbolAdapter::map(&SymbolAdapter::map(&self.platform.get_key_state()))
      }
    };

    for (y, row) in KEYBOARD_MAPPING.iter().enumerate() {
      for (x, key) in row.iter().enumerate() {
        if ((!col_mask & (1 << y)) != 0) && state.is_pressed(*key) {
          value &= !(1 << x);
        }
      }
    }

    value
  }

  fn write(&mut self, _value: u8) {
    panic!("Tried to write to keyboard row");
  }

  fn poll(&mut self, _info: &SystemInfo) -> bool {
    false
  }

  fn reset(&mut self) {}
}

/// Configuration for a Commodore 64 system.
pub struct C64SystemConfig {
  pub mapping: KeyMappingStrategy,
}

/// The Commodore 64 system.
pub struct C64SystemFactory;

impl SystemFactory<C64SystemRoms, C64SystemConfig> for C64SystemFactory {
  fn create(
    roms: C64SystemRoms,
    config: C64SystemConfig,
    platform: Arc<dyn PlatformProvider>,
  ) -> System {
    // Region 1: 0x0000 - 0x0FFF
    let region1 = BlockMemory::ram(0x1000);

    // Region 2: 0x1000 - 0x7FFF
    let selector2 = Rc::new(Cell::new(0));
    let region2 = BankedMemory::new(selector2)
      .bank(Box::new(BlockMemory::ram(0x7000)))
      .bank(Box::new(NullMemory::new()));

    // Region 3: 0x8000 - 0x9FFF
    let selector3 = Rc::new(Cell::new(0));
    let region3 = BankedMemory::new(selector3)
      .bank(Box::new(BlockMemory::ram(0x2000)))
      .bank(Box::new(NullMemory::new())); // TODO: Cartridge Rom Low

    // Region 4: 0xA000 - 0xBFFF
    let selector4 = Rc::new(Cell::new(0));
    let region4 = BankedMemory::new(selector4)
      .bank(Box::new(BlockMemory::from_file(0x2000, roms.basic)))
      .bank(Box::new(BlockMemory::ram(0x2000)))
      .bank(Box::new(NullMemory::new())) // TODO: Cartridge Rom High
      .bank(Box::new(NullMemory::new()));

    // Region 5: 0xC000 - 0xCFFF
    let selector5 = Rc::new(Cell::new(0));
    let region5 = BankedMemory::new(selector5)
      .bank(Box::new(BlockMemory::ram(0x1000)))
      .bank(Box::new(NullMemory::new()));

    // Region 6: 0xD000 - 0xDFFF
    let selector6 = Rc::new(Cell::new(0));

    let character_rom = BlockMemory::from_file(0x1000, roms.character.clone());
    let vic_ii = Rc::new(RefCell::new(VicIIChip::new(
      platform.clone(),
      Box::new(character_rom),
    )));
    let vic_io = VicIIChipIO::new(vic_ii.clone()); // TODO: bank switching!

    let port_a = C64Cia1PortA::new();
    let keyboard_col = port_a.get_keyboard_col();
    let cia_1 = CIA::new(
      Box::new(port_a),
      Box::new(C64Cia1PortB::new(
        keyboard_col,
        config.mapping,
        platform.clone(),
      )),
    );

    let cia_2 = CIA::new(Box::new(NullPort::new()), Box::new(NullPort::new()));

    let region6 = BankedMemory::new(selector6)
      .bank(Box::new(
        BranchMemory::new()
          // .map(0x000, Box::new(vic_io))
          .map(0x400, Box::new(NullMemory::new()))
          .map(0xC00, Box::new(cia_1))
          .map(0xD00, Box::new(cia_2)),
      ))
      .bank(Box::new(BlockMemory::ram(0x1000)))
      .bank(Box::new(BlockMemory::from_file(0x1000, roms.character)));

    // Region 7: 0xE000 - 0xFFFF
    let selector7 = Rc::new(Cell::new(0));
    let region7 = BankedMemory::new(selector7)
      .bank(Box::new(BlockMemory::from_file(0x2000, roms.kernal)))
      .bank(Box::new(BlockMemory::ram(0x2000)))
      .bank(Box::new(NullMemory::new())); // TODO: Cartidge Rom High

    let memory = BranchMemory::new()
      .map(0x0000, Box::new(region1))
      .map(0x1000, Box::new(region2))
      .map(0x8000, Box::new(region3))
      .map(0xA000, Box::new(region4))
      .map(0xC000, Box::new(region5))
      .map(0xD000, Box::new(region6))
      .map(0xE000, Box::new(region7));

    let mut system = System::new(Box::new(memory), 1_000_000);

    system.attach_dma(Box::new(VicIIChipDMA::new(vic_ii)));

    system
  }
}
