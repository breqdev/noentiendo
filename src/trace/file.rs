use crate::trace::{CpuTrace, TraceHandler};
use std::{fs::File, io::Write};

pub struct FileTraceHandler {
  file: File,
}

impl FileTraceHandler {
  pub fn new(filename: String) -> Self {
    Self {
      file: File::create(filename).expect("Invalid filename"),
    }
  }
}

impl TraceHandler for FileTraceHandler {
  fn handle(&mut self, trace: &CpuTrace) {
    self
      .file
      .write_all(format!("{:04X}: {:02X}\n", trace.address, trace.opcode).as_bytes())
      .unwrap();
  }
}
