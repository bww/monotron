use std::io;
use std::fmt;
use std::num;

use crossbeam_channel;

#[derive(Debug)]
pub enum Error {
  DriverError(String),
  IOError(io::Error),
  ParseIntError(num::ParseIntError),
  RecvError(crossbeam_channel::RecvError),
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

impl From<crossbeam_channel::RecvError> for Error {
  fn from(error: crossbeam_channel::RecvError) -> Self {
    Self::RecvError(error)
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::DriverError(msg) => write!(f, "Driver error: {}", msg),
      Self::IOError(err) => err.fmt(f),
      Self::ParseIntError(err) => err.fmt(f),
      Self::RecvError(err) => err.fmt(f),
      Self::VersionError(msg) => write!(f, "Invalid version: {}", msg),
      Self::SequenceError(expect, actual) => write!(f, "Invalid version sequence; expected: {}, found: {}", expect, actual),
      Self::UpgradeError(version, resource, msg) => write!(f, "Could not upgrade to version #{} [{}]: {}", version, resource, msg),
    }
  }
}
