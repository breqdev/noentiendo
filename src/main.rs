#[cfg(not(target_arch = "wasm32"))]
use libnoentiendo::{
  keyboard::KeyMappingStrategy,
  platform::{SyncPlatform, TextPlatform, WinitPlatform},
  roms::DiskLoadable,
  systems::{
    aiie::{AiieSystem, AiieSystemConfig, AiieSystemRoms},
    basic::BasicSystem,
    c64::{C64System, C64SystemConfig, C64SystemRoms},
    easy::Easy6502System,
    klaus::KlausSystem,
    pet::{PetSystem, PetSystemConfig, PetSystemRoms},
    vic::{Vic20System, Vic20SystemConfig, Vic20SystemRoms},
    BuildableSystem,
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

  #[clap(short, long, value_parser, default_value = "false")]
  trace: bool,
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
  use libnoentiendo::{
    cpu::mos6502::Mos6502Variant, systems::klaus::KlausSystemConfig, trace::file::FileTraceHandler,
  };

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

  let mut system = match args.system {
    SystemArg::Basic => BasicSystem::build(romfile.unwrap(), (), platform.provider()),
    SystemArg::Easy => Easy6502System::build(romfile.unwrap(), (), platform.provider()),
    SystemArg::Klaus => KlausSystem::build(
      romfile.unwrap(),
      KlausSystemConfig {
        pc_report: None,
        variant: Mos6502Variant::NMOS,
      },
      platform.provider(),
    ),
    SystemArg::Pet => PetSystem::build(
      PetSystemRoms::from_disk(),
      PetSystemConfig { mapping },
      platform.provider(),
    ),
    SystemArg::Vic => Vic20System::build(
      Vic20SystemRoms::from_disk(match romfile {
        Some(_) => Some(args.rom_path.as_str()),
        None => None,
      }),
      Vic20SystemConfig { mapping },
      platform.provider(),
    ),
    SystemArg::C64 => C64System::build(
      C64SystemRoms::from_disk(),
      C64SystemConfig { mapping },
      platform.provider(),
    ),
    SystemArg::Aiie => AiieSystem::build(
      AiieSystemRoms::from_disk(),
      AiieSystemConfig {},
      platform.provider(),
    ),
  };

  if args.trace {
    system.attach_trace_handler(Box::new(FileTraceHandler::new("./cpu.trace".to_owned())));
  }

  platform.run(system);
}
