#[cfg(not(target_arch = "wasm32"))]
use libnoentiendo::{
  keyboard::KeyMappingStrategy,
  platform::{SyncPlatform, TextPlatform, WinitPlatform},
  roms::DiskLoadable,
  systems::{
    aiie::{AiieSystemBuilder, AiieSystemConfig, AiieSystemRoms},
    basic::BasicSystemBuilder,
    c64::C64SystemBuilder,
    c64::C64SystemConfig,
    c64::C64SystemRoms,
    easy::Easy6502SystemBuilder,
    klaus::KlausSystemBuilder,
    pet::PetSystemBuilder,
    pet::PetSystemConfig,
    pet::PetSystemRoms,
    vic::Vic20SystemBuilder,
    vic::Vic20SystemConfig,
    vic::Vic20SystemRoms,
    SystemBuilder,
  },
};

#[cfg(not(target_arch = "wasm32"))]
use clap::{Parser, ValueEnum};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum SystemArg {
  Basic,
  Easy,
  Klaus,
  Pet,
  Vic,
  C64,
  Aiie,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum PlatformArg {
  Text,
  Winit,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum KeyMappingArg {
  Symbolic,
  Physical,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
  #[clap(short, long, value_parser, default_value = "")]
  rom_path: String,

  #[clap(short, long, value_parser)]
  system: SystemArg,

  #[clap(short, long, value_parser, default_value = "text")]
  platform: PlatformArg,

  #[clap(short, long, value_parser, default_value = "symbolic")]
  key_mapping: KeyMappingArg,
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
  let args = Args::parse();

  let mut platform: Box<dyn SyncPlatform> = match args.platform {
    PlatformArg::Text => Box::new(TextPlatform::new()),
    PlatformArg::Winit => Box::new(WinitPlatform::new()),
  };

  let romfile = match args.rom_path.as_str() {
    "" => None,
    _ => Some(libnoentiendo::roms::RomFile::from_file(&args.rom_path)),
  };

  let mapping = match args.key_mapping {
    KeyMappingArg::Symbolic => KeyMappingStrategy::Symbolic,
    KeyMappingArg::Physical => KeyMappingStrategy::Physical,
  };

  let system = match args.system {
    SystemArg::Basic => BasicSystemBuilder::build(romfile.unwrap(), (), platform.provider()),
    SystemArg::Easy => Easy6502SystemBuilder::build(romfile.unwrap(), (), platform.provider()),
    SystemArg::Klaus => KlausSystemBuilder::build(romfile.unwrap(), (), platform.provider()),
    SystemArg::Pet => PetSystemBuilder::build(
      PetSystemRoms::from_disk(),
      PetSystemConfig { mapping },
      platform.provider(),
    ),
    SystemArg::Vic => Vic20SystemBuilder::build(
      Vic20SystemRoms::from_disk(match romfile {
        Some(_) => Some(args.rom_path.as_str()),
        None => None,
      }),
      Vic20SystemConfig { mapping },
      platform.provider(),
    ),
    SystemArg::C64 => C64SystemBuilder::build(
      C64SystemRoms::from_disk(),
      C64SystemConfig { mapping },
      platform.provider(),
    ),
    SystemArg::Aiie => AiieSystemBuilder::build(
      AiieSystemRoms::from_disk(),
      AiieSystemConfig { mapping },
      platform.provider(),
    ),
  };

  platform.run(system);
}
