use std::str;
use std::fmt;

use tokio_postgres;

use crate::error;

#[derive(Debug, PartialEq)]
pub enum Error {
  MalformedScope(String),
  InvalidOperation(String),
  InvalidResource(String),
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::MalformedScope(msg) => write!(f, "Malformed scope: {}", msg),
      Self::InvalidOperation(msg) => write!(f, "Invalid operation: {}", msg),
      Self::InvalidResource(msg) => write!(f, "Invalid resource: {}", msg),
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
  Read, Write, Delete,
}

impl Operation {
  pub fn parse(s: &str) -> Result<Operation, Error> {
    match s.trim().to_lowercase().as_ref() {
      "read"   => Ok(Operation::Read),
      "write"  => Ok(Operation::Write),
      "delete" => Ok(Operation::Delete),
      _        => Err(Error::InvalidOperation(format!("Invalid operation: {:?}", s))),
    }
  }

  pub fn parse_list(s: &str) -> Result<Vec<Operation>, Error> {
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
  ACL,
  Entry,
}

impl Resource {
  pub fn parse(s: &str) -> Result<Resource, Error> {
    match s.trim().to_lowercase().as_ref() {
      "system" => Ok(Resource::System),
      "acl"    => Ok(Resource::ACL),
      "entry"  => Ok(Resource::Entry),
      _        => Err(Error::InvalidResource(format!("Invalid resource: {:?}", s))),
    }
  }
}

impl fmt::Display for Resource {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Resource::System => write!(f, "system"),
      Resource::ACL    => write!(f, "acl"),
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
  
  pub fn parse(s: &str) -> Result<Scope, Error> {
    let f: Vec<&str> = s.trim().split(':').collect();
    if f.len() != 2 {
      return Err(Error::MalformedScope(format!("Malformed scope: {:?}", s)));
    }
    Ok(Scope{
      ops: Operation::parse_list(f[0])?,
      resource: Resource::parse(f[1])?,
    })
  }
}

impl fmt::Display for Scope {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let ops: Vec<String> = self.ops.iter().map(|e| e.to_string()).collect();
    write!(f, "{}:{}", ops.join(","), self.resource)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn format_source() {
    assert_eq!("read:system".to_string(), Scope{ops: vec!(Operation::Read), resource: Resource::System}.to_string());
    assert_eq!("read,write:system".to_string(), Scope{ops: vec!(Operation::Read, Operation::Write), resource: Resource::System}.to_string());
    assert_eq!("read,write,delete:system".to_string(), Scope{ops: vec!(Operation::Read, Operation::Write, Operation::Delete), resource: Resource::System}.to_string());
  }
  
  #[test]
  fn parse_source() {
    assert_eq!(Ok(Scope::new(Operation::Read, Resource::System)), Scope::parse("read:system"));
    assert_eq!(Ok(Scope::new(Operation::Read, Resource::System)), Scope::parse("READ:SYSTEM"));
    assert_eq!(Ok(Scope::new(Operation::Read, Resource::System)), Scope::parse("read : system"));
    assert_eq!(Ok(Scope{ops: vec!(Operation::Read, Operation::Write), resource: Resource::System}), Scope::parse("read,write:system"));
    assert_eq!(Ok(Scope{ops: vec!(Operation::Read, Operation::Write), resource: Resource::System}), Scope::parse("read , write : system"));
    assert_eq!(Ok(Scope{ops: vec!(Operation::Read, Operation::Write, Operation::Delete), resource: Resource::System}), Scope::parse("read,write,delete:system"));
    assert_eq!(Ok(Scope{ops: vec!(Operation::Read, Operation::Write, Operation::Delete), resource: Resource::System}), Scope::parse(" read , write , delete : system "));
    
    assert_eq!(Err(Error::MalformedScope("Malformed scope: \"\"".to_string())), Scope::parse(""));
    assert_eq!(Err(Error::MalformedScope("Malformed scope: \"foo\"".to_string())), Scope::parse("foo"));
    assert_eq!(Err(Error::InvalidOperation("Invalid operation: \"foo\"".to_string())), Scope::parse("foo:bar"));
    assert_eq!(Err(Error::InvalidResource("Invalid resource: \"bar\"".to_string())), Scope::parse("read:bar"));
    assert_eq!(Err(Error::InvalidOperation("Invalid operation: \"\"".to_string())), Scope::parse("read,:bar"));
    assert_eq!(Err(Error::InvalidOperation("Invalid operation: \"foo\"".to_string())), Scope::parse("read,foo:bar"));
    assert_eq!(Err(Error::InvalidResource("Invalid resource: \"bar\"".to_string())), Scope::parse("read,write:bar"));
    assert_eq!(Err(Error::MalformedScope("Malformed scope: \"read,write\"".to_string())), Scope::parse("read,write"));
    assert_eq!(Err(Error::InvalidResource("Invalid resource: \"\"".to_string())), Scope::parse("read,write: "));
  }
  
}
