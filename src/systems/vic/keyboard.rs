use crate::keyboard::{KeyAdapter, KeyPosition, KeyState};

/// The keys found on a VIC-20 keyboard.
/// Source: http://sleepingelephant.com/denial/wiki/index.php?title=Keyboard
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Vic20Keys {
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
  Restore,

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

// The keyboard matrix with various modifier keys applied.

// https://www.lemon64.com/forum/viewtopic.php?t=68210&sid=8b976b9f8699fc3588c5622b43a1f4b1

/// The keyboard matrix in a VIC-20 system.
pub const KEYBOARD_MAPPING: [[Vic20Keys; 8]; 8] = {
  use Vic20Keys::*;

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

/// An adapter that maps standard keyboard positions to keys on the VIC-20 keyboard.
pub struct Vic20KeyboardAdapter {}

impl KeyAdapter<KeyPosition, Vic20Keys> for Vic20KeyboardAdapter {
  fn map(state: &KeyState<KeyPosition>) -> KeyState<Vic20Keys> {
    let mut mapped = KeyState::new();

    for symbol in state.pressed() {
      use KeyPosition::*;

      mapped.press(match symbol {
        Grave => Vic20Keys::LeftArrow,
        Digit1 => Vic20Keys::Digit1,
        Digit2 => Vic20Keys::Digit2,
        Digit3 => Vic20Keys::Digit3,
        Digit4 => Vic20Keys::Digit4,
        Digit5 => Vic20Keys::Digit5,
        Digit6 => Vic20Keys::Digit6,
        Digit7 => Vic20Keys::Digit7,
        Digit8 => Vic20Keys::Digit8,
        Digit9 => Vic20Keys::Digit9,
        Digit0 => Vic20Keys::Digit0,
        // TODO: Plus
        Minus => Vic20Keys::Minus,
        // TODO: Pound
        // TODO: ClrHome
        Backspace => Vic20Keys::InsertDelete,

        LControl => Vic20Keys::Control,
        Q => Vic20Keys::Q,
        W => Vic20Keys::W,
        E => Vic20Keys::E,
        R => Vic20Keys::R,
        T => Vic20Keys::T,
        Y => Vic20Keys::Y,
        U => Vic20Keys::U,
        I => Vic20Keys::I,
        O => Vic20Keys::O,
        P => Vic20Keys::P,
        // TODO: At
        // TODO: Asterisk
        // TODO: UpArrow
        // TODO: Restore

        // TODO: RunStop
        CapsLock => Vic20Keys::ShiftLock,
        A => Vic20Keys::A,
        S => Vic20Keys::S,
        D => Vic20Keys::D,
        F => Vic20Keys::F,
        G => Vic20Keys::G,
        H => Vic20Keys::H,
        J => Vic20Keys::J,
        K => Vic20Keys::K,
        L => Vic20Keys::L,
        // TODO: Colon
        Semicolon => Vic20Keys::Semicolon,
        Equals => Vic20Keys::Equals,
        Enter => Vic20Keys::Return,

        LAlt => Vic20Keys::Commodore,
        LShift => Vic20Keys::LShift,
        Z => Vic20Keys::Z,
        X => Vic20Keys::X,
        C => Vic20Keys::C,
        V => Vic20Keys::V,
        B => Vic20Keys::B,
        N => Vic20Keys::N,
        M => Vic20Keys::M,
        Comma => Vic20Keys::Comma,
        Period => Vic20Keys::Period,
        Slash => Vic20Keys::Slash,
        RShift => Vic20Keys::RShift,
        // TODO: CursorUpDown
        // TODO: CursorLeftRight
        Space => Vic20Keys::Space,

        F1 => Vic20Keys::F1,
        F3 => Vic20Keys::F3,
        F5 => Vic20Keys::F5,
        F7 => Vic20Keys::F7,

        _ => continue,
      });
    }

    mapped
  }
}
