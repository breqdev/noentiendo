use crate::keyboard::{KeyAdapter, KeyPosition, KeyState};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AppleKeys {
  Esc,
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
  Minus,
  Equals,
  Del,

  Tab,
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
  LeftBracket,
  RightBracket,

  Ctrl,
  A,
  S,
  D,
  F,
  G,
  H,
  J,
  K,
  L,
  Semicolon,
  Apostrophe,
  Grave,
  Return,

  LShift,
  Slash,
  Z,
  X,
  C,
  V,
  B,
  N,
  M,
  Comma,
  Period,
  Backslash,
  RShift,

  CapsLock,
  OpenApple,
  Space,
  ClosedApple,
  LeftArrow,
  RightArrow,
  DownArrow,
  UpArrow,

  NumEsc,
  NumLeft,
  NumRight,
  NumSpace,

  Num0,
  Num1,
  Num2,
  Num3,
  Num4,
  Num5,
  Num6,
  Num7,
  Num8,
  Num9,
  NumComma,
  NumPeriod,
  NumLeftParen,
  NumRightParen,
  NumMinus,
  NumDivide,
  NumPlus,
  NumMultiply,
  NumReturn,
  NumPrint,
}

/// An adapter for mapping positions on a standard keyboard to keys on an Apple IIe.
pub struct AppleKeyboardAdapter;

impl KeyAdapter<KeyPosition, AppleKeys> for AppleKeyboardAdapter {
  fn map(state: &KeyState<KeyPosition>) -> KeyState<AppleKeys> {
    let mut mapped = KeyState::new();

    for symbol in state.pressed() {
      use KeyPosition::*;

      mapped.press(match symbol {
        Escape => AppleKeys::Esc,
        Digit1 => AppleKeys::Digit1,
        Digit2 => AppleKeys::Digit2,
        Digit3 => AppleKeys::Digit3,
        Digit4 => AppleKeys::Digit4,
        Digit5 => AppleKeys::Digit5,
        Digit6 => AppleKeys::Digit6,
        Digit7 => AppleKeys::Digit7,
        Digit8 => AppleKeys::Digit8,
        Digit9 => AppleKeys::Digit9,
        Digit0 => AppleKeys::Digit0,
        Minus => AppleKeys::Minus,
        Equals => AppleKeys::Equals,
        Delete => AppleKeys::Del,

        Tab => AppleKeys::Tab,
        Q => AppleKeys::Q,
        W => AppleKeys::W,
        E => AppleKeys::E,
        R => AppleKeys::R,
        T => AppleKeys::T,
        Y => AppleKeys::Y,
        U => AppleKeys::U,
        I => AppleKeys::I,
        O => AppleKeys::O,
        P => AppleKeys::P,
        LeftBracket => AppleKeys::LeftBracket,
        RightBracket => AppleKeys::RightBracket,

        LControl | RControl => AppleKeys::Ctrl,
        A => AppleKeys::A,
        S => AppleKeys::S,
        D => AppleKeys::D,
        F => AppleKeys::F,
        G => AppleKeys::G,
        H => AppleKeys::H,
        J => AppleKeys::J,
        K => AppleKeys::K,
        L => AppleKeys::L,
        Semicolon => AppleKeys::Semicolon,
        Apostrophe => AppleKeys::Apostrophe,
        Grave => AppleKeys::Grave,
        Enter => AppleKeys::Return,

        LShift => AppleKeys::LShift,
        Slash => AppleKeys::Slash,
        Z => AppleKeys::Z,
        X => AppleKeys::X,
        C => AppleKeys::C,
        V => AppleKeys::V,
        B => AppleKeys::B,
        N => AppleKeys::N,
        M => AppleKeys::M,
        Comma => AppleKeys::Comma,
        Period => AppleKeys::Period,
        Backslash => AppleKeys::Backslash,
        RShift => AppleKeys::RShift,

        CapsLock => AppleKeys::CapsLock,
        LSuper => AppleKeys::OpenApple,
        Space => AppleKeys::Space,
        RSuper => AppleKeys::ClosedApple,
        LeftArrow => AppleKeys::LeftArrow,
        RightArrow => AppleKeys::RightArrow,
        DownArrow => AppleKeys::DownArrow,
        UpArrow => AppleKeys::UpArrow,

        NumLock => AppleKeys::NumEsc,
        // NumLeft => AppleKeys::NumLeft,
        // NumRight => AppleKeys::NumRight,
        // NumSpace => AppleKeys::NumSpace,
        Num0 => AppleKeys::Num0,
        Num1 => AppleKeys::Num1,
        Num2 => AppleKeys::Num2,
        Num3 => AppleKeys::Num3,
        Num4 => AppleKeys::Num4,
        Num5 => AppleKeys::Num5,
        Num6 => AppleKeys::Num6,
        Num7 => AppleKeys::Num7,
        Num8 => AppleKeys::Num8,
        Num9 => AppleKeys::Num9,
        // NumPeriod => AppleKeys::NumComma,
        NumPeriod => AppleKeys::NumPeriod,
        // NumLeftParen => AppleKeys::NumLeftParen,
        // NumRightParen => AppleKeys::NumRightParen,
        NumMinus => AppleKeys::NumMinus,
        NumDivide => AppleKeys::NumDivide,
        NumPlus => AppleKeys::NumPlus,
        NumMultiply => AppleKeys::NumMultiply,
        NumEnter => AppleKeys::NumReturn,
        PrintScreen => AppleKeys::NumPrint,

        _ => continue,
      });
    }

    mapped
  }
}