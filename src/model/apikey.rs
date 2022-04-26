use std::fmt;
use std::str;

use rand::{self, Rng};
use serde::{Serialize, Deserialize};
use serde_json::json;
use tokio_postgres;

use crate::store;
use crate::acl::scope;

const AUTH_TYPE_BASIC: &str = "Basic";

#[derive(Debug, PartialEq)]
pub enum Error {
  Unauthorized(String),
  Forbidden(String),
  Utf8Error(std::str::Utf8Error),
  DecodeBase64Error(base64::DecodeError),
}

impl warp::reject::Reject for Error {}

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
      Self::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
      Self::Forbidden(msg) => write!(f, "Forbidden: {}", msg),
      Self::Utf8Error(err) => err.fmt(f),
      Self::DecodeBase64Error(err) => err.fmt(f),
    }
  }
}

fn random_string(len: usize) -> String {
  rand::thread_rng()
    .sample_iter(&rand::distributions::Alphanumeric)
    .take(len)
    .map(char::from)
    .collect()
}

pub fn gen_apikey() -> (String, String) {
  (random_string(24), random_string(128))
}

pub fn parse_apikey(data: &str) -> Result<(String, String), Error> {
  let parts: Vec<&str> = data.split(' ').collect();
  if parts.len() != 2 {
    return Err(Error::Unauthorized("Invalid authorization provided".to_string()).into());
  }
  if parts[0].trim() != AUTH_TYPE_BASIC {
    return Err(Error::Unauthorized("Unsupported authorization type".to_string()).into());
  }
  let data = match base64::decode(parts[1].trim()) {
    Ok(data) => data,
    Err(err) => return Err(Error::DecodeBase64Error(err).into()),
  };
  let data = match str::from_utf8(data.as_slice()) {
    Ok(data) => data,
    Err(err) => return Err(Error::Utf8Error(err).into()),
  };
  let parts: Vec<&str> = data.split(':').collect();
  if parts.len() != 2 {
    return Err(Error::Unauthorized("Invalid authorization data provided".to_string()).into());
  }
  Ok((parts[0].to_string(), parts[1].to_string()))
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiKey {
  pub id: i64,
  pub key: String,
  pub secret: String,
}

impl ApiKey {
  pub fn unmarshal(row: &tokio_postgres::Row) -> Result<ApiKey, store::error::Error> {
    Ok(Self{
      id: row.try_get(0)?,
      key: row.try_get(1)?,
      secret: row.try_get(2)?,
    })
  }
  
  pub fn auth(&self, key: &str, secret: &str) -> bool {
    self.key == key && self.secret == secret
  }
  
  pub fn with_id(&self, id: i64) -> Self {
    Self{
      id: id,
      key: self.key.to_owned(),
      secret: self.secret.to_owned(),
    }
  }
}

impl warp::Reply for ApiKey {
  fn into_response(self) -> warp::reply::Response {
    warp::reply::Response::new(json!(self).to_string().into())
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Authorization {
  pub api_key: ApiKey,
  pub account_id: i64,
  pub scopes: scope::Scopes,
}

impl Authorization {
  pub fn unmarshal(row: &tokio_postgres::Row) -> Result<Authorization, store::error::Error> {
    let scope_specs: Vec<String> = row.try_get(3)?;
    let scope_set: Vec<scope::Scope> = if scope_specs.len() > 0 {
      scope::Scope::parse_set(scope_specs)?
    }else{
      Vec::new()
    };
    Ok(Authorization{
      api_key: ApiKey::unmarshal(row)?,
      account_id: row.try_get(4)?,
      scopes: scope::Scopes::new(scope_set),
    })
  }
  
  pub fn auth(&self, key: &str, secret: &str) -> bool {
    self.api_key.auth(key, secret)
  }
  
  pub fn allows(&self, op: scope::Operation, rc: scope::Resource) -> bool {
    self.scopes.allows(op, rc)
  }
  
  pub fn assert_allows(&self, op: scope::Operation, rc: scope::Resource) -> Result<(), Error> {
    if self.allows(op, rc) {
      Ok(())
    }else{
      Err(Error::Forbidden(format!("{} cannot satisfy: {}:{}", self.scopes, op, rc)))
    }
  }
  
  pub fn assert_allows_in_account(&self, account_id: i64, op: scope::Operation, rc: scope::Resource) -> Result<(), Error> {
    if self.account_id != account_id {
      return Err(Error::Forbidden(format!("Account mismatch: {} != {}", self.account_id, account_id)))
    }
    if !self.allows(op, rc) {
      return Err(Error::Forbidden(format!("{} cannot satisfy: {}:{}", self.scopes, op, rc)))
    }
    Ok(())
  }
}

impl warp::Reply for Authorization {
  fn into_response(self) -> warp::reply::Response {
    warp::reply::Response::new(json!(self).to_string().into())
  }
}
