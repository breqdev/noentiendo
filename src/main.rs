#[cfg(feature = "desktop")]
use libnoentiendo::{
  graphics::{GraphicsService, NullGraphicsService, WinitGraphicsService},
  memory::systems::{create_memory, Mapping},
  system::System,
};

#[cfg(feature = "desktop")]
use clap::Parser;
#[cfg(feature = "desktop")]
use std::thread;

#[cfg(feature = "desktop")]
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

#[cfg(feature = "desktop")]
fn main() {
  let args = Args::parse();

  let mut graphics: Box<dyn GraphicsService> = match args.graphics.as_str() {
    "none" => Box::new(NullGraphicsService::new()),
    "winit" => Box::new(WinitGraphicsService::new()),
    _ => panic!("Unknown graphics provider"),
  };

  let mapping = match args.system.as_str() {
    "brooke" => Mapping::BrookeSystem,
    "easy" => Mapping::Easy6502,
    "pet" => Mapping::CommodorePET,
    _ => panic!("Unknown system"),
  };

  let memory = create_memory(mapping, graphics.provider(), &args.rom_path);
  // let mut system = System::new(memory, 10000);
  let mut system = System::new(memory, 0);

  thread::spawn(move || {
    system.reset();

    loop {
      system.tick();
    }
  });

  graphics.run();
}

#[cfg(not(feature = "desktop"))]
fn main() {
  panic!("No supported platform found! Please enable the `desktop` feature.");
}
