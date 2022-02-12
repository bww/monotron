use crate::error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
  Single(u64),
  Semver(String),
}

#[derive(Debug, PartialEq)]
pub struct Entry {
  token: Option<String>,
  value: Value,
}

impl Entry {
  
  pub fn new(value: &Value) -> Entry {
    Entry{
      token: None,
      value: value.to_owned(),
    }
  }
  
  pub fn new_with_token(token: &str, value: &Value) -> Entry {
    Entry{
      token: Some(token.to_string()),
      value: value.to_owned(),
    }
  }
  
  pub fn next(&self) -> Result<Entry, error::Error> {
    match self.value {
      Value::Single(val) => Ok(Entry::new(&Value::Single(val + 1))),
      _ => Err(error::Generic::new("Method does not support type").into())
    }
  }
  
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn next_single() {
    let entry = Entry::new(&Value::Single(10));
    let expect = Entry::new(&Value::Single(11));
    match entry.next() {
      Ok(v) => assert_eq!(expect, v),
      Err(err) => assert!(false, "Expected no error; got {:?}", err),
    };
    let entry = expect;
    let expect = Entry::new(&Value::Single(12));
    match entry.next() {
      Ok(v) => assert_eq!(expect, v),
      Err(err) => assert!(false, "Expected no error; got {:?}", err),
    };
  }
  
}
