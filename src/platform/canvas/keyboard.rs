use crate::keyboard::{KeyAdapter, KeyPosition, KeyState};

pub struct JavaScriptAdapter {}

impl JavaScriptAdapter {
  pub fn new() -> Self {
    Self {}
  }
}

impl KeyAdapter<String, KeyPosition> for JavaScriptAdapter {
  fn map(state: &KeyState<String>) -> KeyState<KeyPosition> {
    let mut mapped = KeyState::new();

    for symbol in state.pressed() {
      use KeyPosition::*;

      // Source:
      // https://developer.mozilla.org/en-US/docs/Web/API/UI_Events/Keyboard_event_code_values#code_values_on_linux_x11
      mapped.press(match symbol.as_ref() {
        "Escape" => Escape,

        "Digit1" => Digit1,
        "Digit2" => Digit2,
        "Digit3" => Digit3,
        "Digit4" => Digit4,
        "Digit5" => Digit5,
        "Digit6" => Digit6,
        "Digit7" => Digit7,
        "Digit8" => Digit8,
        "Digit9" => Digit9,
        "Digit0" => Digit0,

        "Minus" => Minus,
        "Equal" => Equals,
        "Backspace" => Backspace,

        "Tab" => Tab,
        "KeyQ" => Q,
        "KeyW" => W,
        "KeyE" => E,
        "KeyR" => R,
        "KeyT" => T,
        "KeyY" => Y,
        "KeyU" => U,
        "KeyI" => I,
        "KeyO" => O,
        "KeyP" => P,
        "BracketLeft" => LeftBracket,
        "BracketRight" => RightBracket,
        "Enter" => Enter,

        "ControlLeft" => LControl,
        "KeyA" => A,
        "KeyS" => S,
        "KeyD" => D,
        "KeyF" => F,
        "KeyG" => G,
        "KeyH" => H,
        "KeyJ" => J,
        "KeyK" => K,
        "KeyL" => L,
        "Semicolon" => Semicolon,
        "Quote" => Apostrophe,
        "Backquote" => Grave,

        "ShiftLeft" => LShift,
        "Backslash" => Backslash,
        "KeyZ" => Z,
        "KeyX" => X,
        "KeyC" => C,
        "KeyV" => V,
        "KeyB" => B,
        "KeyN" => N,
        "KeyM" => M,
        "Comma" => Comma,
        "Period" => Period,
        "Slash" => Slash,
        "ShiftRight" => RShift,

        "NumpadMultiply" => NumMultiply,
        "AltLeft" => LAlt,
        "Space" => Space,
        "CapsLock" => CapsLock,

        "F1" => F1,
        "F2" => F2,
        "F3" => F3,
        "F4" => F4,
        "F5" => F5,
        "F6" => F6,
        "F7" => F7,
        "F8" => F8,
        "F9" => F9,
        "F10" => F10,

        "NumLock" => NumLock,
        "ScrollLock" => ScrollLock,
        "Numpad7" => Num7,
        "Numpad8" => Num8,
        "Numpad9" => Num9,
        "NumpadSubtract" => NumMinus,
        "Numpad4" => Num4,
        "Numpad5" => Num5,
        "Numpad6" => Num6,
        "NumpadAdd" => NumPlus,
        "Numpad1" => Num1,
        "Numpad2" => Num2,
        "Numpad3" => Num3,
        "Numpad0" => Num0,
        "NumpadDecimal" => NumPeriod,

        "F11" => F11,
        "F12" => F12,

        "NumpadEnter" => NumEnter,
        "ControlRight" => RControl,
        "NumpadDivide" => NumDivide,
        "PrintScreen" => PrintScreen,
        "AltRight" => RAlt,
        "Home" => Home,
        "ArrowUp" => UpArrow,
        "PageUp" => PageUp,
        "ArrowLeft" => LeftArrow,
        "ArrowRight" => RightArrow,
        "End" => End,
        "ArrowDown" => DownArrow,
        "PageDown" => PageDown,
        "Insert" => Insert,
        "Delete" => Delete,

        "Pause" => Pause,
        "ContextMenu" => Menu,

        "MetaLeft" => LSuper,
        "MetaRight" => RSuper,
        "OSLeft" => LSuper,
        "OSRight" => RSuper,

        _ => continue,
      });
    }

    mapped
  }
}
