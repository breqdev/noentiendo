mod execute;
mod fetch;
mod graphics;
mod memory;
mod registers;
mod system;

use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();
  match args.len() {
    2 => {}
    _ => panic!("Usage: {} <file>", args[0]),
  }
  let filename = &args[1];

  let ram = Box::new(memory::BlockMemory::new(14));
  // let io = Box::new(memory::MappedIO::new());
  let io = Box::new(memory::EasyMemory::new(
    5,
    5,
    Box::new(graphics::SdlGraphicsProvider::new()),
  ));
  let rom = Box::new(memory::BlockMemory::from_file(15, filename));

  let low = Box::new(memory::BranchMemory::new(ram, io, 15));
  let memory = Box::new(memory::BranchMemory::new(low, rom, 16));

  let mut system = system::System::new(memory);

  system.reset();

  loop {
    system.tick();
  }
}
