use std::{
  cell::{Cell, RefCell},
  rc::Rc,
  sync::Arc,
};

use crate::{
  cpu::Mos6502,
  keyboard::{
    commodore::{C64KeyboardAdapter, C64SymbolAdapter},
    KeyAdapter, KeyMappingStrategy, SymbolAdapter,
  },
  memory::{
    mos652x::Cia, BankedMemory, BlockMemory, BranchMemory, Mos6510Port, NullMemory, NullPort, Port,
    SystemInfo,
  },
  platform::{PlatformProvider, WindowConfig},
  systems::System,
};

mod keyboard;
mod roms;
mod vic_ii;

use instant::Duration;
pub use roms::C64SystemRoms;

use self::{
  keyboard::KEYBOARD_MAPPING,
  vic_ii::{VicIIChip, VicIIChipIO},
};

use super::SystemBuilder;

/// Port A on the first CIA chip on the C64 deals with setting the keyboard row being scanned.
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
  pub fn get_keyboard_row(&self) -> Rc<Cell<u8>> {
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

  fn poll(&mut self, _cycles: u32, _info: &SystemInfo) -> bool {
    false
  }

  fn reset(&mut self) {}
}

/// Port B on the first CIA chip on the C64 deals with reading columns of the keyboard matrix.
struct C64Cia1PortB {
  keyboard_row: Rc<Cell<u8>>,
  mapping_strategy: KeyMappingStrategy,
  platform: Arc<dyn PlatformProvider>,
}

impl C64Cia1PortB {
  /// Create a new instance of the port, with the given keyboard column,
  /// reading the key status from the given platform.
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

impl Port for C64Cia1PortB {
  fn read(&mut self) -> u8 {
    let row_mask = self.keyboard_row.get();

    let mut value = 0b1111_1111;

    let state = match &self.mapping_strategy {
      KeyMappingStrategy::Physical => C64KeyboardAdapter::map(&self.platform.get_key_state()),
      KeyMappingStrategy::Symbolic => {
        C64SymbolAdapter::map(&SymbolAdapter::map(&self.platform.get_key_state()))
      }
    };

    for (y, row) in KEYBOARD_MAPPING.iter().enumerate() {
      for (x, key) in row.iter().enumerate() {
        if ((!row_mask & (1 << y)) != 0) && state.is_pressed(*key) {
          value &= !(1 << x);
        }
      }
    }

    value
  }

  fn write(&mut self, _value: u8) {
    panic!("Tried to write to keyboard row");
  }

  fn poll(&mut self, _cycles: u32, _info: &SystemInfo) -> bool {
    false
  }

  fn reset(&mut self) {}
}

/// Bank switching implementation performed using the 6510's I/O port.
/// Source: <https://www.c64-wiki.com/wiki/Bank_Switching>
pub struct C64BankSwitching {
  /// CPU Control Lines
  hiram: bool,
  loram: bool,
  charen: bool,

  /// Selectors to choose what is mapped in each memory region.
  selectors: [Rc<Cell<usize>>; 6],
}

impl C64BankSwitching {
  pub fn new(mut selectors: [Rc<Cell<usize>>; 6]) -> Self {
    selectors.iter_mut().for_each(|s| s.set(0));
    Self {
      hiram: true,
      loram: true,
      charen: true,
      selectors,
    }
  }
}

impl Port for C64BankSwitching {
  fn read(&mut self) -> u8 {
    (self.loram as u8) | (self.hiram as u8) << 1 | (self.charen as u8) << 2
  }

  #[allow(clippy::bool_to_int_with_if)]
  fn write(&mut self, value: u8) {
    self.loram = (value & 0b001) != 0;
    self.hiram = (value & 0b010) != 0;
    self.charen = (value & 0b100) != 0;

    // TODO: EXROM, GAME signals

    // Region 2: RAM or inaccessible
    self.selectors[0].set(0);

    // Region 3: RAM or Cartridge ROM Low
    self.selectors[1].set(0);

    // Region 4: BASIC ROM, RAM, Cartridge ROM High, or inaccessible
    self.selectors[2].set(if self.hiram && self.loram { 0 } else { 1 });

    // Region 5: RAM or inaccessible
    self.selectors[3].set(0);

    // Region 6: I/O, RAM, or character rom
    self.selectors[4].set(if !self.hiram && !self.loram {
      1
    } else if !self.charen {
      2
    } else {
      0
    });

    // Region 7: Kernal ROM or RAM
    self.selectors[5].set(if !self.hiram { 1 } else { 0 });
  }

  fn poll(&mut self, _cycles: u32, _info: &SystemInfo) -> bool {
    false
  }

