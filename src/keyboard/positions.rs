/// A representation for a position on a modern keyboard.
/// Source: https://en.wikipedia.org/wiki/Keyboard_layout#/media/File:Qwerty.svg
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyPosition {
  Escape,

  F1,
  F2,
  F3,
  F4,
  F5,
  F6,
  F7,
  F8,
  F9,
  F10,
  F11,
  F12,

  PrintScreen,
  ScrollLock,
  Pause,

  Grave,
  Digit1,
  Digit2,
  Digit3,
  Digit4,
  Digit5,
  Digit6,
  Digit7,
  Digit8,
  Digit9,
  Digit0,
  Minus,
  Equals,
  Backspace,

  Insert,
  Home,
  PageUp,

  NumLock,
  NumDivide,
  NumMultiply,
  NumMinus,

  Tab,
  Q,
  W,
  E,
  R,
  T,
  Y,
  U,
  I,
  O,
  P,
  LeftBracket,
  RightBracket,
  Backslash,

  Delete,
  End,
  PageDown,

  Num7,
  Num8,
  Num9,
  NumPlus,

  CapsLock,
  A,
  S,
  D,
  F,
  G,
  H,
  J,
  K,
  L,
  Semicolon,
  Apostrophe,
  Enter,

  Num4,
  Num5,
  Num6,

  LShift,
  Z,
  X,
  C,
  V,
  B,
  N,
  M,
  Comma,
  Period,
  Slash,
  RShift,

  UpArrow,

  Num1,
  Num2,
  Num3,
  NumEnter,

  LControl,
  LSuper,
  LAlt,
  Space,
  RAlt,
  RSuper,
  Menu,
  RControl,

  LeftArrow,
  DownArrow,
  RightArrow,

  Num0,
  NumPeriod,
}
