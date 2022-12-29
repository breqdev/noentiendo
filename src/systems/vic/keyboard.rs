use crate::keyboard::commodore::C64Keys;

/// The keyboard matrix in a VIC-20 system.
/// Source: <https://www.lemon64.com/forum/viewtopic.php?t=68210&sid=8b976b9f8699fc3588c5622b43a1f4b1>
pub const KEYBOARD_MAPPING: [[C64Keys; 8]; 8] = {
  use C64Keys::*;

  [
    [
      Digit1, LeftArrow, Control, RunStop, Space, Commodore, Q, Digit2,
    ],
    [Digit3, W, A, LShift, Z, S, E, Digit4],
    [Digit5, R, D, X, C, F, T, Digit6],
    [Digit7, Y, G, V, B, H, U, Digit8],
    [Digit9, I, J, N, M, K, O, Digit0],
    [Plus, P, L, Comma, Period, Colon, At, Minus],
    [
      Pound, Asterisk, Semicolon, Slash, RShift, Equals, UpArrow, ClrHome,
    ],
    [
      InsertDelete,
      Return,
      CursorLeftRight,
      CursorUpDown,
      F1,
      F3,
      F5,
      F7,
    ],
  ]
};
