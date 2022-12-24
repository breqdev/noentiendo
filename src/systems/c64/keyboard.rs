use crate::keyboard::commodore::C64Keys;

/// The keyboard matrix found on a Commodore 64.
/// Source: <https://www.c64-wiki.com/wiki/Keyboard>.
pub const KEYBOARD_MAPPING: [[C64Keys; 8]; 8] = {
  use C64Keys::*;

  [
    [
      InsertDelete,
      Return,
      CursorLeftRight,
      F7,
      F1,
      F3,
      F5,
      CursorUpDown,
    ],
    [Digit3, W, A, Digit4, Z, S, E, LShift],
    [Digit5, R, D, Digit6, C, F, T, X],
    [Digit7, Y, G, Digit8, B, H, U, V],
    [Digit9, I, J, Digit0, M, K, O, N],
    [Plus, P, L, Minus, Period, Colon, At, Comma],
    [
      Pound, Asterisk, Semicolon, ClrHome, RShift, Equals, UpArrow, Slash,
    ],
    [
      Digit1, LeftArrow, Control, Digit2, Space, Commodore, Q, RunStop,
    ],
  ]
};
