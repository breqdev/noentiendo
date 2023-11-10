use crate::keyboard::apple::AppleKeys;

/// The keyboard matrix found on a Commodore 64.
/// Source: <https://www.c64-wiki.com/wiki/Keyboard>.
pub const KEYBOARD_MAPPING: [[AppleKeys; 10]; 8] = {
  use AppleKeys::*;

  [
    [
      Esc, Digit1, Digit2, Digit3, Digit4, Digit6, Digit5, Digit7, Digit8, Digit9,
    ],
    [Tab, Q, W, E, R, Y, T, U, I, O],
    [A, D, S, H, F, G, J, K, Semicolon, L],
    [Z, X, C, V, B, N, M, Comma, Period, Backslash],
    [
      NumDivide, NumLeft, Num0, Num1, Num2, Num3, Slash, Equals, Digit0, Minus,
    ],
    [
      NumRightParen,
      NumEsc,
      Num4,
      Num5,
      Num6,
      Num7,
      Grave,
      P,
      LeftBracket,
      RightBracket,
    ],
    [
      NumMultiply,
      NumRight,
      Digit8,
      Digit9,
      NumPeriod,
      NumPlus,
      Return,
      UpArrow,
      Space,
      Apostrophe,
    ],
    [
      NumPrint,
      NumSpace,
      NumLeftParen,
      NumMinus,
      NumReturn,
      NumComma,
      Del,
      DownArrow,
      LeftArrow,
      RightArrow,
    ],
  ]
};
