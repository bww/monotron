use std::io;
use std::fmt;
use std::num;

use url;
use xid;
use warp;
use base64;

use crate::acl;
use crate::store;
use crate::model;

#[derive(Debug)]
pub enum Error {
  StoreError(store::error::Error),
  ScopeError(acl::scope::Error),
  ApiKeyError(model::apikey::Error),
  NotFoundError(store::error::Error),
  IOError(io::Error),
  URLParseError(url::ParseError),
  ParseIdError(xid::ParseIdError),
  ParseIntError(num::ParseIntError),
  Utf8Error(std::str::Utf8Error),
  DecodeBase64Error(base64::DecodeError),
}

impl warp::reject::Reject for Error {}

impl From<store::error::Error> for Error {
  fn from(error: store::error::Error) -> Self {
    match error {
      store::error::Error::NotFoundError => Self::NotFoundError(error),
      other => Self::StoreError(other),
    }
  }
}

impl From<acl::scope::Error> for Error {
  fn from(error: acl::scope::Error) -> Self {
    Self::ScopeError(error)
  }
}

impl From<model::apikey::Error> for Error {
  fn from(error: model::apikey::Error) -> Self {
    Self::ApiKeyError(error)
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

impl From<std::str::Utf8Error> for Error {
  fn from(error: std::str::Utf8Error) -> Self {
    Self::Utf8Error(error)
  }
}

impl From<base64::DecodeError> for Error {
  fn from(error: base64::DecodeError) -> Self {
    Self::DecodeBase64Error(error)
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::StoreError(err) => err.fmt(f),
      Self::ScopeError(err) => err.fmt(f),
      Self::ApiKeyError(err) => err.fmt(f),
      Self::NotFoundError(err) => err.fmt(f),
      Self::IOError(err) => err.fmt(f),
      Self::URLParseError(err) => err.fmt(f),
      Self::ParseIdError(err) => err.fmt(f),
      Self::ParseIntError(err) => err.fmt(f),
      Self::Utf8Error(err) => err.fmt(f),
      Self::DecodeBase64Error(err) => err.fmt(f),
    }
  }
}
