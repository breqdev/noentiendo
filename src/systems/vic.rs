pub struct Vic20SystemFactory {}

impl SystemFactory<PetSystemRoms> for Vic20SystemFactory {
  fn create(roms: PetSystemRoms, platform: Arc<dyn PlatformProvider>) -> System {
    let low_ram = BlockMemory::ram(0x0400);
    let main_ram = BlockMemory::ram(0x1000);

    let characters = BlockMemory::rom(0x1000);

    let basic_rom = BlockMemory::from_file(0x2000, roms.basic);

    let kernel_rom = BlockMemory::from_file(0x2000, roms.kernal);

    let memory = BranchMemory::new()
      .map(0x0000, Box::new(low_ram))
      .map(0x1000, Box::new(main_ram))
      .map(0x8000, Box::new(characters))
      .map(0xC000, Box::new(basic_rom))
      .map(0xE000, Box::new(kernel_rom));

    System::new(Box::new(memory), 1_000_000)
  }
}
