use crate::trace::{CpuTrace, TraceHandler};
use std::{
  fs::File,
  io::{BufWriter, Write},
};

pub struct FileTraceHandler {
  file: BufWriter<File>,
}

impl FileTraceHandler {
  pub fn new(filename: String) -> Self {
    Self {
      file: BufWriter::new(File::create(filename).expect("Invalid filename")),
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

  fn flush(&mut self) -> Result<(), &str> {
    self.file.flush().map_err(|_| "failed to flush file")
  }
}
