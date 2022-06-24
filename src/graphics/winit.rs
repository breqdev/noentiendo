use crate::graphics::{Color, GraphicsProvider};
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::VirtualKeyCode;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::run_return::EventLoopExtRunReturn;
use winit::window::{Window, WindowBuilder};
use winit_input_helper::WinitInputHelper;

pub struct WinitGraphicsProvider {
  event_loop: EventLoop<()>,
  input: WinitInputHelper,
  window: Option<Window>,
  pixels: Option<Pixels>,
  dimensions: Option<(u32, u32)>,
}

impl WinitGraphicsProvider {
  pub fn new() -> Self {
    let event_loop = EventLoop::new();
    let input = WinitInputHelper::new();

    Self {
      event_loop,
      input,
      window: None,
      pixels: None,
      dimensions: None,
    }
  }
}

impl GraphicsProvider for WinitGraphicsProvider {
  fn create_window(&mut self, width: u32, height: u32) {
    let window = WindowBuilder::new()
      .with_title("noentiendo")
      .with_inner_size(LogicalSize::new(width as f64, height as f64))
      .build(&self.event_loop)
      .unwrap();

    let inner_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(inner_size.width, inner_size.height, &window);
    let pixels = Pixels::new(width, height, surface_texture).unwrap();

    self.window = Some(window);
    self.pixels = Some(pixels);
    self.dimensions = Some((width, height));
  }

  fn tick(&mut self) {
    let pixels = self.pixels.as_mut().unwrap();

    self.event_loop.run_return(|event, _, control_flow| {
      if self.input.update(&event) {
        if self.input.key_pressed(VirtualKeyCode::Escape) || self.input.quit() {
          panic!("Quit");
        }

        if let Some(size) = self.input.window_resized() {
          pixels.resize_surface(size.width, size.height);
        }
      }

      *control_flow = ControlFlow::Exit;
    });

    self.pixels.as_ref().unwrap().render().unwrap();
  }

  fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
    let frame = self.pixels.as_mut().unwrap().get_frame();
    let (width, _height) = self.dimensions.unwrap();
    let index = ((y * width + x) * 4) as usize;
    let pixel = &mut frame[index..(index + 4)];
    pixel.copy_from_slice(&color.to_rgba());
  }
}
