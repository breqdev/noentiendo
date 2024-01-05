use crate::keyboard::{KeyAdapter, KeyState, KeySymbol};

pub struct AppleIISymbolAdapter;

impl KeyAdapter<KeySymbol, u8> for AppleIISymbolAdapter {
  fn map(state: &KeyState<KeySymbol>) -> KeyState<u8> {
    let mut mapped = KeyState::new();

    for symbol in state.pressed() {
      use KeySymbol::*;

      let mapped_symbol = match symbol {
        &Char(c) => Some(c as u8),
        Return => Some(0x0D), // Carriage Return
        Backspace => Some(0x08),
        Escape => Some(0x1B),
        _ => None,
      };

      if let Some(symbol) = mapped_symbol {
        mapped.press(symbol);
      }
    }

    mapped
  }
}
