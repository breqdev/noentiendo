use crate::keyboard::{KeyAdapter, KeyPosition, KeyState};

/// A representation for a symbol on a modern keyboard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeySymbol {
  /// Alphanumeric or symbol keys
  Char(char),

  // ASCII control characters
  Return,
  Backspace,
  Delete,
  Escape,
  PrintScreen,
  ScrollLock,
  Pause,
  Insert,
  Home,
  PageUp,
  PageDown,
  End,
  NumLock,
  CapsLock,

  // Modifier keys
  LShift,
  RShift,
  LSuper,
  RSuper,
  LAlt,
  RAlt,
  LControl,
  RControl,
  Menu,

  // Arrow keys
  LeftArrow,
  RightArrow,
  UpArrow,
  DownArrow,

  // Function keys
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
}

/// An adapter that maps physical key positions to symbols.
pub struct SymbolAdapter;

impl KeyAdapter<KeyPosition, KeySymbol> for SymbolAdapter {
  fn map(state: &KeyState<KeyPosition>) -> KeyState<KeySymbol> {
    use KeyPosition::*;

    let mut symbols = KeyState::new();

    if !state.is_pressed(KeyPosition::LShift) && !state.is_pressed(KeyPosition::RShift) {
      for position in state.pressed() {
        symbols.press(match position {
          Escape => KeySymbol::Escape,
          F1 => KeySymbol::F1,
          F2 => KeySymbol::F2,
          F3 => KeySymbol::F3,
          F4 => KeySymbol::F4,
          F5 => KeySymbol::F5,
          F6 => KeySymbol::F6,
          F7 => KeySymbol::F7,
          F8 => KeySymbol::F8,
          F9 => KeySymbol::F9,
          F10 => KeySymbol::F10,
          F11 => KeySymbol::F11,
          F12 => KeySymbol::F12,

          PrintScreen => KeySymbol::PrintScreen,
          ScrollLock => KeySymbol::ScrollLock,
          Pause => KeySymbol::Pause,

          Grave => KeySymbol::Char('`'),
          Digit1 => KeySymbol::Char('1'),
          Digit2 => KeySymbol::Char('2'),
          Digit3 => KeySymbol::Char('3'),
          Digit4 => KeySymbol::Char('4'),
          Digit5 => KeySymbol::Char('5'),
          Digit6 => KeySymbol::Char('6'),
          Digit7 => KeySymbol::Char('7'),
          Digit8 => KeySymbol::Char('8'),
          Digit9 => KeySymbol::Char('9'),
          Digit0 => KeySymbol::Char('0'),
          Minus => KeySymbol::Char('-'),
          Equals => KeySymbol::Char('='),
          Backspace => KeySymbol::Backspace,

          Insert => KeySymbol::Insert,
          Home => KeySymbol::Home,
          PageUp => KeySymbol::PageUp,

          NumLock => KeySymbol::NumLock,
          NumDivide => KeySymbol::Char('/'),
          NumMultiply => KeySymbol::Char('*'),
          NumMinus => KeySymbol::Char('-'),

          Tab => KeySymbol::Char('\t'),
          Q => KeySymbol::Char('q'),
          W => KeySymbol::Char('w'),
          E => KeySymbol::Char('e'),
          R => KeySymbol::Char('r'),
          T => KeySymbol::Char('t'),
          Y => KeySymbol::Char('y'),
          U => KeySymbol::Char('u'),
          I => KeySymbol::Char('i'),
          O => KeySymbol::Char('o'),
          P => KeySymbol::Char('p'),
          LeftBracket => KeySymbol::Char('['),
          RightBracket => KeySymbol::Char(']'),
          Backslash => KeySymbol::Char('\\'),

          Delete => KeySymbol::Delete,
          End => KeySymbol::End,
          PageDown => KeySymbol::PageDown,

          Num7 => KeySymbol::Char('7'),
          Num8 => KeySymbol::Char('8'),
          Num9 => KeySymbol::Char('9'),
          NumPlus => KeySymbol::Char('+'),

          CapsLock => KeySymbol::CapsLock,
          A => KeySymbol::Char('a'),
          S => KeySymbol::Char('s'),
          D => KeySymbol::Char('d'),
          F => KeySymbol::Char('f'),
          G => KeySymbol::Char('g'),
          H => KeySymbol::Char('h'),
          J => KeySymbol::Char('j'),
          K => KeySymbol::Char('k'),
          L => KeySymbol::Char('l'),
          Semicolon => KeySymbol::Char(';'),
          Apostrophe => KeySymbol::Char('\''),
          Enter => KeySymbol::Return,

          Num4 => KeySymbol::Char('4'),
          Num5 => KeySymbol::Char('5'),
          Num6 => KeySymbol::Char('6'),

          LShift => continue, // We rewrite the shift keys separately
          Z => KeySymbol::Char('z'),
          X => KeySymbol::Char('x'),
          C => KeySymbol::Char('c'),
          V => KeySymbol::Char('v'),
          B => KeySymbol::Char('b'),
          N => KeySymbol::Char('n'),
          M => KeySymbol::Char('m'),
          Comma => KeySymbol::Char(','),
          Period => KeySymbol::Char('.'),
          Slash => KeySymbol::Char('/'),
          RShift => continue,

          Num1 => KeySymbol::Char('1'),
          Num2 => KeySymbol::Char('2'),
          Num3 => KeySymbol::Char('3'),
          NumEnter => KeySymbol::Return,

          LControl => KeySymbol::LControl,
          LSuper => KeySymbol::LSuper,
          LAlt => KeySymbol::LAlt,
          Space => KeySymbol::Char(' '),
          RAlt => KeySymbol::RAlt,
          RSuper => KeySymbol::RSuper,
          Menu => KeySymbol::Menu,
          RControl => KeySymbol::RControl,

          UpArrow => KeySymbol::UpArrow,
          LeftArrow => KeySymbol::LeftArrow,
          DownArrow => KeySymbol::DownArrow,
          RightArrow => KeySymbol::RightArrow,

          Num0 => KeySymbol::Char('0'),
          NumPeriod => KeySymbol::Char('.'),
        })
      }
    } else {
      for position in state.pressed() {
        symbols.press(match position {
          Escape => KeySymbol::Escape,
          F1 => KeySymbol::F1,
          F2 => KeySymbol::F2,
          F3 => KeySymbol::F3,
          F4 => KeySymbol::F4,
          F5 => KeySymbol::F5,
          F6 => KeySymbol::F6,
          F7 => KeySymbol::F7,
          F8 => KeySymbol::F8,
          F9 => KeySymbol::F9,
          F10 => KeySymbol::F10,
          F11 => KeySymbol::F11,
          F12 => KeySymbol::F12,

          PrintScreen => KeySymbol::PrintScreen,
          ScrollLock => KeySymbol::ScrollLock,
          Pause => KeySymbol::Pause,

          Grave => KeySymbol::Char('~'),
          Digit1 => KeySymbol::Char('!'),
          Digit2 => KeySymbol::Char('@'),
          Digit3 => KeySymbol::Char('#'),
          Digit4 => KeySymbol::Char('$'),
          Digit5 => KeySymbol::Char('%'),
          Digit6 => KeySymbol::Char('^'),
          Digit7 => KeySymbol::Char('&'),
          Digit8 => KeySymbol::Char('*'),
          Digit9 => KeySymbol::Char('('),
          Digit0 => KeySymbol::Char(')'),
          Minus => KeySymbol::Char('_'),
          Equals => KeySymbol::Char('+'),
          Backspace => KeySymbol::Backspace,

          Insert => KeySymbol::Insert,
          Home => KeySymbol::Home,
          PageUp => KeySymbol::PageUp,

          NumLock => KeySymbol::NumLock,
          NumDivide => KeySymbol::Char('/'),
          NumMultiply => KeySymbol::Char('*'),
          NumMinus => KeySymbol::Char('-'),

          Tab => KeySymbol::Char('\t'),
          Q => KeySymbol::Char('Q'),
          W => KeySymbol::Char('W'),
          E => KeySymbol::Char('E'),
          R => KeySymbol::Char('R'),
          T => KeySymbol::Char('T'),
          Y => KeySymbol::Char('Y'),
          U => KeySymbol::Char('U'),
          I => KeySymbol::Char('I'),
          O => KeySymbol::Char('O'),
          P => KeySymbol::Char('P'),
          LeftBracket => KeySymbol::Char('{'),
          RightBracket => KeySymbol::Char('}'),
          Backslash => KeySymbol::Char('|'),

          Delete => KeySymbol::Delete,
          End => KeySymbol::End,
          PageDown => KeySymbol::PageDown,

          Num7 => KeySymbol::Char('7'),
          Num8 => KeySymbol::Char('8'),
          Num9 => KeySymbol::Char('9'),
          NumPlus => KeySymbol::Char('+'),

          CapsLock => KeySymbol::CapsLock,
          A => KeySymbol::Char('A'),
          S => KeySymbol::Char('S'),
          D => KeySymbol::Char('D'),
          F => KeySymbol::Char('F'),
          G => KeySymbol::Char('G'),
          H => KeySymbol::Char('H'),
          J => KeySymbol::Char('J'),
          K => KeySymbol::Char('K'),
          L => KeySymbol::Char('L'),
          Semicolon => KeySymbol::Char(':'),
          Apostrophe => KeySymbol::Char('"'),
          Enter => KeySymbol::Return,

          Num4 => KeySymbol::Char('4'),
          Num5 => KeySymbol::Char('5'),
          Num6 => KeySymbol::Char('6'),

          LShift => continue, // We rewrite the shift keys separately
          Z => KeySymbol::Char('Z'),
          X => KeySymbol::Char('X'),
          C => KeySymbol::Char('C'),
          V => KeySymbol::Char('V'),
          B => KeySymbol::Char('B'),
          N => KeySymbol::Char('N'),
          M => KeySymbol::Char('M'),
          Comma => KeySymbol::Char('<'),
          Period => KeySymbol::Char('>'),
          Slash => KeySymbol::Char('?'),
          RShift => continue,

          Num1 => KeySymbol::Char('1'),
          Num2 => KeySymbol::Char('2'),
          Num3 => KeySymbol::Char('3'),
          NumEnter => KeySymbol::Return,

          LControl => KeySymbol::LControl,
          LSuper => KeySymbol::LSuper,
          LAlt => KeySymbol::LAlt,
          Space => KeySymbol::Char(' '),
          RAlt => KeySymbol::RAlt,
          RSuper => KeySymbol::RSuper,
          Menu => KeySymbol::Menu,
          RControl => KeySymbol::RControl,

          UpArrow => KeySymbol::UpArrow,
          LeftArrow => KeySymbol::LeftArrow,
          DownArrow => KeySymbol::DownArrow,
          RightArrow => KeySymbol::RightArrow,

          Num0 => KeySymbol::Char('0'),
          NumPeriod => KeySymbol::Char('.'),
        })
      }
    }

    symbols
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_letter_mapping() {
    let mut positions = KeyState::<KeyPosition>::new();
    positions.press(KeyPosition::B);
    positions.press(KeyPosition::M);
    positions.press(KeyPosition::C);

    let symbols = SymbolAdapter::map(&positions);
    assert_eq!(
      &vec![
        KeySymbol::Char('b'),
        KeySymbol::Char('m'),
        KeySymbol::Char('c'),
      ],
      symbols.pressed()
    );
  }

  #[test]
  fn test_capital_letters() {
    let mut positions = KeyState::<KeyPosition>::new();
    positions.press(KeyPosition::B);
    positions.press(KeyPosition::M);
    positions.press(KeyPosition::LShift);
    positions.press(KeyPosition::C);

    let symbols = SymbolAdapter::map(&positions);
    assert_eq!(
      &vec![
        KeySymbol::Char('B'),
        KeySymbol::Char('M'),
        KeySymbol::Char('C'),
      ],
      symbols.pressed()
    );
  }

  #[test]
  fn test_digit_mapping() {
    let mut positions = KeyState::<KeyPosition>::new();
    positions.press(KeyPosition::Digit1);
    positions.press(KeyPosition::Digit2);
    positions.press(KeyPosition::Digit3);

    let symbols = SymbolAdapter::map(&positions);
    assert_eq!(
      &vec![
        KeySymbol::Char('1'),
        KeySymbol::Char('2'),
        KeySymbol::Char('3'),
      ],
      symbols.pressed()
    );

    positions.press(KeyPosition::LShift);

    let symbols = SymbolAdapter::map(&positions);
    assert_eq!(
      &vec![
        KeySymbol::Char('!'),
        KeySymbol::Char('@'),
        KeySymbol::Char('#'),
      ],
      symbols.pressed()
    );
  }

  #[test]
  fn test_special_keys() {
    let mut positions = KeyState::<KeyPosition>::new();
    positions.press(KeyPosition::Enter);
    positions.press(KeyPosition::Backspace);
    positions.press(KeyPosition::Space);

    let symbols = SymbolAdapter::map(&positions);
    assert_eq!(
      &vec![
        KeySymbol::Return,
        KeySymbol::Backspace,
        KeySymbol::Char(' '),
      ],
      symbols.pressed()
    );
  }

  #[test]
  fn test_mixed_keys() {
    let mut positions = KeyState::<KeyPosition>::new();

    // letter keys
    positions.press(KeyPosition::Q);
    positions.press(KeyPosition::A);
    positions.press(KeyPosition::B);

    // digit keys
    positions.press(KeyPosition::Digit6);
    positions.press(KeyPosition::Digit7);
    positions.press(KeyPosition::Digit8);

    // symbol keys
    positions.press(KeyPosition::Minus);
    positions.press(KeyPosition::Backslash);
    positions.press(KeyPosition::Semicolon);
    positions.press(KeyPosition::Comma);

    // special keys
    positions.press(KeyPosition::Enter);
    positions.press(KeyPosition::LAlt);

    let symbols = SymbolAdapter::map(&positions);

    assert_eq!(
      &vec![
        KeySymbol::Char('q'),
        KeySymbol::Char('a'),
        KeySymbol::Char('b'),
        KeySymbol::Char('6'),
        KeySymbol::Char('7'),
        KeySymbol::Char('8'),
        KeySymbol::Char('-'),
        KeySymbol::Char('\\'),
        KeySymbol::Char(';'),
        KeySymbol::Char(','),
        KeySymbol::Return,
        KeySymbol::LAlt,
      ],
      symbols.pressed()
    );

    positions.press(KeyPosition::LShift);

    let symbols = SymbolAdapter::map(&positions);

    assert_eq!(
      &vec![
        KeySymbol::Char('Q'),
        KeySymbol::Char('A'),
        KeySymbol::Char('B'),
        KeySymbol::Char('^'),
        KeySymbol::Char('&'),
        KeySymbol::Char('*'),
        KeySymbol::Char('_'),
        KeySymbol::Char('|'),
        KeySymbol::Char(':'),
        KeySymbol::Char('<'),
        KeySymbol::Return,
        KeySymbol::LAlt,
      ],
      symbols.pressed()
    );
  }

  #[test]
  fn test_press_release() {
    let mut positions = KeyState::<KeyPosition>::new();

    positions.press(KeyPosition::Q);
    positions.press(KeyPosition::A);
    positions.press(KeyPosition::B);
    positions.release(KeyPosition::A);

    assert!(positions.is_pressed(KeyPosition::Q));
    assert!(!positions.is_pressed(KeyPosition::A));
    assert!(positions.is_pressed(KeyPosition::B));

    positions.press(KeyPosition::M);
    positions.release(KeyPosition::Q);
    positions.press(KeyPosition::C);

    let symbols = SymbolAdapter::map(&positions);

    assert_eq!(
      &vec![
        KeySymbol::Char('b'),
        KeySymbol::Char('m'),
        KeySymbol::Char('c'),
      ],
      symbols.pressed()
    );
  }
}
