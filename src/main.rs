mod execute;
mod fetch;
mod memory;
mod registers;
mod system;

fn main() {
  let ram = Box::new(memory::BlockMemory::new(14));
  let io = Box::new(memory::MappedIO::new());
  let rom = Box::new(memory::BlockMemory::from_file(15, "bin/syscall.bin"));

  let low = Box::new(memory::BranchMemory::new(ram, io, 15));
  let memory = Box::new(memory::BranchMemory::new(low, rom, 16));

  let mut system = system::System::new(memory);

  system.reset();

  // Run program for a while (contains an infinite loop)
  for _ in 0..200 {
    system.tick();
  }
}
