use crate::graphics::{Color, GraphicsProvider};
use beryllium::{
  event::Event,
  gl_window::{GlAttr, GlContextFlags, GlProfile, GlWindow},
  init::{InitFlags, Sdl},
  window::WindowFlags,
};
use core::{ptr::null, str};
use glitz::{println_gl_debug_callback, GlFns, GL_COLOR_BUFFER_BIT};
use zstring::zstr;

pub struct SdlGraphicsProvider {
  sdl: Sdl,
  gl_win: Option<GlWindow>,
  gl: Option<GlFns>,
}

impl SdlGraphicsProvider {
  pub fn new() -> Self {
    println!("Initializing SDL...");
    let sdl = Sdl::init(InitFlags::EVERYTHING).unwrap();
    sdl.allow_drop_events(true);

    const FLAGS: i32 = if cfg!(debug_assertions) {
      GlContextFlags::FORWARD_COMPATIBLE.as_i32() | GlContextFlags::DEBUG.as_i32()
    } else {
      GlContextFlags::FORWARD_COMPATIBLE.as_i32()
    };

    sdl.gl_set_attribute(GlAttr::MajorVersion, 3).unwrap();
    sdl.gl_set_attribute(GlAttr::MinorVersion, 3).unwrap();
    sdl
      .gl_set_attribute(GlAttr::Profile, GlProfile::Core as _)
      .unwrap();
    sdl.gl_set_attribute(GlAttr::Flags, FLAGS).unwrap();

    Self {
      sdl,
      gl_win: None,
      gl: None,
    }
  }
}

impl GraphicsProvider for SdlGraphicsProvider {
  fn create_window(&mut self, width: u32, height: u32) {
    println!("Creating window... {} {}", width, height);

    let gl_win = self
      .sdl
      .create_gl_window(
        zstr!("noentiendo"),
        None,
        (width.try_into().unwrap(), height.try_into().unwrap()),
        WindowFlags::ALLOW_HIGHDPI,
      )
      .unwrap();

    gl_win.set_swap_interval(1).unwrap();

    let gl = unsafe { GlFns::from_loader(&|zs| gl_win.get_proc_address(zs)).unwrap() };

    if gl_win.is_extension_supported(zstr!("GL_KHR_debug")) {
      println!("Activating the debug callback...");
      unsafe { gl.DebugMessageCallback(Some(println_gl_debug_callback), null()) };
    }
    gl.ClearColor(0.7, 0.6, 0.5, 1.0);

    self.gl_win = Some(gl_win);
    self.gl = Some(gl);
  }

  fn tick(&mut self) {
    while let Some(e) = self.sdl.poll_event() {
      match e {
        Event::Quit => panic!("Quit"),
        Event::MouseMotion { .. } => (),
        Event::Keyboard { .. } => (),
        Event::TextInput { text, .. } => {
          println!("TextInput: {:?}", str::from_utf8(&text));
        }
        other => println!("Event: {:?}", other),
      }
    }
    // now draw and swap

    self.gl.as_ref().unwrap().Clear(GL_COLOR_BUFFER_BIT);

    self.gl_win.as_ref().unwrap().swap_backbuffer();
  }

  fn set_pixel(&mut self, x: u32, y: u32, color: Color) {}
}
