use crate::memory::{BlockMemory, BranchMemory, MappedIO, Memory};

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
