use crate::error;

#[derive(Debug, PartialEq)]
pub struct Entry {
  key: String,
  creator_id: u64,
  token: Option<String>,
  value: u64,
}

impl Entry {
  
  pub fn new(key: &str, creator_id: u64, value: u64) -> Entry {
    Entry{
      key: key.to_string(),
      creator_id: creator_id,
      token: None,
      value: value,
    }
  }

  pub fn new_with_token(key: &str, token: &str, creator_id: u64, value: u64) -> Entry {
    Entry{
      key: key.to_string(),
      creator_id: creator_id,
      token: Some(token.to_string()),
      value: value,
    }
  }
  
  pub fn next(&self) -> Result<Entry, error::Error> {
    Ok(Entry{
      key: self.key.to_owned(),
      creator_id: self.creator_id,
      token: if let Some(tok) = &self.token { Some(tok.to_string()) } else { None },
      value: self.value + 1,
    })
  }
  
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn next_single() {
    let entry = Entry::new("a", 1, 10);
    let expect = Entry::new("a", 1, 11);
    match entry.next() {
      Ok(v) => assert_eq!(expect, v),
      Err(err) => assert!(false, "Expected no error; got {:?}", err),
    };
    let entry = expect;
    let expect = Entry::new("a", 1, 12);
    match entry.next() {
      Ok(v) => assert_eq!(expect, v),
      Err(err) => assert!(false, "Expected no error; got {:?}", err),
    };
  }
  
}
