use winit::event::VirtualKeyCode;

use crate::keyboard::{KeyAdapter, KeyPosition, KeyState};

pub struct WinitAdapter;

impl KeyAdapter<VirtualKeyCode, KeyPosition> for WinitAdapter {
  fn map(state: &KeyState<VirtualKeyCode>) -> KeyState<KeyPosition> {
    let mut mapped = KeyState::new();

    for symbol in state.pressed() {
      use KeyPosition::*;

      mapped.press(match symbol {
        VirtualKeyCode::Key1 => Digit1,
        VirtualKeyCode::Key2 => Digit2,
        VirtualKeyCode::Key3 => Digit3,
        VirtualKeyCode::Key4 => Digit4,
        VirtualKeyCode::Key5 => Digit5,
        VirtualKeyCode::Key6 => Digit6,
        VirtualKeyCode::Key7 => Digit7,
        VirtualKeyCode::Key8 => Digit8,
        VirtualKeyCode::Key9 => Digit9,
        VirtualKeyCode::Key0 => Digit0,

        VirtualKeyCode::A => A,
        VirtualKeyCode::B => B,
        VirtualKeyCode::C => C,
        VirtualKeyCode::D => D,
        VirtualKeyCode::E => E,
        VirtualKeyCode::F => F,
        VirtualKeyCode::G => G,
        VirtualKeyCode::H => H,
        VirtualKeyCode::I => I,
        VirtualKeyCode::J => J,
        VirtualKeyCode::K => K,
        VirtualKeyCode::L => L,
        VirtualKeyCode::M => M,
        VirtualKeyCode::N => N,
        VirtualKeyCode::O => O,
        VirtualKeyCode::P => P,
        VirtualKeyCode::Q => Q,
        VirtualKeyCode::R => R,
        VirtualKeyCode::S => S,
        VirtualKeyCode::T => T,
        VirtualKeyCode::U => U,
        VirtualKeyCode::V => V,
        VirtualKeyCode::W => W,
        VirtualKeyCode::X => X,
        VirtualKeyCode::Y => Y,
        VirtualKeyCode::Z => Z,

        VirtualKeyCode::Escape => Escape,
        VirtualKeyCode::F1 => F1,
        VirtualKeyCode::F2 => F2,
        VirtualKeyCode::F3 => F3,
        VirtualKeyCode::F4 => F4,
        VirtualKeyCode::F5 => F5,
        VirtualKeyCode::F6 => F6,
        VirtualKeyCode::F7 => F7,
        VirtualKeyCode::F8 => F8,
        VirtualKeyCode::F9 => F9,
        VirtualKeyCode::F10 => F10,
        VirtualKeyCode::F11 => F11,
        VirtualKeyCode::F12 => F12,

        VirtualKeyCode::Snapshot => PrintScreen,
        VirtualKeyCode::Scroll => ScrollLock,
        VirtualKeyCode::Pause => Pause,
        VirtualKeyCode::Insert => Insert,
        VirtualKeyCode::Home => Home,
        VirtualKeyCode::Delete => Delete,
        VirtualKeyCode::End => End,
        VirtualKeyCode::PageDown => PageDown,
        VirtualKeyCode::PageUp => PageUp,

        VirtualKeyCode::Left => LeftArrow,
        VirtualKeyCode::Up => UpArrow,
        VirtualKeyCode::Right => RightArrow,
        VirtualKeyCode::Down => DownArrow,

        VirtualKeyCode::Back => Backspace,
        VirtualKeyCode::Return => Enter,
        VirtualKeyCode::Space => Space,

        VirtualKeyCode::Numlock => NumLock,
        VirtualKeyCode::Numpad0 => Num0,
        VirtualKeyCode::Numpad1 => Num1,
        VirtualKeyCode::Numpad2 => Num2,
        VirtualKeyCode::Numpad3 => Num3,
        VirtualKeyCode::Numpad4 => Num4,
        VirtualKeyCode::Numpad5 => Num5,
        VirtualKeyCode::Numpad6 => Num6,
        VirtualKeyCode::Numpad7 => Num7,
        VirtualKeyCode::Numpad8 => Num8,
        VirtualKeyCode::Numpad9 => Num9,

        VirtualKeyCode::NumpadAdd => NumPlus,
        VirtualKeyCode::NumpadDivide => NumDivide,
        VirtualKeyCode::NumpadMultiply => NumMultiply,
        VirtualKeyCode::NumpadSubtract => NumMinus,
        VirtualKeyCode::NumpadDecimal => NumPeriod,
        VirtualKeyCode::NumpadEnter => NumEnter,

        VirtualKeyCode::Apostrophe => Apostrophe,
        VirtualKeyCode::Backslash => Backslash,
        VirtualKeyCode::Comma => Comma,
        VirtualKeyCode::Equals => Equals,
        VirtualKeyCode::Grave => Grave,
        VirtualKeyCode::LBracket => LeftBracket,
        VirtualKeyCode::Minus => Minus,
        VirtualKeyCode::Period => Period,
        VirtualKeyCode::RBracket => RightBracket,
        VirtualKeyCode::Semicolon => Semicolon,
        VirtualKeyCode::Slash => Slash,
        VirtualKeyCode::Tab => Tab,

        VirtualKeyCode::LAlt => LAlt,
        VirtualKeyCode::RAlt => RAlt,
        VirtualKeyCode::LControl => LControl,
        VirtualKeyCode::RControl => RControl,
        VirtualKeyCode::LShift => LShift,
        VirtualKeyCode::RShift => RShift,
        VirtualKeyCode::LWin => LSuper,
        VirtualKeyCode::RWin => RSuper,

        _ => continue,
      })
    }

    mapped
  }
}
