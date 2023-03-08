use crate::{
  platform::{PlatformProvider, TapeState},
  roms::RomFile,
};
use std::{io::Write, sync::Arc};

const CLOCK_CYCLES_PER_BIT: u32 = 3284;

pub struct Tape {
  /// Current index within the "tape" (file).
  index: usize,

  /// The underlying tape file.
  file: Vec<u8>,

  /// The platform, containing the tape controls.
  platform: Arc<dyn PlatformProvider>,

  /// The previous state of the tape control buttons.
  prev_tape_state: TapeState,

  /// The value that should be written as the tape moves past.
  write: bool,
}

impl Tape {
  fn new(platform: Arc<dyn PlatformProvider>) -> Self {
    Self {
      index: 0,
      file: vec![0; 0x1000000],
      platform,
      prev_tape_state: TapeState::empty(),
      write: false,
    }
  }

  fn from_file(platform: Arc<dyn PlatformProvider>, tape: RomFile) -> Self {
    Self {
      index: 0,
      file: tape.get_data(),
      platform,
      prev_tape_state: TapeState::empty(),
      write: false,
    }
  }

  fn reset(&mut self) {
    self.index = 0;
  }

  fn read(&mut self) -> bool {
    let byte = self.file[self.index / 8];
    let bit = 1 << (self.index % 8);
    (byte & bit) != 0
  }

  fn write(&mut self, value: bool) {
    self.write = value;
  }

  fn sense(&mut self) -> bool {
    let tape_state = self.platform.get_tape_state();

    tape_state.play
  }

  fn advance(&mut self, bits: usize) {
    self.index += bits;
  }

  fn poll(&mut self, cycles: u32) {
    let tape_state = self.platform.get_tape_state();
    println!("index: {:04x}", self.index);

    if tape_state.record {
      let byte = &mut self.file[self.index / 8];
      let bit = 1 << (self.index % 8);
      if self.write {
        *byte |= bit;
      } else {
        *byte &= !bit;
      }
    }

    if tape_state.rewind && !self.prev_tape_state.rewind {
      println!("rewinding!");
      self.index = 0;
    }

    if tape_state.eject && !self.prev_tape_state.eject {
      println!("ejecting!");
      // save the tape contents as a binary file
      let mut file = std::fs::File::create("tape.bin").unwrap();
      file.write_all(&self.file).unwrap();
    }

    self.prev_tape_state = tape_state;
  }
}

pub struct Datasette {
  /// Whether the motor is currently on.
  motor: bool,

  /// The current tape.
  tape: Tape,

  /// Clock cycles remaining until the next bit should be written.
  ticks_until_ready: i32,
}

impl Datasette {
  pub fn new(platform: Arc<dyn PlatformProvider>) -> Self {
    Self {
      motor: false,
      tape: Tape::new(platform),
      ticks_until_ready: 0,
    }
  }

  pub fn from_file(platform: Arc<dyn PlatformProvider>, tape: RomFile) -> Self {
    Self {
      motor: false,
      tape: Tape::from_file(platform, tape),
      ticks_until_ready: 0,
    }
  }

  pub fn reset(&mut self) {
    self.tape.reset();
    self.motor = false;
  }

  pub fn set_motor(&mut self, motor: bool) {
    self.motor = motor;
  }

  pub fn write(&mut self, value: bool) {
    self.tape.write(value);
  }

  pub fn read(&mut self) -> bool {
    self.tape.read()
  }

  pub fn sense(&mut self) -> bool {
    !self.tape.sense()
  }

  pub fn tick(&mut self, cycles: u32) {
    self.tape.poll(cycles);

    if self.ticks_until_ready <= 0 {
      self.tape.advance(1);
      self.ticks_until_ready += CLOCK_CYCLES_PER_BIT as i32;
    }

    if self.motor {
      self.ticks_until_ready -= cycles as i32;
    }
  }
}
