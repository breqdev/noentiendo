use crate::graphics::WinitGraphicsProvider;
use crate::memory::{
  easy::{EasyIO, EasyVram},
  BlockMemory, BranchMemory, MappedIO, Memory,
};

pub fn brooke(rom: &str) -> Box<dyn Memory> {
  let ram = BlockMemory::new(0x4000);
  let io = MappedIO::new();
  let rom = BlockMemory::from_file(0x8000, rom);

  let memory = BranchMemory::new()
    .map(0x0000, Box::new(ram))
    .map(0x4000, Box::new(io))
    .map(0x8000, Box::new(rom));

  Box::new(memory)
}

pub fn easy(rom: &str) -> Box<dyn Memory> {
  let zero_page = BlockMemory::new(0x00fe);
  let io = EasyIO::new();
  let stack_ram = BlockMemory::new(0x0100);
  let vram = {
    let graphics = WinitGraphicsProvider::new();
    EasyVram::new(32, 32, Box::new(graphics))
  };
  let high_ram = BlockMemory::new(0x7A00);
  let rom = BlockMemory::from_file(0x8000, rom);

  let memory = BranchMemory::new()
    .map(0x0000, Box::new(zero_page))
    .map(0x00fe, Box::new(io))
    .map(0x0100, Box::new(stack_ram))
    .map(0x0200, Box::new(vram))
    .map(0x0600, Box::new(high_ram))
    .map(0x8000, Box::new(rom));

  Box::new(memory)
}
