use crate::graphics::GraphicsProvider;
use crate::memory::{
  easy::{EasyIO, EasyVram},
  pet::{PetPia1PortA, PetPia1PortB, PetVram},
  pia::{NullPort, PIA},
  BlockMemory, BranchMemory, MappedStdIO, Memory, NullMemory,
};
use std::sync::Arc;

pub enum Mapping {
  BrookeSystem,
  Easy6502,
  CommodorePET,
}

pub fn create_memory(
  mapping: Mapping,
  graphics: Arc<dyn GraphicsProvider>,
  rom: &str,
) -> Box<dyn Memory> {
  match mapping {
    Mapping::BrookeSystem => {
      let ram = BlockMemory::ram(0x4000);
      let io = MappedStdIO::new();
      let rom = BlockMemory::from_file(0x8000, rom);

      let memory = BranchMemory::new()
        .map(0x0000, Box::new(ram))
        .map(0x4000, Box::new(io))
        .map(0x8000, Box::new(rom));

      Box::new(memory)
    }
    Mapping::Easy6502 => {
      let zero_page = BlockMemory::ram(0x0100);
      let io = EasyIO::new(graphics.clone());
      let stack_ram = BlockMemory::ram(0x0100);
      let vram = EasyVram::new(32, 32, graphics);
      let high_ram = BlockMemory::ram(0x7A00);
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
      let ram = BlockMemory::ram(0x8000);
      let vram = PetVram::new("bin/pet/char.bin", graphics);

      let expansion_rom_9 = NullMemory::new();
      let expansion_rom_a = NullMemory::new();
      let expansion_rom_b = NullMemory::new();

      let basic_rom = BlockMemory::from_file(0x2000, "bin/pet/basic.bin");

      let editor_rom = BlockMemory::from_file(0x1000, "bin/pet/editor.bin");

      let pia1 = PIA::new(Box::new(PetPia1PortA::new()), Box::new(PetPia1PortB::new()));
      let pia2 = PIA::new(Box::new(NullPort::new()), Box::new(NullPort::new()));
      let via = PIA::new(Box::new(NullPort::new()), Box::new(NullPort::new()));

      let kernel_rom = BlockMemory::from_file(0x1000, "bin/pet/kernal.bin");

      let memory = BranchMemory::new()
        .map(0x0000, Box::new(ram))
        .map(0x8000, Box::new(vram))
        .map(0x9000, Box::new(expansion_rom_9))
        .map(0xA000, Box::new(expansion_rom_a))
        .map(0xB000, Box::new(expansion_rom_b))
        .map(0xC000, Box::new(basic_rom))
        .map(0xE000, Box::new(editor_rom))
        .map(0xE810, Box::new(pia1))
        .map(0xE820, Box::new(pia2))
        .map(0xE840, Box::new(via))
        .map(0xF000, Box::new(kernel_rom));

      Box::new(memory)
    }
  }
}
