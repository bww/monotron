use std::io;
use std::fmt;
use std::num;
use std::error;

use url;
use xid;
use warp;

use crate::store;
use crate::model;

#[derive(Debug)]
pub struct Generic {
  msg: String,
}

impl Generic {
  pub fn new(msg: &str) -> Generic {
    Generic{msg: msg.to_string()}
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
  ScopeError(model::scope::Error),
  NotFoundError(store::error::Error),
  Unauthorized,
  IOError(io::Error),
  URLParseError(url::ParseError),
  ParseIdError(xid::ParseIdError),
  ParseIntError(num::ParseIntError),
}

impl warp::reject::Reject for Error {}

impl From<Generic> for Error {
  fn from(error: Generic) -> Self {
    Self::Generic(error)
  }
}

impl From<store::error::Error> for Error {
  fn from(error: store::error::Error) -> Self {
    match error {
      store::error::Error::NotFoundError => Self::NotFoundError(error),
      other => Self::StoreError(other),
    }
  }
}

impl From<model::scope::Error> for Error {
  fn from(error: model::scope::Error) -> Self {
    Self::ScopeError(error)
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

impl From<xid::ParseIdError> for Error {
  fn from(error: xid::ParseIdError) -> Self {
    Self::ParseIdError(error)
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
      Self::Generic(err) => err.fmt(f),
      Self::StoreError(err) => err.fmt(f),
      Self::ScopeError(err) => err.fmt(f),
      Self::NotFoundError(err) => err.fmt(f),
      Self::Unauthorized => f.write_str("Unauthorized"),
      Self::IOError(err) => err.fmt(f),
      Self::URLParseError(err) => err.fmt(f),
      Self::ParseIdError(err) => err.fmt(f),
      Self::ParseIntError(err) => err.fmt(f),
    }
  }
}
