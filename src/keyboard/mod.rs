mod symbols;
pub use symbols::{KeyPosition, KeySymbol};

/// A set of keys that are currently pressed.
/// Parameter `T` is the type of the key symbols.
pub struct KeyState<T: PartialEq> {
  pressed: Vec<T>,
}

impl<T: PartialEq> KeyState<T> {
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
  pub fn pressed(&self) -> &Vec<T> {
    &self.pressed
  }

  /// Returns true if the given key is currently pressed.
  pub fn is_pressed(&self, symbol: T) -> bool {
    self.pressed.contains(&symbol)
  }
}

/// Represents a mapping from a key state of one type to a key state of another type.
/// Mappings can be symbolic (preserve symbols across the mapping, and rewrite
/// modifier keys as needed) or physical (maintain a one-to-one mapping from
/// physical keys to physical keys).
pub trait KeyAdapter<F: PartialEq, T: PartialEq> {
  /// Map the current state of the keyboard with symbols of type `F` to an
  /// equivalent keyboard state with symbols of type `T`.
  fn map(state: &KeyState<F>) -> KeyState<T>;
}
