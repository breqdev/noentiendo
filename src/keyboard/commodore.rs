use serde::{Deserialize, Serialize};

use crate::keyboard::{KeyAdapter, KeyPosition, KeyState, KeySymbol};

use super::VirtualKey;

/// Keys found on a VIC-20 or Commodore 64 keyboard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum C64Keys {
  LeftArrow,
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
  Plus,
  Minus,
  Pound,
  ClrHome,
  InsertDelete,

  Control,
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
  At,
  Asterisk,
  UpArrow,

  RunStop,
  ShiftLock,
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
  Semicolon,
  Equals,
  Return,

  Commodore,
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
  CursorUpDown,
  CursorLeftRight,

  Space,

  F1,
  F3,
  F5,
  F7,
}

/// An adapter for mapping positions on a standard keyboard to keys on a VIC-20 or Commodore 64.
pub struct C64KeyboardAdapter;

impl KeyAdapter<KeyPosition, C64Keys> for C64KeyboardAdapter {
  fn map(state: &KeyState<KeyPosition>) -> KeyState<C64Keys> {
    let mut mapped = KeyState::new();

    for symbol in state.pressed() {
      use KeyPosition::*;

      mapped.press(match symbol {
        Grave => C64Keys::LeftArrow,
        Digit1 => C64Keys::Digit1,
        Digit2 => C64Keys::Digit2,
        Digit3 => C64Keys::Digit3,
        Digit4 => C64Keys::Digit4,
        Digit5 => C64Keys::Digit5,
        Digit6 => C64Keys::Digit6,
        Digit7 => C64Keys::Digit7,
        Digit8 => C64Keys::Digit8,
        Digit9 => C64Keys::Digit9,
        Digit0 => C64Keys::Digit0,
        // TODO: Plus
        Minus => C64Keys::Minus,
        // TODO: Pound
        // TODO: ClrHome
        Backspace => C64Keys::InsertDelete,

        LControl => C64Keys::Control,
        Q => C64Keys::Q,
        W => C64Keys::W,
        E => C64Keys::E,
        R => C64Keys::R,
        T => C64Keys::T,
        Y => C64Keys::Y,
        U => C64Keys::U,
        I => C64Keys::I,
        O => C64Keys::O,
        P => C64Keys::P,
        // TODO: At
        // TODO: Asterisk
        // TODO: UpArrow
        // TODO: Restore

        // TODO: RunStop
        CapsLock => C64Keys::ShiftLock,
        A => C64Keys::A,
        S => C64Keys::S,
        D => C64Keys::D,
        F => C64Keys::F,
        G => C64Keys::G,
        H => C64Keys::H,
        J => C64Keys::J,
        K => C64Keys::K,
        L => C64Keys::L,
        // TODO: Colon
        Semicolon => C64Keys::Semicolon,
        Equals => C64Keys::Equals,
        Enter => C64Keys::Return,

        LSuper | RSuper => C64Keys::Commodore,
        LShift => C64Keys::LShift,
        Z => C64Keys::Z,
        X => C64Keys::X,
        C => C64Keys::C,
        V => C64Keys::V,
        B => C64Keys::B,
        N => C64Keys::N,
        M => C64Keys::M,
        Comma => C64Keys::Comma,
        Period => C64Keys::Period,
        Slash => C64Keys::Slash,
        RShift => C64Keys::RShift,
        // TODO: CursorUpDown
        // TODO: CursorLeftRight
        Space => C64Keys::Space,

        F1 => C64Keys::F1,
        F3 => C64Keys::F3,
        F5 => C64Keys::F5,
        F7 => C64Keys::F7,

        _ => continue,
      });
    }

    mapped
  }
}

/// An adapter for mapping symbols pressed on a standard keyboard to keys pressed on a Commodore 64.
pub struct C64SymbolAdapter;

