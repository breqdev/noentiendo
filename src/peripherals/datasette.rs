use crate::roms::RomFile;

const CLOCK_CYCLES_PER_BIT: u32 = 3284;

pub struct Tape {
  /// Current index within the "tape" (file).
  index: usize,

  /// The underlying tape file.
  file: Option<Vec<u8>>,
}

impl Tape {
  fn new() -> Self {
    Self {
      index: 0,
      file: None,
    }
  }

  fn from_file(tape: RomFile) -> Self {
    Self {
      index: 0,
      file: Some(tape.get_data()),
    }
  }

  fn reset(&mut self) {
    self.index = 0;
  }

  fn read(&mut self) -> bool {
    println!("tape read: {}", self.index);
    match &self.file {
      Some(file) => {
        let byte = file[self.index / 8];
        let bit = (byte >> (self.index % 8)) & 1;
        bit == 1
      }
      None => false,
    }
  }

  fn advance(&mut self, bits: usize) {
    self.index += bits;
  }

  fn is_present(&mut self) -> bool {
    return self.file.is_some();
  }
}

pub struct Datasette {
  /// Whether the motor is currently on.
  motor: bool,

  /// The value to write, if the tape is being written to.
  write: bool,

  /// The current tape.
  tape: Tape,

  /// Clock cycles remaining until the next bit should be written.
  ticks_until_ready: u32,
}

impl Datasette {
  pub fn new() -> Self {
    Self {
      motor: false,
      write: false,
      tape: Tape::new(),
      ticks_until_ready: 0,
    }
  }

  pub fn from_file(tape: RomFile) -> Self {
    Self {
      motor: false,
      write: false,
      tape: Tape::from_file(tape),
      ticks_until_ready: 0,
    }
  }

  pub fn reset(&mut self) {
    self.tape.reset();
    self.motor = false;
    self.write = false;
  }

  pub fn set_motor(&mut self, motor: bool) {
    println!("motor: {}", motor);
    self.motor = motor;
  }

  pub fn write(&mut self, value: bool) {
    println!("write: {}", value);
    self.write = value;
  }

  pub fn read(&mut self) -> bool {
    println!("read");
    self.tape.read()
  }

  pub fn sense(&mut self) -> bool {
    println!("sense");
    // TODO: proper play/record/stop

    use std::fs;

    let path = "sense.txt";

    let contents = fs::read_to_string(path).unwrap();

    // extract 0 or 1
    let value = match contents.chars().nth(0) {
      Some('0') => false,
      Some('1') => true,
      _ => false,
    };

    !value // low indicates tape present
  }

  pub fn tick(&mut self, cycles: u32) {
    if self.ticks_until_ready <= 0 {
      self.tape.advance(1);
      // TODO: if in write mode, do write
    }

    if self.motor {
      self.ticks_until_ready += CLOCK_CYCLES_PER_BIT - cycles;
    }
  }
}
