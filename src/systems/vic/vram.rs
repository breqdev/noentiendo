use crate::memory::{ActiveInterrupt, Memory, SystemInfo};
use crate::systems::vic::VicChip;
use std::sync::{Arc, Mutex};

/// Represents the video/screen RAM used by the MOS 6560 VIC.
/// Each address corresponds to a pixel on the screen.
/// Addresses are laid out left-to-right, top-to-bottom.
pub struct VicVram {
  chip: Arc<Mutex<VicChip>>,
}

impl VicVram {
  pub fn new(chip: Arc<Mutex<VicChip>>) -> VicVram {
    VicVram { chip }
  }
}

impl Memory for VicVram {
  fn read(&mut self, address: u16) -> u8 {
    let mut chip = self.chip.lock().unwrap();
    chip.read_vram(address)
  }

  fn write(&mut self, address: u16, value: u8) {
    let mut chip = self.chip.lock().unwrap();
    chip.write_vram(address, value)
  }

  fn reset(&mut self) {}

  fn poll(&mut self, _info: &SystemInfo) -> ActiveInterrupt {
    ActiveInterrupt::None
  }
}