impl KeyAdapter<KeySymbol, C64Keys> for C64SymbolAdapter {
  fn map(state: &KeyState<KeySymbol>) -> KeyState<C64Keys> {
    let mut mapped = KeyState::new();

    for symbol in state.pressed() {
      use KeySymbol::*;

      mapped.press(match symbol {
        // TODO: Left Arrow
        Char('1') => C64Keys::Digit1,
        Char('2') => C64Keys::Digit2,
        Char('3') => C64Keys::Digit3,
        Char('4') => C64Keys::Digit4,
        Char('5') => C64Keys::Digit5,
        Char('6') => C64Keys::Digit6,
        Char('7') => C64Keys::Digit7,
        Char('8') => C64Keys::Digit8,
        Char('9') => C64Keys::Digit9,
        Char('0') => C64Keys::Digit0,
        Char('+') => C64Keys::Plus,
        Char('-') => C64Keys::Minus,
        Char('£') => C64Keys::Pound,
        // TODO: ClrHome
        Backspace => C64Keys::InsertDelete,

        LControl | RControl => C64Keys::Control,
        Char('q') | Char('Q') => C64Keys::Q,
        Char('w') | Char('W') => C64Keys::W,
        Char('e') | Char('E') => C64Keys::E,
        Char('r') | Char('R') => C64Keys::R,
        Char('t') | Char('T') => C64Keys::T,
        Char('y') | Char('Y') => C64Keys::Y,
        Char('u') | Char('U') => C64Keys::U,
        Char('i') | Char('I') => C64Keys::I,
        Char('o') | Char('O') => C64Keys::O,
        Char('p') | Char('P') => C64Keys::P,
        Char('@') => C64Keys::At,
        Char('*') => C64Keys::Asterisk,
        // TODO: UpArrow
        // TODO: Restore
        Interrupt => C64Keys::RunStop,
        CapsLock => C64Keys::ShiftLock,
        Char('a') | Char('A') => C64Keys::A,
        Char('s') | Char('S') => C64Keys::S,
        Char('d') | Char('D') => C64Keys::D,
        Char('f') | Char('F') => C64Keys::F,
        Char('g') | Char('G') => C64Keys::G,
        Char('h') | Char('H') => C64Keys::H,
        Char('j') | Char('J') => C64Keys::J,
        Char('k') | Char('K') => C64Keys::K,
        Char('l') | Char('L') => C64Keys::L,
        Char(':') => C64Keys::Colon,
        Char(';') => C64Keys::Semicolon,
        Char('=') => C64Keys::Equals,
        Return => C64Keys::Return,

        LSuper | RSuper => C64Keys::Commodore,
        LShift => continue, // Handled separately
        Char('z') | Char('Z') => C64Keys::Z,
        Char('x') | Char('X') => C64Keys::X,
        Char('c') | Char('C') => C64Keys::C,
        Char('v') | Char('V') => C64Keys::V,
        Char('b') | Char('B') => C64Keys::B,
        Char('n') | Char('N') => C64Keys::N,
        Char('m') | Char('M') => C64Keys::M,
        Char(',') => C64Keys::Comma,
        Char('.') => C64Keys::Period,
        Char('/') => C64Keys::Slash,
        RShift => continue, // Handled separately
        DownArrow => C64Keys::CursorUpDown,
        RightArrow => C64Keys::CursorLeftRight,
        Char(' ') => C64Keys::Space,

        F1 => C64Keys::F1,
        F3 => C64Keys::F3,
        F5 => C64Keys::F5,
        F7 => C64Keys::F7,

        _ => continue,
      })
    }

    if mapped.pressed().is_empty() {
      // If no non-shifted keys were pressed, check for shifted keys.
      for symbol in state.pressed() {
        use KeySymbol::*;

        mapped.press(match symbol {
          Char('!') => C64Keys::Digit1,
          Char('"') => C64Keys::Digit2,
          Char('#') => C64Keys::Digit3,
          Char('$') => C64Keys::Digit4,
          Char('%') => C64Keys::Digit5,
          Char('&') => C64Keys::Digit6,
          Char('\'') => C64Keys::Digit7,
          Char('(') => C64Keys::Digit8,
          Char(')') => C64Keys::Digit9,

          Char('[') => C64Keys::Colon,
          Char(']') => C64Keys::Semicolon,

          Char('<') => C64Keys::Comma,
          Char('>') => C64Keys::Period,
          Char('?') => C64Keys::Slash,

          UpArrow => C64Keys::CursorUpDown,
          LeftArrow => C64Keys::CursorLeftRight,

          _ => continue,
        })
      }

      // If we added keys, make sure shift is pressed
      if !mapped.pressed().is_empty() {
        mapped.press(C64Keys::LShift);
      }
    }

    mapped
  }
}

