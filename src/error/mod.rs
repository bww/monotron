use std::io;
use std::fmt;
use std::error;

use url;
use xid;
use tokio_postgres;

use crate::store;

#[derive(Debug)]
pub struct Generic {
  msg: String,
}

impl Generic {
  pub fn new(msg: String) -> Generic {
    Generic{msg: msg}
  }
}

impl error::Error for Generic {
  fn source(&self) -> Option<&(dyn error::Error + 'static)> {
    None
  }
}

impl fmt::Display for Generic {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.msg)
  }
}

#[derive(Debug)]
pub enum Error {
  Generic(Generic),
  StoreError(store::error::Error),
  NotFoundError(store::error::Error),
  IOError(io::Error),
  URLParseError(url::ParseError),
  PostgresError(tokio_postgres::Error),
  ParseIdError(xid::ParseIdError)
}

impl From<Generic> for Error {
  fn from(error: Generic) -> Self {
    Self::Generic(error)
  }
}

impl From<store::error::Error> for Error {
  fn from(error: store::error::Error) -> Self {
    match error {
      store::error::Error::NotFoundError(_) => Self::NotFoundError(error),
      other => Self::StoreError(other),
    }
  }
}

impl From<io::Error> for Error {
  fn from(error: io::Error) -> Self {
    Self::IOError(error)
  }
}

impl From<url::ParseError> for Error {
  fn from(error: url::ParseError) -> Self {
    Self::URLParseError(error)
  }
}

impl From<tokio_postgres::Error> for Error {
  fn from(error: tokio_postgres::Error) -> Self {
    Self::PostgresError(error)
  }
}

impl From<xid::ParseIdError> for Error {
  fn from(error: xid::ParseIdError) -> Self {
    Self::ParseIdError(error)
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Generic(err) => err.fmt(f),
      Self::StoreError(err) => err.fmt(f),
      Self::NotFoundError(err) => err.fmt(f),
      Self::IOError(err) => err.fmt(f),
      Self::URLParseError(err) => err.fmt(f),
      Self::PostgresError(err) => err.fmt(f),
      Self::ParseIdError(err) => err.fmt(f),
    }
  }
}
