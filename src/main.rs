#[cfg(not(target_arch = "wasm32"))]
use libnoentiendo::{
  keyboard::KeyMappingStrategy,
  platform::{EguiPlatform, SyncPlatform, TextPlatform, WinitPlatform},
  roms::DiskLoadable,
  systems::{
    basic::BasicSystemBuilder, c64::C64SystemBuilder, c64::C64SystemConfig, c64::C64SystemRoms,
    easy::Easy6502SystemBuilder, klaus::KlausSystemBuilder, pet::PetSystemBuilder,
    pet::PetSystemConfig, pet::PetSystemRoms, vic::Vic20SystemBuilder, vic::Vic20SystemConfig,
    vic::Vic20SystemRoms, SystemBuilder,
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

  #[clap(short, long, value_parser, default_value = "symbolic")]
  key_mapping: String,
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
  let args = Args::parse();

  let mut platform: Box<dyn SyncPlatform> = match args.platform.as_str() {
    "text" => Box::new(TextPlatform::new()),
    "winit" => Box::new(WinitPlatform::new()),
    "egui" => Box::new(EguiPlatform::new()),
    _ => panic!("Unknown platform"),
  };

  let romfile = match args.rom_path.as_str() {
    "" => None,
    _ => Some(libnoentiendo::roms::RomFile::from_file(&args.rom_path)),
  };

  let mapping = match args.key_mapping.as_str() {
    "symbolic" => KeyMappingStrategy::Symbolic,
    "physical" => KeyMappingStrategy::Physical,
    _ => panic!("Unknown key mapping"),
  };

  let system = match args.system.as_str() {
    "basic" => BasicSystemBuilder::build(romfile.unwrap(), (), platform.provider()),
    "easy" => Easy6502SystemBuilder::build(romfile.unwrap(), (), platform.provider()),
    "klaus" => KlausSystemBuilder::build(romfile.unwrap(), (), platform.provider()),
    "pet" => PetSystemBuilder::build(
      PetSystemRoms::from_disk(),
      PetSystemConfig { mapping },
      platform.provider(),
    ),
    "vic" => Vic20SystemBuilder::build(
      Vic20SystemRoms::from_disk(match romfile {
        Some(_) => Some(args.rom_path.as_str()),
        None => None,
      }),
      Vic20SystemConfig { mapping },
      platform.provider(),
    ),
    "c64" => C64SystemBuilder::build(
      C64SystemRoms::from_disk(),
      C64SystemConfig { mapping },
      platform.provider(),
    ),
    _ => panic!("Unknown system"),
  };

  platform.run(system);
}
