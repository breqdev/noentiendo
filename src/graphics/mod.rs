mod sdl;

pub use sdl::SdlGraphicsProvider;

#[derive(Copy, Clone, Debug)]
pub struct Color {
  r: u8,
  g: u8,
  b: u8,
}

impl Color {
  pub fn new(r: u8, g: u8, b: u8) -> Self {
    Self { r, g, b }
  }
}

pub trait GraphicsProvider {
  fn create_window(&mut self, width: u32, height: u32);
  fn tick(&mut self);
  fn set_pixel(&mut self, x: u32, y: u32, color: Color);
}
