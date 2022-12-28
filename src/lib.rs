#![doc = include_str!("../README.md")]
#![allow(clippy::new_without_default)]

/// The [`cpu::Mos6502`] represents a 6502 processor and associated memory.
pub mod cpu;

/// A [`memory::Memory`] implementation can be read from and written to, but it can also be polled for interrupts. This is used for the PIA, VIA, and other chips that interface over memory but also trigger interrupts. The [`memory`] module provides implementations for various types of memory and other memory-mapped devices. Mappings are handled using [`memory::BranchMemory`].
///
/// Off-cycle memory access schemes such as the one used in the VIC-20 are handled using the [`memory::DMA`] trait. Systems may have objects which implement this trait attached, and these objects will have a chance to access memory in between clock cycles.
pub mod memory;

/// Various representations of keyboard scancodes are required in different parts of the codebase. Each platform typically has its own definition of a scancode (e.g. JavaScript's `event.code` or Winit's `VirtualKeyCode`), and each emulated system has a different set of keys (e.g. the `Commodore` key on the VIC-20 or the standalone `"` key on the PET).
///
/// Utilities for handling keyboard input are defined in the [`keyboard`] module. This module defines a [`keyboard::KeyPosition`] enum to represent the physical keys which appear on a modern host keyboard and a [`keyboard::KeySymbol`] enum to represent the symbols which can be typed with a modern host keyboard. It also defines a [`keyboard::KeyState<T>`] struct to represent the set of currently-pressed keys in some representation `T`.
///
/// Mapping between different keyboard representations is handled using [`keyboard::KeyAdapter<F, T>`] implementations. These exist for four different scenarios:
///
/// - Mapping from platform-specific scancodes to the common [`keyboard::KeyPosition`] representation
/// - Mapping from [`keyboard::KeyPosition`] to system-specific scancodes, using a one-to-one "physical" mapping strategy
/// - Mapping from [`keyboard::KeyPosition`] to [`keyboard::KeySymbol`], respecting the currently-pressed modifier keys
/// - Mapping from [`keyboard::KeySymbol`] to system-specific scancodes, to preserve the symbols that the user pressed even if it requires rewriting the currently-pressed modifier keys
pub mod keyboard;

/// A [`platform::Platform`] consumes a system and runs it. Platforms provide access to the video output, keyboard input, system random number generator, and other details via a [`platform::PlatformProvider`]. Some platforms run synchronously (taking over the thread) while others run asynchronously with the help of an event loop (such as when compiling to WASM). Platforms are defined in the [`platform`] module.

/// Currently, available platforms include `TextPlatform` for simple headless text-based operation, `WinitPlatform` for a graphical window on a desktop environment, and `CanvasPlatform` for drawing to a `<canvas>` element on the web. In the future, platforms for mobile apps are planned, in addition to a platform for running on a microcontroller (e.g. the RP2040).
pub mod platform;

/// ROM file loading and unloading is different on different platforms: desktop platforms typically load ROMs from a file, while WebAssembly platforms need to load ROMs from a `Uint8Array`. ROM file definition and loading is handled in the [`roms`] module, with specific [`roms::DiskLoadable`] and `roms::JsValueLoadable` traits for these two cases. Loaded ROMs are represented with a [`roms::RomFile`] object, which can be passed to [`memory::BlockMemory::from_file`].
pub mod roms;

/// Systems are created by a [`systems::SystemBuilder`]. A system is created with some roms, configuration, and platform. For instance, the `build` implementation on [`systems::pet::PetSystemBuilder`] takes in [`systems::pet::PetSystemRoms`], [`systems::pet::PetSystemConfig`], and an `Arc<dyn PlatformProvider>`.
pub mod systems;

mod time;

#[cfg(target_arch = "wasm32")]
extern crate console_error_panic_hook;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn main(roms: &JsValue, system: &JsValue) {
  console_error_panic_hook::set_once();

  use js_sys::Reflect;
  use keyboard::KeyMappingStrategy;
  use platform::{AsyncPlatform, CanvasPlatform, Platform};
  use systems::{
    pet::PetSystemBuilder, pet::PetSystemConfig, pet::PetSystemRoms, vic::Vic20SystemBuilder,
    vic::Vic20SystemConfig, vic::Vic20SystemRoms, SystemBuilder,
  };
  use wasm_bindgen_futures::spawn_local;

  let mut platform = CanvasPlatform::new();

  let pet_object = Reflect::get(&roms, &JsValue::from_str("pet")).unwrap();
  let vic_object = Reflect::get(&roms, &JsValue::from_str("vic")).unwrap();

  let pet_roms = PetSystemRoms::from_jsvalue(&pet_object);
  let vic_roms = Vic20SystemRoms::from_jsvalue(&vic_object);

  let system = match system.as_string().unwrap().as_str() {
    "pet" => PetSystemBuilder::build(
      pet_roms,
      PetSystemConfig {
        mapping: KeyMappingStrategy::Symbolic,
      },
      platform.provider(),
    ),
    "vic" => Vic20SystemBuilder::build(
      vic_roms,
      Vic20SystemConfig {
        mapping: KeyMappingStrategy::Symbolic,
      },
      platform.provider(),
    ),
    _ => panic!("Unknown system"),
  };

  spawn_local(async move {
    platform.run_async(system).await;
  });
}
