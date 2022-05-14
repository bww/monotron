use std::io;
use std::fmt;
use std::num;

#[derive(Debug)]
pub enum Error {
  DriverError,
  IOError(io::Error),
  ParseIntError(num::ParseIntError),
}

impl From<io::Error> for Error {
  fn from(error: io::Error) -> Self {
    Self::IOError(error)
  }
}

impl From<num::ParseIntError> for Error {
  fn from(error: num::ParseIntError) -> Self {
    Self::ParseIntError(error)
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::DriverError => "driver_error".fmt(f),
      Self::IOError(err) => err.fmt(f),
      Self::ParseIntError(err) => err.fmt(f),
    }
  }
}
