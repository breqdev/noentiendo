/// Keys and adapters for the Commodore 64, VIC-20, and other Commodore machines.
pub mod commodore;

/// Keys used for positional keyboard mapping.
mod positions;

/// Keys used for symbolic keyboard mapping.
mod symbols;

/// Keys which can be pressed on a virtual / emulated keyboard.
mod virtualkey;

use std::ops::BitOr;

pub use positions::KeyPosition;
pub use symbols::{KeySymbol, SymbolAdapter};
pub use virtualkey::VirtualKey;

/// A set of keys that are currently pressed.
/// Parameter `T` is the type of the key symbols.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct KeyState<T: PartialEq + Clone> {
  pressed: Vec<T>,
}

impl<T: PartialEq + Clone> KeyState<T> {
  /// Creates a new, empty key state.
  pub fn new() -> Self {
    Self {
      pressed: Vec::new(),
    }
  }

  /// Adds a key to the set of pressed keys.
  pub fn press(&mut self, symbol: T) {
    self.pressed.push(symbol);
  }

  /// Removes a key from the set of pressed keys.
  pub fn release(&mut self, symbol: T) {
    self.pressed.retain(|s| *s != symbol);
  }

  /// Return the set of pressed keys.
  pub fn pressed(&self) -> impl Iterator<Item = &T> {
    self.pressed.iter()
  }

  /// Returns true if the given key is currently pressed.
  pub fn is_pressed(&self, symbol: T) -> bool {
    self.pressed.contains(&symbol)
  }

  /// Returns true if the set of keys is empty.
  pub fn is_empty(&self) -> bool {
    self.pressed.is_empty()
  }

  /// Returns the most recent key pressed.
  pub fn get_one_key(&self) -> Option<T> {
    self.pressed.last().cloned()
  }
}

impl<T: PartialEq + Clone> BitOr<KeyState<T>> for KeyState<T> {
  type Output = KeyState<T>;

  fn bitor(self, rhs: Self) -> Self::Output {
    let mut pressed = self.pressed;
    for key in rhs.pressed {
      if !pressed.contains(&key) {
        pressed.push(key);
      }
    }
    KeyState { pressed }
  }
}

/// Represents a mapping from a key state of one type to a key state of another type.
/// Mappings can be symbolic (preserve symbols across the mapping, and rewrite
/// modifier keys as needed) or physical (maintain a one-to-one mapping from
/// physical keys to physical keys).
pub trait KeyAdapter<F: PartialEq + Clone, T: PartialEq + Clone> {
  /// Map the current state of the keyboard with symbols of type `F` to an
  /// equivalent keyboard state with symbols of type `T`.
  fn map(state: &KeyState<F>) -> KeyState<T>;
}

/// Represents different approaches to mapping key states, to allow the user to
/// indicate their preference.
pub enum KeyMappingStrategy {
  /// Preserve physical keys one-to-one. This is most compatible, but the
  /// resulting mapping may be less intuitive. For instance, symbols may
  /// not be mapped as expected.
  Physical,

  /// Preserve symbols one-to-one. This is more intuitive, but may cause issues
  /// with some software. This approach will rewrite the state of the modifier
  /// keys to convey the symbols being pressed.
  Symbolic,
}
