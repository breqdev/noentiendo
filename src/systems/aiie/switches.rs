use std::{cell::Cell, io::Write, rc::Rc, sync::Arc};

use crate::{
  keyboard::{KeyAdapter, SymbolAdapter},
  memory::{ActiveInterrupt, Memory},
  platform::PlatformProvider,
  systems::aiie::keyboard::AppleIISymbolAdapter,
};

/// Implementation of the "soft switches" which control system parameters in the Apple IIe.
pub struct AiieSoftSwitches {
  platform: Arc<dyn PlatformProvider>,
  selectors: AiieBankSelectors,

  /// The most recent key pressed by the user. Used to detect changes.
  previous_key: Option<u8>,

  /// Whether there is a keypress waiting for the user.
  pub keypress_waiting: bool,

  /// Whether the system is using the "80 column" memory layout.
  pub eighty_col_memory: bool,

  /// Whether any reads to RAM in the lower 48k bytes should go to main RAM or aux RAM.
  pub read_aux_48k: bool,
  /// Whether any writes to RAM in the lower 48k bytes should go to main RAM or aux RAM.
  pub write_aux_48k: bool,

  /// Whether the external slot area should be mapped to external cards or ROM.
  pub ext_slot_rom: bool,

  /// Whether the zero page, stack page, and upper RAM should be in main RAM or aux RAM.
  pub aux_zeropage: bool,

  /// Not sure what this one does.
  pub ext_slot_c3_rom: bool,

  /// Whether the display should be configured to display 40 columns or 80 columns.
  pub eighty_col_display: bool,

  /// Whether an alternative character ROM should be used.
  pub alt_characters: bool,

  /// Whether text or graphics mode should be used.
  pub text_mode: bool,

  /// TODO
  pub mixed_mode: bool,

  /// Selects the page used for text or graphics data.
  pub text_page2: bool,

  /// Enables HiRes graphics.
  pub hi_res: bool,

  /// Voice parameters for the Annunciator.
  pub annunciator: (bool, bool, bool, bool),

  /// Control the mapping of RAM/ROM starting at 0xD000.
  /// True: Read from RAM. False: Read from ROM.
  pub bank_read_ram: bool,

  /// True: Write to RAM. False: Writes are no-ops.
  pub bank_write_ram: bool,

  /// True: Select RAM bank 2. False: Select RAM bank 1.
  pub bank_ram_select: bool,
}

impl AiieSoftSwitches {
  pub fn new(platform: Arc<dyn PlatformProvider>, selectors: AiieBankSelectors) -> Self {
    Self {
      platform,
      selectors,
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

      bank_read_ram: false,
      bank_write_ram: false,
      bank_ram_select: false,
    }
  }

  /// Set or clear a softswitch value.
  /// Each softswitch effectively has two addresses.
  /// Writing to or reading from the address of a softswitch where the LSB is
  /// set will toggle the softswitch ON, and if the LSB of the address is cleared
  /// then the softswitch will be toggled OFF.
  fn softswitch(&mut self, address: u16) {
    let value = (address & 1) == 1;

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

    self.selectors.clone().set(self);
  }

  /// Read one of the "RD" locations.
  fn read_flag(&mut self, address: u16) -> u8 {
    let value = match address {
      0x11 => self.bank_ram_select,
      0x12 => self.bank_read_ram,
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
        self.previous_key = key;

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
      0x11..=0x1F => self.read_flag(address) << 7,
      0x30 => {
        print!("🔈");
        std::io::stdout().flush().unwrap();
        0
      }
      0x61 => 0, //todo!("OPNAPPLE: open apple (command) key data"),
      0x62 => 0, //todo!("CLSAPPLE: closed apple (option) key data"),
      0x70 => todo!("PDLTRIG : trigger paddles"),

      // These softswitches are used for the upper section of ROM/RAM (past 0xD000).
      0x80..=0x8F => {
        self.bank_write_ram = (address & 0b0001) != 0;
        self.bank_ram_select = (address & 0b1000) == 0;

        self.bank_read_ram = match address & 0b11 {
          0b00 => true,
          0b01 => false,
          0b10 => false,
          0b11 => true,
          _ => unreachable!(),
        };

        self.selectors.clone().set(self);

        0
      }

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
    self.bank_read_ram = false;
    self.bank_write_ram = false;
    self.bank_ram_select = false;
  }

  fn poll(&mut self, _cycles_since_poll: u64, _total_cycle_count: u64) -> ActiveInterrupt {
    ActiveInterrupt::None
  }
}

#[derive(Clone)]
pub struct AiieBankSelectors {
  pub zp_stack: Rc<Cell<(usize, usize)>>,
  pub low_segment: Rc<Cell<(usize, usize)>>,
  pub text_page_1: Rc<Cell<(usize, usize)>>,
  pub text_page_2: Rc<Cell<(usize, usize)>>,
  pub hires_page_1: Rc<Cell<(usize, usize)>>,
  pub hires_page_2: Rc<Cell<(usize, usize)>>,

