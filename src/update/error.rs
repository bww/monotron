use std::io;
use std::fmt;

#[derive(Debug)]
pub enum Error {
  DriverError,
  IOError(io::Error),
}

impl From<io::Error> for Error {
  fn from(error: io::Error) -> Self {
    Self::IOError(error)
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::DriverError => "driver_error".fmt(f),
      Self::IOError(err) => err.fmt(f),
    }
  }
}
