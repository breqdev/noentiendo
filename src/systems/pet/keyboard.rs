use crate::keyboard::{KeyAdapter, KeyPosition, KeyState, KeySymbol};

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
pub struct PetKeyboardAdapter;

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

/// An adapter that maps keyboard symbols to keys on the PET's "Graphics" keyboard.
pub struct PetSymbolAdapter;

impl KeyAdapter<KeySymbol, PetKeys> for PetSymbolAdapter {
  fn map(state: &KeyState<KeySymbol>) -> KeyState<PetKeys> {
    let mut mapped = KeyState::new();

    for symbol in state.pressed() {
      use KeySymbol::*;

      mapped.press(match symbol {
        Char('!') => PetKeys::Exclamation,
        Char('"') => PetKeys::DoubleQuote,
        Char('#') => PetKeys::Hash,
        Char('$') => PetKeys::Dollar,
        Char('%') => PetKeys::Percent,
        Char('\'') => PetKeys::Apostrophe,
        Char('&') => PetKeys::Ampersand,
        Char('(') => PetKeys::LeftParen,
        Char(')') => PetKeys::RightParen,
        // TODO: Left Arrow
        Home => PetKeys::ClrHome,
        DownArrow => PetKeys::CursorUpDown,
        RightArrow => PetKeys::CursorLeftRight,
        Backspace => PetKeys::InsertDelete,

        Char('q') | Char('Q') => PetKeys::Q,
        Char('w') | Char('W') => PetKeys::W,
        Char('e') | Char('E') => PetKeys::E,
        Char('r') | Char('R') => PetKeys::R,
        Char('t') | Char('T') => PetKeys::T,
        Char('y') | Char('Y') => PetKeys::Y,
        Char('u') | Char('U') => PetKeys::U,
        Char('i') | Char('I') => PetKeys::I,
        Char('o') | Char('O') => PetKeys::O,
        Char('p') | Char('P') => PetKeys::P,
        Char('^') => PetKeys::UpArrow,

        Char('7') => PetKeys::Num7,
        Char('8') => PetKeys::Num8,
        Char('9') => PetKeys::Num9,
        Char('/') => PetKeys::NumDivide,

        Char('a') | Char('A') => PetKeys::A,
        Char('s') | Char('S') => PetKeys::S,
        Char('d') | Char('D') => PetKeys::D,
        Char('f') | Char('F') => PetKeys::F,
        Char('g') | Char('G') => PetKeys::G,
        Char('h') | Char('H') => PetKeys::H,
        Char('j') | Char('J') => PetKeys::J,
        Char('k') | Char('K') => PetKeys::K,
        Char('l') | Char('L') => PetKeys::L,
        Char(':') => PetKeys::Colon,
        Return => PetKeys::Return,

        Char('4') => PetKeys::Num4,
        Char('5') => PetKeys::Num5,
        Char('6') => PetKeys::Num6,
        Char('*') => PetKeys::NumMultiply,

        Char('z') | Char('Z') => PetKeys::Z,
        Char('x') | Char('X') => PetKeys::X,
        Char('c') | Char('C') => PetKeys::C,
        Char('v') | Char('V') => PetKeys::V,
        Char('b') | Char('B') => PetKeys::B,
        Char('n') | Char('N') => PetKeys::N,
        Char('m') | Char('M') => PetKeys::M,
        Char(',') => PetKeys::Comma,
        Char(';') => PetKeys::Semicolon,
        Char('?') => PetKeys::Question,

        Char('1') => PetKeys::Num1,
        Char('2') => PetKeys::Num2,
        Char('3') => PetKeys::Num3,
        Char('+') => PetKeys::NumPlus,

        LAlt => PetKeys::LShift, // Map Alt to Shift since "shift" actually does graphics characters.
        // TODO: Reverse
        Char('@') => PetKeys::At,
        Char('[') => PetKeys::LeftBracket,
        Char(']') => PetKeys::RightBracket,
        Char(' ') => PetKeys::Space,
        Char('<') => PetKeys::LessThan,
        Char('>') => PetKeys::GreaterThan,
        // TODO: Run Stop
        RAlt => PetKeys::RShift,

        Char('0') => PetKeys::Num0,
        Char('.') => PetKeys::NumPeriod,
        Char('-') => PetKeys::NumMinus,
        Char('=') => PetKeys::NumEquals,

        _ => continue,
      })
    }

    mapped
  }
}
