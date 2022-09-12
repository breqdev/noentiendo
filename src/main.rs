#[cfg(not(target_arch = "wasm32"))]
use libnoentiendo::{
  platform::{Platform, TextPlatform, WinitPlatform},
  systems::pet::PetSystemRoms,
  systems::{
    BrookeSystemFactory, EasySystemFactory, KlausSystemFactory, PetSystemFactory, SystemFactory,
  },
};

#[cfg(not(target_arch = "wasm32"))]
use clap::Parser;

#[cfg(not(target_arch = "wasm32"))]
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
  #[clap(short, long, value_parser, default_value = "")]
  rom_path: String,

  #[clap(short, long, value_parser)]
  system: String,

  #[clap(short, long, value_parser, default_value = "text")]
  platform: String,
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
  let args = Args::parse();

  let mut platform: Box<dyn Platform> = match args.platform.as_str() {
    "text" => Box::new(TextPlatform::new()),
    "winit" => Box::new(WinitPlatform::new()),
    _ => panic!("Unknown platform"),
  };

  let romfile = match args.rom_path.as_str() {
    "" => None,
    _ => Some(libnoentiendo::memory::RomFile::from_file(&args.rom_path)),
  };

  let system = match args.system.as_str() {
    "brooke" => BrookeSystemFactory::create(romfile.unwrap(), platform.provider()),
    "easy" => EasySystemFactory::create(romfile.unwrap(), platform.provider()),
    "klaus" => KlausSystemFactory::create(romfile.unwrap(), platform.provider()),
    "pet" => PetSystemFactory::create(PetSystemRoms::from_disk(), platform.provider()),
    _ => panic!("Unknown system"),
  };

  platform.run(system);
}
