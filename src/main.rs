mod execute;
mod fetch;
mod graphics;
mod memory;
mod registers;
mod system;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
  #[clap(short, long, value_parser)]
  rom_path: String,

  #[clap(short, long, value_parser)]
  system: String,

  #[clap(short, long, value_parser)]
  graphics: String,
}

fn main() {
  let args = Args::parse();

  let graphics: Option<Box<dyn graphics::GraphicsProvider>> = match args.graphics.as_str() {
    "none" => None,
    "winit" => Some(Box::new(graphics::WinitGraphicsProvider::new())),
    _ => panic!("Unknown graphics provider"),
  };

  let mapping = match args.system.as_str() {
    "brooke" => memory::systems::Mapping::BrookeSystem,
    "easy" => memory::systems::Mapping::Easy6502,
    "pet" => memory::systems::Mapping::CommodorePET,
    _ => panic!("Unknown system"),
  };

  let memory = memory::systems::create_memory(mapping, graphics, &args.rom_path);

  let mut system = system::System::new(memory);

  system.reset();

  loop {
    system.tick();
  }
}
