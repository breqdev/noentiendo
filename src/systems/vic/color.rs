use crate::memory::{ActiveInterrupt, Memory, SystemInfo};
use crate::systems::vic::VicChip;
use std::sync::{Arc, Mutex};

/// Represents the color RAM used by the MOS 6560 VIC.
/// Each position corresponds to a pixel in the VIC's video RAM.
/// The value at each position is an index into the VIC's color palette.
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
