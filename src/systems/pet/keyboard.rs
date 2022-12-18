use crate::keyboard::{KeyAdapter, KeyPosition, KeyState};

/// The keys found on the PET's "Graphics" keyboard.
/// https://commons.wikimedia.org/wiki/File:PET_Keyboard.svg
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PetKeys {
  Exclamation,
  DoubleQuote,
  Hash,
  Dollar,
  Percent,
  Apostrophe,
  Ampersand,
  Backslash,
  LeftParen,
  RightParen,
  LeftArrow, // doesn't move cursor, literally types "<-"

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
  UpArrow, // literally types "^"

  A,
  S,
  D,
  F,
  G,
  H,
  J,
  K,
  L,
  Colon,
  Return,

  Z,
  X,
  C,
  V,
  B,
  N,
  M,
  Comma,
  Semicolon,
  Question,

  LShift,
  Reverse,
  At,
  LeftBracket,
  RightBracket,
  Space,
  LessThan,
  GreaterThan,
  RunStop,
  RShift,

  ClrHome,
  CursorUpDown,
  CursorLeftRight,
  InsertDelete,
  Num7,
  Num8,
  Num9,
  NumDivide,
  Num4,
  Num5,
  Num6,
  NumMultiply,
  Num1,
  Num2,
  Num3,
  NumPlus,
  Num0,
  NumPeriod,
  NumMinus,
  NumEquals,

  Unused,
}

/// The keyboard matrix for the PET's "Graphics" keyboard.
/// Source: https://www.lemon64.com/forum/viewtopic.php?t=68210&sid=8b976b9f8699fc3588c5622b43a1f4b1
pub const KEYBOARD_MAPPING: [[PetKeys; 8]; 10] = {
  use PetKeys::*;

  [
    [
      Exclamation,
      Hash,
      Percent,
      Ampersand,
      LeftParen,
      LeftArrow,
      ClrHome,
      CursorLeftRight,
    ],
    [
      DoubleQuote,
      Dollar,
      Apostrophe,
      Backslash,
      RightParen,
      Unused,
      CursorUpDown,
      InsertDelete,
    ],
    [Q, E, T, U, O, UpArrow, Num7, Num9],
    [W, R, Y, I, P, Unused, Num8, NumDivide],
    [A, D, G, J, L, Unused, Num4, Num6],
    [S, F, H, K, Colon, Unused, Num5, NumMultiply],
    [Z, C, B, M, Semicolon, Return, Num1, Num3],
    [X, V, N, Comma, Question, Unused, Num2, NumPlus],
    [
      LShift,
      At,
      RightBracket,
      Unused,
      GreaterThan,
      RShift,
      Num0,
      NumMinus,
    ],
    [
      Reverse,
      LeftBracket,
      Space,
      LessThan,
      RunStop,
      Unused,
      NumPeriod,
      NumEquals,
    ],
  ]
};

/// An adapter that maps standard keyboard positions to keys on the PET's "Graphics" keyboard.
pub struct PetKeyboardAdapter {}

impl KeyAdapter<KeyPosition, PetKeys> for PetKeyboardAdapter {
  fn map(state: &KeyState<KeyPosition>) -> KeyState<PetKeys> {
    let mut mapped = KeyState::new();

    for symbol in state.pressed() {
      use KeyPosition::*;

      mapped.press(match symbol {
        Digit0 => PetKeys::Num0,
        Digit1 => PetKeys::Num1,
        Digit2 => PetKeys::Num2,
        Digit3 => PetKeys::Num3,
        Digit4 => PetKeys::Num4,
        Digit5 => PetKeys::Num5,
        Digit6 => PetKeys::Num6,
        Digit7 => PetKeys::Num7,
        Digit8 => PetKeys::Num8,
        Digit9 => PetKeys::Num9,

        A => PetKeys::A,
        B => PetKeys::B,
        C => PetKeys::C,
        D => PetKeys::D,
        E => PetKeys::E,
        F => PetKeys::F,
        G => PetKeys::G,
        H => PetKeys::H,
        I => PetKeys::I,
        J => PetKeys::J,
        K => PetKeys::K,
        L => PetKeys::L,
        M => PetKeys::M,
        N => PetKeys::N,
        O => PetKeys::O,
        P => PetKeys::P,
        Q => PetKeys::Q,
        R => PetKeys::R,
        S => PetKeys::S,
        T => PetKeys::T,
        U => PetKeys::U,
        V => PetKeys::V,
        W => PetKeys::W,
        X => PetKeys::X,
        Y => PetKeys::Y,
        Z => PetKeys::Z,

        Minus => PetKeys::NumMinus,
        Equals => PetKeys::NumEquals,
        LeftBracket => PetKeys::LeftBracket,
        RightBracket => PetKeys::RightBracket,
        Backslash => PetKeys::Backslash,
        Semicolon => PetKeys::Semicolon,
        Apostrophe => PetKeys::Apostrophe,
        Enter => PetKeys::Return,

        LShift => PetKeys::LShift,
        RShift => PetKeys::RShift,
        Comma => PetKeys::Comma,
        Period => PetKeys::NumPeriod,
        Slash => PetKeys::NumDivide,

        Space => PetKeys::Space,
        Backspace => PetKeys::InsertDelete,

        _ => continue,
      })
    }

    mapped
  }
}
