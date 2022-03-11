use std::str;
use std::fmt;

use tokio_postgres;

use crate::error;

#[derive(Debug)]
pub enum Error {
  MalformedScope(String),
  InvalidOperation(String),
  InvalidResource(String),
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::MalformedScope(msg) => write!(f, "malformed scope: {}", msg),
      Self::InvalidOperation(msg) => write!(f, "invalid operation: {}", msg),
      Self::InvalidResource(msg) => write!(f, "invalid resource: {}", msg),
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
  Read, Write, Delete,
}

impl Operation {
  pub fn parse(s: &str) -> Result<Operation, error::Error> {
    match s.trim().to_lowercase().as_ref() {
      "read"   => Ok(Operation::Read),
      "write"  => Ok(Operation::Write),
      "delete" => Ok(Operation::Delete),
      _        => Err(Error::InvalidOperation(format!("Invalid operation: {}", s)).into()),
    }
  }

  pub fn parse_list(s: &str) -> Result<Vec<Operation>, error::Error> {
    let mut ops = Vec::new();
    for e in s.split(',') {
      ops.push(Self::parse(e)?);
    }
    Ok(ops)
  }
}

impl fmt::Display for Operation {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Operation::Read   => write!(f, "read"),
      Operation::Write  => write!(f, "write"),
      Operation::Delete => write!(f, "delete"),
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Resource {
  System,
  Entry,
}

impl Resource {
  pub fn parse(s: &str) -> Result<Resource, error::Error> {
    match s.trim().to_lowercase().as_ref() {
      "system" => Ok(Resource::System),
      "entry"  => Ok(Resource::Entry),
      _        => Err(Error::InvalidResource(format!("Invalid resource: {}", s)).into()),
    }
  }
}

impl fmt::Display for Resource {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Resource::System => write!(f, "system"),
      Resource::Entry  => write!(f, "entry"),
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Scope {
  pub ops: Vec<Operation>,
  pub resource: Resource,
}

impl Scope {
  pub fn new(op: Operation, rc: Resource) -> Scope {
    Scope{
      ops: vec!(op),
      resource: rc,
    }
  }
  
  pub fn parse(s: &str) -> Result<Scope, error::Error> {
    let f: Vec<&str> = s.split(':').collect();
    if f.len() != 2 {
      return Err(Error::MalformedScope(format!("Invalid resource: {}", s)).into());
    }
    Ok(Scope{
      ops: Operation::parse_list(f[0])?,
      resource: Resource::parse(f[1])?,
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn parse_source() {
    assert_eq!(Scope::new(Operation::Read, Resource::System), Scope::parse("read:system").unwrap());
  }
  
}
