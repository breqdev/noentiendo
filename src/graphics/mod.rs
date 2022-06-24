mod winit;

pub use self::winit::WinitGraphicsProvider;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color {
  r: u8,
  g: u8,
  b: u8,
}

impl Color {
  pub fn new(r: u8, g: u8, b: u8) -> Self {
    Self { r, g, b }
  }

  pub fn to_rgba(&self) -> [u8; 4] {
    [self.r, self.g, self.b, 255]
  }
}

pub trait GraphicsProvider {
  fn create_window(&mut self, width: u32, height: u32, scale: f64);
  fn tick(&mut self);
  fn set_pixel(&mut self, x: u32, y: u32, color: Color);
  fn get_last_key(&self) -> u8;
}
