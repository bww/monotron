use std::io;
use std::fmt;

use url;
use bb8;
use warp;
use tokio_postgres;

use crate::model;

#[derive(Debug)]
pub enum Error {
  NotFoundError,
  URLParseError(url::ParseError),
  IOError(io::Error),
  PostgresError(tokio_postgres::Error),
  ConnectionError(bb8::RunError<tokio_postgres::Error>),
  ScopeError(model::scope::Error),
}

impl warp::reject::Reject for Error {}

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

impl From<bb8::RunError<tokio_postgres::Error>> for Error {
  fn from(error: bb8::RunError<tokio_postgres::Error>) -> Self {
    Self::ConnectionError(error)
  }
}

impl From<model::scope::Error> for Error {
  fn from(error: model::scope::Error) -> Self {
    Self::ScopeError(error)
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::NotFoundError => "not_found".fmt(f),
      Self::URLParseError(err) => err.fmt(f),
      Self::IOError(err) => err.fmt(f),
      Self::PostgresError(err) => err.fmt(f),
      Self::ConnectionError(err) => err.fmt(f),
      Self::ScopeError(err) => err.fmt(f),
    }
  }
}