pub struct C64VirtualAdapter;

impl KeyAdapter<VirtualKey, C64Keys> for C64VirtualAdapter {
  fn map(state: &KeyState<VirtualKey>) -> KeyState<C64Keys> {
    let mut mapped = KeyState::new();

    for symbol in state.pressed() {
      if let VirtualKey::Commodore(symbol) = symbol {
        mapped.press(symbol.clone());
      }
    }

    mapped
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_positional() {
    let mut positions = KeyState::<KeyPosition>::new();

    positions.press(KeyPosition::Q);
    positions.press(KeyPosition::Digit2);
    positions.press(KeyPosition::Comma);
    positions.press(KeyPosition::Enter);

    let mapped = C64KeyboardAdapter::map(&positions);

    assert_eq!(
      &vec![C64Keys::Q, C64Keys::Digit2, C64Keys::Comma, C64Keys::Return],
      mapped.pressed()
    );

    positions.press(KeyPosition::LShift);
    positions.release(KeyPosition::Enter);

    let mapped = C64KeyboardAdapter::map(&positions);

    assert_eq!(
      &vec![C64Keys::Q, C64Keys::Digit2, C64Keys::Comma, C64Keys::LShift],
      mapped.pressed()
    );
  }

  #[test]
  fn test_symbols_unshifted() {
    let mut symbols = KeyState::<KeySymbol>::new();

    symbols.press(KeySymbol::Char('q'));
    symbols.press(KeySymbol::Char('2'));
    symbols.press(KeySymbol::Char(','));
    symbols.press(KeySymbol::Return);

    let mapped = C64SymbolAdapter::map(&symbols);

    assert_eq!(
      &vec![C64Keys::Q, C64Keys::Digit2, C64Keys::Comma, C64Keys::Return],
      mapped.pressed()
    );
  }

  #[test]
  fn test_symbols_shifted() {
    let mut symbols = KeyState::<KeySymbol>::new();

    symbols.press(KeySymbol::Char('!'));
    symbols.press(KeySymbol::Char('<'));
    symbols.press(KeySymbol::Char('>'));

    let mapped = C64SymbolAdapter::map(&symbols);

    assert_eq!(
      &vec![
        C64Keys::Digit1,
        C64Keys::Comma,
        C64Keys::Period,
        C64Keys::LShift
      ],
      mapped.pressed()
    );
  }

  #[test]
  fn test_mixed_shifted() {
    let mut symbols = KeyState::<KeySymbol>::new();

    symbols.press(KeySymbol::Char('1'));
    symbols.press(KeySymbol::Char('%'));
    symbols.press(KeySymbol::Char('"'));
    symbols.press(KeySymbol::Return);

    let mapped = C64SymbolAdapter::map(&symbols);

    // Do a "best effort" mapping, dropping the shifted keys
    assert_eq!(&vec![C64Keys::Digit1, C64Keys::Return], mapped.pressed());
  }

  #[test]
  fn test_cursor_keys() {
    let mut state = KeyState::<KeySymbol>::new();

    state.press(KeySymbol::DownArrow);
    assert_eq!(
      &vec![C64Keys::CursorUpDown],
      C64SymbolAdapter::map(&state).pressed()
    );

    state.release(KeySymbol::DownArrow);
    state.press(KeySymbol::UpArrow);
    assert_eq!(
      &vec![C64Keys::CursorUpDown, C64Keys::LShift],
      C64SymbolAdapter::map(&state).pressed()
    );

    state.press(KeySymbol::LeftArrow);
    assert_eq!(
      &vec![
        C64Keys::CursorUpDown,
        C64Keys::CursorLeftRight,
        C64Keys::LShift
      ],
      C64SymbolAdapter::map(&state).pressed()
    );

    // map the right arrow, but give up on the rest
    state.press(KeySymbol::RightArrow);
    assert_eq!(
      &vec![C64Keys::CursorLeftRight],
      C64SymbolAdapter::map(&state).pressed()
    );
  }
}
