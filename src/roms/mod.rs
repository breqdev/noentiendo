mod romfile;

#[cfg(not(target_arch = "wasm32"))]
mod disk;

#[cfg(target_arch = "wasm32")]
mod wasm;

pub use romfile::RomFile;

#[cfg(not(target_arch = "wasm32"))]
pub use disk::DiskLoadable;

#[cfg(target_arch = "wasm32")]
pub use wasm::JsValueLoadable;