  pub ext_slot_rom: Rc<Cell<(usize, usize)>>,

  pub rom_ram_select: Rc<Cell<(usize, usize)>>,
  pub upper_ram: Rc<Cell<(usize, usize)>>,
  pub ram_bank_select: Rc<Cell<(usize, usize)>>,
}

impl AiieBankSelectors {
  pub fn new() -> AiieBankSelectors {
    AiieBankSelectors {
      zp_stack: Rc::new(Cell::new((0, 0))),
      low_segment: Rc::new(Cell::new((0, 0))),
      text_page_1: Rc::new(Cell::new((0, 0))),
      text_page_2: Rc::new(Cell::new((0, 0))),
      hires_page_1: Rc::new(Cell::new((0, 0))),
      hires_page_2: Rc::new(Cell::new((0, 0))),
      ext_slot_rom: Rc::new(Cell::new((0, 0))),
      rom_ram_select: Rc::new(Cell::new((0, 0))),
      upper_ram: Rc::new(Cell::new((0, 0))),
      ram_bank_select: Rc::new(Cell::new((0, 0))),
    }
  }

  fn set(&mut self, switches: &AiieSoftSwitches) {
    let selector_value = (
      switches.read_aux_48k as usize,
      switches.write_aux_48k as usize,
    );
    self.low_segment.set(selector_value);
    self.text_page_1.set(selector_value);
    self.text_page_2.set(selector_value);
    self.hires_page_1.set(selector_value);
    self.hires_page_2.set(selector_value);

    if switches.eighty_col_memory {
      if switches.text_page2 {
        self.text_page_1.set((1, 1));
      } else {
        self.text_page_1.set((0, 0));
      }

      if switches.hi_res {
        if switches.text_page2 {
          self.hires_page_1.set((1, 1));
        } else {
          self.hires_page_1.set((0, 0));
        }
      }
    }

    if switches.ext_slot_rom {
      self.ext_slot_rom.set((1, 1));
    }

    self.rom_ram_select.set((
      switches.bank_read_ram as usize,
      switches.bank_write_ram as usize,
    ));

    if switches.aux_zeropage {
      self.zp_stack.set((1, 1));
      self.upper_ram.set((1, 1));
    } else {
      self.zp_stack.set((0, 0));
      self.upper_ram.set((0, 0));
    }

    if switches.bank_ram_select {
      self.ram_bank_select.set((1, 1));
    } else {
      self.ram_bank_select.set((0, 0));
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::platform::{Platform, TextPlatform};
  use std::sync::Arc;

  // http://www.apple-iigs.info/doc/fichiers/Apple%20IIe%20Technical%20Notes.pdf
  #[test]
  fn fig4_left() {
    for hires in 0..=1 {
      let platform = Arc::new(TextPlatform::new());
      let selectors = AiieBankSelectors::new();
      let mut switches = AiieSoftSwitches::new(platform.provider(), selectors.clone());

      switches.softswitch(0x00); // 80STORE OFF
      switches.softswitch(0x54); // PAGE2 OFF (should also be on)
      switches.softswitch(0x56 + hires); // HIRES OFF or ON
      switches.softswitch(0x02); // RAMRD OFF
      switches.softswitch(0x04); // RAMWRT OFF

      assert_eq!(selectors.zp_stack.get(), (0, 0));
      assert_eq!(selectors.low_segment.get(), (0, 0));
      assert_eq!(selectors.text_page_1.get(), (0, 0));
      assert_eq!(selectors.text_page_2.get(), (0, 0));
      assert_eq!(selectors.hires_page_1.get(), (0, 0));
      assert_eq!(selectors.hires_page_2.get(), (0, 0));

      switches.softswitch(0x01); // 80STORE ON
      switches.softswitch(0x54); // PAGE2 OFF
      switches.softswitch(0x56 + hires); // HIRES OFF or ON
      switches.softswitch(0x02); // RAMRD OFF
      switches.softswitch(0x04); // RAMWRT OFF

      assert_eq!(selectors.zp_stack.get(), (0, 0));
      assert_eq!(selectors.low_segment.get(), (0, 0));
      assert_eq!(selectors.text_page_1.get(), (0, 0));
      assert_eq!(selectors.text_page_2.get(), (0, 0));
      assert_eq!(selectors.hires_page_1.get(), (0, 0));
      assert_eq!(selectors.hires_page_2.get(), (0, 0));
    }
  }

  #[test]
  fn fig4_right() {
    for hires in 0..=1 {
      let platform = Arc::new(TextPlatform::new());
      let selectors = AiieBankSelectors::new();
      let mut switches = AiieSoftSwitches::new(platform.provider(), selectors.clone());

      switches.softswitch(0x00); // 80STORE OFF
      switches.softswitch(0x54); // PAGE2 OFF (should also be on)
      switches.softswitch(0x56 + hires); // HIRES OFF or ON
      switches.softswitch(0x03); // RAMRD ON
      switches.softswitch(0x05); // RAMWRT ON

      assert_eq!(selectors.zp_stack.get(), (0, 0));
      assert_eq!(selectors.low_segment.get(), (1, 1));
      assert_eq!(selectors.text_page_1.get(), (1, 1));
      assert_eq!(selectors.text_page_2.get(), (1, 1));
      assert_eq!(selectors.hires_page_1.get(), (1, 1));
      assert_eq!(selectors.hires_page_2.get(), (1, 1));

      switches.softswitch(0x01); // 80STORE ON
      switches.softswitch(0x55); // PAGE2 ON
      switches.softswitch(0x56 + hires); // HIRES OFF or ON
      switches.softswitch(0x03); // RAMRD ON
      switches.softswitch(0x05); // RAMWRT ON

      assert_eq!(selectors.zp_stack.get(), (0, 0));
      assert_eq!(selectors.low_segment.get(), (1, 1));
      assert_eq!(selectors.text_page_1.get(), (1, 1));
      assert_eq!(selectors.text_page_2.get(), (1, 1));
      assert_eq!(selectors.hires_page_1.get(), (1, 1));
      assert_eq!(selectors.hires_page_2.get(), (1, 1));
    }
  }

  #[test]
  fn fig5_left() {
    let platform = Arc::new(TextPlatform::new());
    let selectors = AiieBankSelectors::new();
    let mut switches = AiieSoftSwitches::new(platform.provider(), selectors.clone());

    switches.softswitch(0x01); // 80STORE ON
    switches.softswitch(0x55); // PAGE2 ON
    switches.softswitch(0x56); // HIRES OFF
    switches.softswitch(0x02); // RAMRD OFF
    switches.softswitch(0x04); // RAMWRT OFF

    assert_eq!(selectors.zp_stack.get(), (0, 0));
    assert_eq!(selectors.low_segment.get(), (0, 0));
    assert_eq!(selectors.text_page_1.get(), (1, 1));
    assert_eq!(selectors.text_page_2.get(), (0, 0));
    assert_eq!(selectors.hires_page_1.get(), (0, 0));
    assert_eq!(selectors.hires_page_2.get(), (0, 0));
  }

  #[test]
  fn fig5_right() {
    let platform = Arc::new(TextPlatform::new());
    let selectors = AiieBankSelectors::new();
    let mut switches = AiieSoftSwitches::new(platform.provider(), selectors.clone());

    switches.softswitch(0x01); // 80STORE ON
    switches.softswitch(0x55); // PAGE2 ON
    switches.softswitch(0x57); // HIRES ON
    switches.softswitch(0x02); // RAMRD OFF
    switches.softswitch(0x04); // RAMWRT OFF

    assert_eq!(selectors.zp_stack.get(), (0, 0));
    assert_eq!(selectors.low_segment.get(), (0, 0));
    assert_eq!(selectors.text_page_1.get(), (1, 1));
    assert_eq!(selectors.text_page_2.get(), (0, 0));
    assert_eq!(selectors.hires_page_1.get(), (1, 1));
    assert_eq!(selectors.hires_page_2.get(), (0, 0));
  }

  #[test]
  fn fig6_left() {
    let platform = Arc::new(TextPlatform::new());
    let selectors = AiieBankSelectors::new();
    let mut switches = AiieSoftSwitches::new(platform.provider(), selectors.clone());

    switches.softswitch(0x01); // 80STORE ON
    switches.softswitch(0x54); // PAGE2 OFF
    switches.softswitch(0x56); // HIRES OFF
    switches.softswitch(0x03); // RAMRD ON
    switches.softswitch(0x05); // RAMWRT ON

    assert_eq!(selectors.zp_stack.get(), (0, 0));
    assert_eq!(selectors.low_segment.get(), (1, 1));
    assert_eq!(selectors.text_page_1.get(), (0, 0));
    assert_eq!(selectors.text_page_2.get(), (1, 1));
    assert_eq!(selectors.hires_page_1.get(), (1, 1));
    assert_eq!(selectors.hires_page_2.get(), (1, 1));
  }

  #[test]
  fn fig6_right() {
    let platform = Arc::new(TextPlatform::new());
    let selectors = AiieBankSelectors::new();
    let mut switches = AiieSoftSwitches::new(platform.provider(), selectors.clone());

    switches.softswitch(0x01); // 80STORE ON
    switches.softswitch(0x54); // PAGE2 OFF
    switches.softswitch(0x57); // HIRES ON
    switches.softswitch(0x03); // RAMRD ON
    switches.softswitch(0x05); // RAMWRT ON

    assert_eq!(selectors.zp_stack.get(), (0, 0));
    assert_eq!(selectors.low_segment.get(), (1, 1));
    assert_eq!(selectors.text_page_1.get(), (0, 0));
    assert_eq!(selectors.text_page_2.get(), (1, 1));
    assert_eq!(selectors.hires_page_1.get(), (0, 0));
    assert_eq!(selectors.hires_page_2.get(), (1, 1));
  }

  // http://www.applelogic.org/files/AIIETECHREF2.pdf
  #[test]
  fn test_c08x() {
    let platform = Arc::new(TextPlatform::new());
    let selectors = AiieBankSelectors::new();
    let mut switches = AiieSoftSwitches::new(platform.provider(), selectors.clone());

    switches.read(0x80);
    assert_eq!(selectors.rom_ram_select.get(), (1, 0));
    assert_eq!(selectors.ram_bank_select.get(), (1, 1));

    switches.read(0x81);
    assert_eq!(selectors.rom_ram_select.get(), (0, 1));
    assert_eq!(selectors.ram_bank_select.get(), (1, 1));

    switches.read(0x82);
    assert_eq!(selectors.rom_ram_select.get(), (0, 0));
    assert_eq!(selectors.ram_bank_select.get(), (1, 1));

    switches.read(0x83);
    assert_eq!(selectors.rom_ram_select.get(), (1, 1));
    assert_eq!(selectors.ram_bank_select.get(), (1, 1));

    switches.read(0x88);
    assert_eq!(selectors.rom_ram_select.get(), (1, 0));
    assert_eq!(selectors.ram_bank_select.get(), (0, 0));

    switches.read(0x89);
    assert_eq!(selectors.rom_ram_select.get(), (0, 1));
    assert_eq!(selectors.ram_bank_select.get(), (0, 0));

    switches.read(0x8A);
    assert_eq!(selectors.rom_ram_select.get(), (0, 0));
    assert_eq!(selectors.ram_bank_select.get(), (0, 0));

    switches.read(0x8B);
    assert_eq!(selectors.rom_ram_select.get(), (1, 1));
    assert_eq!(selectors.ram_bank_select.get(), (0, 0));
  }

  #[test]
  fn test_rdbnk2() {
    let platform = Arc::new(TextPlatform::new());
    let selectors = AiieBankSelectors::new();
    let mut switches = AiieSoftSwitches::new(platform.provider(), selectors.clone());

    switches.read(0x80);
    assert_eq!(switches.read(0x11) & 0x80, 0x80);
    switches.read(0x88);
    assert_eq!(switches.read(0x11) & 0x80, 0x00);
  }

  #[test]
  fn test_rdlcram() {
    let platform = Arc::new(TextPlatform::new());
    let selectors = AiieBankSelectors::new();
    let mut switches = AiieSoftSwitches::new(platform.provider(), selectors.clone());

    switches.read(0x80);
    assert_eq!(switches.read(0x12) & 0x80, 0x80);
    switches.read(0x81);
    assert_eq!(switches.read(0x12) & 0x80, 0x00);
  }

  #[test]
  fn test_altzp() {
    let platform = Arc::new(TextPlatform::new());
    let selectors = AiieBankSelectors::new();
    let mut switches = AiieSoftSwitches::new(platform.provider(), selectors.clone());

    switches.softswitch(0x08);
    assert_eq!(selectors.zp_stack.get(), (0, 0));
    assert_eq!(selectors.upper_ram.get(), (0, 0));

    switches.softswitch(0x09);
    assert_eq!(selectors.zp_stack.get(), (1, 1));
    assert_eq!(selectors.upper_ram.get(), (1, 1));
  }

  #[test]
  fn test_rdaltzp() {
    let platform = Arc::new(TextPlatform::new());
    let selectors = AiieBankSelectors::new();
    let mut switches = AiieSoftSwitches::new(platform.provider(), selectors.clone());

    switches.softswitch(0x08);
    assert_eq!(switches.read(0x16) & 0x80, 0x00);

    switches.softswitch(0x09);
    assert_eq!(switches.read(0x16) & 0x80, 0x80);
  }
}
