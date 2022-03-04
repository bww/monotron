use crate::error;
use crate::store;

use warp;
use serde::{Serialize, Deserialize};
use serde_json::json;
use tokio_postgres;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Entry {
  pub key: String,
  pub creator_id: i64,
  pub token: Option<String>,
  pub value: i64,
}

impl Entry {
  
  pub fn new(key: &str, creator_id: i64, token: Option<String>, value: i64) -> Entry {
    Entry{
      key: key.to_string(),
      creator_id: creator_id,
      token: token,
      value: value,
    }
  }
  
  pub fn unmarshal(row: &tokio_postgres::Row) -> Result<Entry, store::error::Error> {
    Ok(Entry{
      key: row.try_get(0)?,
      creator_id: row.try_get(1)?,
      token: row.try_get(2)?,
      value: row.try_get(3)?,
    })
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

impl warp::Reply for Entry {
  fn into_response(self) -> warp::reply::Response {
    warp::reply::Response::new(json!(self).to_string().into())
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
    assert_eq!(Some(Entry::new("a", 1, Some("d261470109"), 11)), ent.next_with_token("d261470109"));
    
    let ent = if let Some(nxt) = ent.next_with_token("d261470109") { nxt } else { ent };
    assert_eq!(None, ent.next_with_token("d261470109")); // no change, same token
    assert_eq!(Some(Entry::new("a", 1, Some("3096048bb3"), 12)), ent.next_with_token("3096048bb3")); // different token, inc again
    
    let ent = if let Some(nxt) = ent.next_with_token("3096048bb3") { nxt } else { ent };
    assert_eq!(None, ent.next_with_token("3096048bb3")); // same again
  }
  
}
