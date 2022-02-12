use crate::error;

#[derive(Debug, Clone, PartialEq)]
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

  pub fn new_with_token(key: &str, creator_id: u64, token: &str, value: u64) -> Entry {
    Entry{
      key: key.to_string(),
      creator_id: creator_id,
      token: Some(token.to_string()),
      value: value,
    }
  }
  
  pub fn next(&self) -> Entry {
    Entry{
      key: self.key.to_owned(),
      creator_id: self.creator_id,
      token: if let Some(tok) = &self.token { Some(tok.to_string()) } else { None },
      value: self.value + 1,
    }
  }
  
  pub fn next_with_token(&self, token: &str) -> Option<Entry> {
    if let Some(tok) = &self.token {
      if tok == token {
        return None; // token matches, no update
      }
    }
    Some(Entry{
      key: self.key.to_owned(),
      creator_id: self.creator_id,
      token: Some(token.to_string()),
      value: self.value + 1,
    })
  }
  
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn next_unqualified() {
    let ent = Entry::new("a", 1, 10);
    assert_eq!(Entry::new("a", 1, 11), ent.next());
    
    let ent = ent.next();
    assert_eq!(Entry::new("a", 1, 12), ent.next());
    
    let ent = ent.next();
    assert_eq!(Entry::new("a", 1, 13), ent.next());
    
    let ent = ent.next();
    assert_eq!(Entry::new("a", 1, 14), ent.next());
  }
  
  #[test]
  fn next_with_token() {
    let ent = Entry::new("a", 1, 10);
    assert_eq!(Some(Entry::new_with_token("a", 1, "d261470109", 11)), ent.next_with_token("d261470109"));
    
    let ent = if let Some(nxt) = ent.next_with_token("d261470109") { nxt } else { ent };
    assert_eq!(None, ent.next_with_token("d261470109")); // no change, same token
    assert_eq!(Some(Entry::new_with_token("a", 1, "3096048bb3", 12)), ent.next_with_token("3096048bb3")); // different token, inc again
    
    let ent = if let Some(nxt) = ent.next_with_token("3096048bb3") { nxt } else { ent };
    assert_eq!(None, ent.next_with_token("3096048bb3")); // same again
  }
  
}
