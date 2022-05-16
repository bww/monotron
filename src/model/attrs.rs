use std::collections;

use warp;
use serde_json::json;

#[derive(Debug, Clone, PartialEq)]
pub struct Attrs {
  pub attrs: collections::HashMap<String, String>,
}

impl Attrs {
  pub fn new(attrs: collections::HashMap<String, String>) -> Attrs {
    Attrs{
      attrs: attrs,
    }
  }
}

impl warp::Reply for Attrs {
  fn into_response(self) -> warp::reply::Response {
    warp::reply::Response::new(json!(self.attrs).to_string().into())
  }
}
