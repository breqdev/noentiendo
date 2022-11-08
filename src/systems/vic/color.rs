use crate::memory::{ActiveInterrupt, Memory, SystemInfo};
use crate::systems::vic::VicChip;
use std::sync::{Arc, Mutex};

const WIDTH: u32 = 22;
const HEIGHT: u32 = 23;
const CHAR_WIDTH: u32 = 8;
const CHAR_HEIGHT: u32 = 8;
const VRAM_SIZE: usize = 512; // 6 extra bytes to make mapping easier

pub struct VicColorRam {
  chip: Arc<Mutex<VicChip>>,
}

impl VicColorRam {
  pub fn new(chip: Arc<Mutex<VicChip>>) -> Self {
    Self { chip }
  }
}

impl Memory for VicColorRam {
  fn read(&mut self, address: u16) -> u8 {
    let mut chip = self.chip.lock().unwrap();
    chip.read_color(address)
  }

  fn write(&mut self, address: u16, value: u8) {
    let mut chip = self.chip.lock().unwrap();
    chip.write_color(address, value)
  }

  fn reset(&mut self) {}

  fn poll(&mut self, _info: &SystemInfo) -> ActiveInterrupt {
    ActiveInterrupt::None
  }
}
