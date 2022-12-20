use pixels::raw_window_handle::{
  HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle, RawWindowHandle, WebDisplayHandle,
  WebWindowHandle,
};
use web_sys::HtmlCanvasElement;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CanvasWindow {
  id: u32,
}

impl CanvasWindow {
  pub fn new(canvas: &HtmlCanvasElement) -> Self {
    // Get the ID from the canvas's `data-raw-handle` attribute.
    let id = canvas
      .attributes()
      .get_named_item("data-raw-handle")
      .expect("Canvas has no `data-raw-handle` attribute")
      .value()
      .parse::<u32>()
      .expect("Canvas `data-raw-handle` attribute is not a number");

    Self { id }
  }
}

unsafe impl HasRawWindowHandle for CanvasWindow {
  fn raw_window_handle(&self) -> RawWindowHandle {
    let mut handle = WebWindowHandle::empty();
    handle.id = self.id;

    RawWindowHandle::Web(handle)
  }
}

unsafe impl HasRawDisplayHandle for CanvasWindow {
  fn raw_display_handle(&self) -> RawDisplayHandle {
    let handle = WebDisplayHandle::empty();

    RawDisplayHandle::Web(handle)
  }
}