  fn reset(&mut self) {
    self.hiram = true;
    self.loram = true;
    self.charen = true;
  }
}

/// Configuration for a Commodore 64 system.
pub struct C64SystemConfig {
  pub mapping: KeyMappingStrategy,
}

/// A factory for creating a Commodore 64 system.
pub struct C64SystemBuilder;

impl SystemBuilder<C64System, C64SystemRoms, C64SystemConfig> for C64SystemBuilder {
  fn build(
    roms: C64SystemRoms,
    config: C64SystemConfig,
    platform: Arc<dyn PlatformProvider>,
  ) -> Box<dyn System> {
    platform.request_window(WindowConfig::new(
      vic_ii::FULL_WIDTH,
      vic_ii::FULL_HEIGHT,
      2.0,
    ));

    // Region 1: 0x0000 - 0x0FFF
    let region1 = BlockMemory::ram(0x1000);

    // Region 2: 0x1000 - 0x7FFF
    let selector2 = Rc::new(Cell::new(0));
    let region2 = BankedMemory::new(selector2.clone())
      .bank(Box::new(BlockMemory::ram(0x7000)))
      .bank(Box::new(NullMemory::new()));

    // Region 3: 0x8000 - 0x9FFF
    let selector3 = Rc::new(Cell::new(0));
    let region3 = BankedMemory::new(selector3.clone())
      .bank(Box::new(BlockMemory::ram(0x2000)))
      .bank(Box::new(NullMemory::new())); // TODO: Cartridge Rom Low

    // Region 4: 0xA000 - 0xBFFF
    let selector4 = Rc::new(Cell::new(0));
    let region4 = BankedMemory::new(selector4.clone())
      .bank(Box::new(BlockMemory::from_file(0x2000, roms.basic)))
      .bank(Box::new(BlockMemory::ram(0x2000)))
      .bank(Box::new(NullMemory::new())) // TODO: Cartridge Rom High
      .bank(Box::new(NullMemory::new()));

    // Region 5: 0xC000 - 0xCFFF
    let selector5 = Rc::new(Cell::new(0));
    let region5 = BankedMemory::new(selector5.clone())
      .bank(Box::new(BlockMemory::ram(0x1000)))
      .bank(Box::new(NullMemory::new()));

    // Region 6: 0xD000 - 0xDFFF
    let selector6 = Rc::new(Cell::new(0));

    let character_rom = BlockMemory::from_file(0x1000, roms.character.clone());
    let vic_ii = Rc::new(RefCell::new(VicIIChip::new(Box::new(character_rom))));
    let vic_io = VicIIChipIO::new(vic_ii.clone()); // TODO: bank switching!

    let port_a = C64Cia1PortA::new();
    let keyboard_col = port_a.get_keyboard_row();
    let cia_1 = Cia::new(
      Box::new(port_a),
      Box::new(C64Cia1PortB::new(
        keyboard_col,
        config.mapping,
        platform.clone(),
      )),
    );

    let cia_2 = Cia::new(Box::new(NullPort::new()), Box::new(NullPort::new()));

    let region6 = BankedMemory::new(selector6.clone())
      .bank(Box::new(
        BranchMemory::new()
          .map(0x000, Box::new(vic_io))
          .map(0x400, Box::new(NullMemory::new())) // TODO: SID
          .map(0x800, Box::new(BlockMemory::ram(0x0400)))
          .map(0xC00, Box::new(cia_1))
          .map(0xD00, Box::new(cia_2))
          .map(0xE00, Box::new(NullMemory::new())) // TODO: Expansion card
          .map(0xF00, Box::new(NullMemory::new())), // TODO: Expansion card
      ))
      .bank(Box::new(BlockMemory::ram(0x1000)))
      .bank(Box::new(BlockMemory::from_file(0x1000, roms.character)));

    // Region 7: 0xE000 - 0xFFFF
    let selector7 = Rc::new(Cell::new(0));
    let region7 = BankedMemory::new(selector7.clone())
      .bank(Box::new(BlockMemory::from_file(0x2000, roms.kernal)))
      .bank(Box::new(BlockMemory::ram(0x2000)))
      .bank(Box::new(NullMemory::new())); // TODO: Cartidge Rom High

    let bank_switching = C64BankSwitching::new([
      selector2, selector3, selector4, selector5, selector6, selector7,
    ]);

    let memory = BranchMemory::new()
      .map(0x0000, Box::new(Mos6510Port::new(Box::new(bank_switching))))
      .map(0x0002, Box::new(region1))
      .map(0x1000, Box::new(region2))
      .map(0x8000, Box::new(region3))
      .map(0xA000, Box::new(region4))
      .map(0xC000, Box::new(region5))
      .map(0xD000, Box::new(region6))
      .map(0xE000, Box::new(region7));

    let cpu = Mos6502::new(Box::new(memory));

    Box::new(C64System { cpu, vic: vic_ii })
  }
}

/// The Commodore 64 system.
pub struct C64System {
  cpu: Mos6502,
  vic: Rc<RefCell<VicIIChip>>,
}

impl System for C64System {
  fn tick(&mut self) -> Duration {
    Duration::from_secs_f64(1.0 / 1_000_000.0) * self.cpu.tick() as u32
  }

  fn reset(&mut self) {
    self.cpu.reset();
  }

  fn render(&mut self, framebuffer: &mut [u8], config: WindowConfig) {
    self
      .vic
      .borrow_mut()
      .draw_screen(&mut self.cpu.memory, framebuffer, config)
  }
}
