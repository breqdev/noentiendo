use crate::graphics::GraphicsProvider;
use crate::memory::{
  easy::{EasyIO, EasyVram},
  pet::PetVram,
  BlockMemory, BranchMemory, MappedStdIO, Memory, NullMemory,
};
use std::cell::RefCell;
use std::rc::Rc;

pub enum Mapping {
  BrookeSystem,
  Easy6502,
  CommodorePET,
}

pub fn create_memory(
  mapping: Mapping,
  graphics: Option<Box<dyn GraphicsProvider>>,
  rom: &str,
) -> Box<dyn Memory> {
  match mapping {
    Mapping::BrookeSystem => {
      let ram = BlockMemory::new(0x4000);
      let io = MappedStdIO::new();
      let rom = BlockMemory::from_file(0x8000, rom);

      let memory = BranchMemory::new()
        .map(0x0000, Box::new(ram))
        .map(0x4000, Box::new(io))
        .map(0x8000, Box::new(rom));

      Box::new(memory)
    }
    Mapping::Easy6502 => {
      let graphics = Rc::new(RefCell::new(graphics.unwrap()));

      let zero_page = BlockMemory::new(0x0100);
      let io = EasyIO::new(Rc::clone(&graphics));
      let stack_ram = BlockMemory::new(0x0100);
      let vram = EasyVram::new(32, 32, graphics);
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
    Mapping::CommodorePET => {
      let graphics = Rc::new(RefCell::new(graphics.unwrap()));

      let ram = BlockMemory::new(0x8000);
      let vram = PetVram::new("bin/pet_char.bin", Rc::clone(&graphics));

      let expansion_rom_9 = NullMemory::new();
      let expansion_rom_a = NullMemory::new();
      let expansion_rom_b = NullMemory::new();

      let basic_rom_c = NullMemory::new();
      let basic_rom_d = NullMemory::new();

      let editor_rom = NullMemory::new();

      let io = NullMemory::new();

      let kernel_rom = BlockMemory::from_file(0x1000, rom); // TODO: actual kernel

      let memory = BranchMemory::new()
        .map(0x0000, Box::new(ram))
        .map(0x8000, Box::new(vram))
        .map(0x9000, Box::new(expansion_rom_9))
        .map(0xA000, Box::new(expansion_rom_a))
        .map(0xB000, Box::new(expansion_rom_b))
        .map(0xC000, Box::new(basic_rom_c))
        .map(0xD000, Box::new(basic_rom_d))
        .map(0xE000, Box::new(editor_rom))
        .map(0xE800, Box::new(io))
        .map(0xF000, Box::new(kernel_rom));

      Box::new(memory)
    }
  }
}
