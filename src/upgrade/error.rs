use std::io;
use std::fmt;
use std::num;

#[derive(Debug)]
pub enum Error {
  DriverError,
  IOError(io::Error),
  ParseIntError(num::ParseIntError),
  VersionError(String),
  SequenceError(usize, usize),
  UpgradeError(usize, String, String),
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
      Self::DriverError => "Driver error".fmt(f),
      Self::IOError(err) => err.fmt(f),
      Self::ParseIntError(err) => err.fmt(f),
      Self::VersionError(msg) => write!(f, "Invalid version: {}", msg),
      Self::SequenceError(expect, actual) => write!(f, "Invalid version sequence; expected: {}, found: {}", expect, actual),
      Self::UpgradeError(version, resource, msg) => write!(f, "Could not upgrade to version #{} [{}]: {}", version, resource, msg),
    }
  }
}
