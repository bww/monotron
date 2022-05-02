use crate::store;

use warp;
use serde::{Serialize, Deserialize};
use serde_json::json;
use tokio_postgres;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Account {
  pub id: i64,
}

impl Account {
  pub fn unmarshal(row: &tokio_postgres::Row) -> Result<Account, store::error::Error> {
    Ok(Account{
      id: row.try_get(0)?,
    })
  }
}

impl warp::Reply for Account {
  fn into_response(self) -> warp::reply::Response {
    warp::reply::Response::new(json!(self).to_string().into())
  }
}
