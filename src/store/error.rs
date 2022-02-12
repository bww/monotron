use std::io;
use std::fmt;

use url;
use tokio_postgres;

#[derive(Debug)]
pub enum Error {
  URLParseError(url::ParseError),
  IOError(io::Error),
  NotFoundError(tokio_postgres::Error),
  PostgresError(tokio_postgres::Error),
}

impl From<url::ParseError> for Error {
  fn from(error: url::ParseError) -> Self {
    Self::URLParseError(error)
  }
}

impl From<io::Error> for Error {
  fn from(error: io::Error) -> Self {
    Self::IOError(error)
  }
}

impl From<tokio_postgres::Error> for Error {
  fn from(error: tokio_postgres::Error) -> Self {
    Self::PostgresError(error)
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::URLParseError(err) => err.fmt(f),
      Self::IOError(err) => err.fmt(f),
      Self::NotFoundError(err) => err.fmt(f),
      Self::PostgresError(err) => err.fmt(f),
    }
  }
}
