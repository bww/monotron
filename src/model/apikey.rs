use crate::store;

use rand::{self, Rng};
use tokio_postgres;

fn random_string(len: usize) -> String {
  rand::thread_rng()
    .sample_iter(&rand::distributions::Alphanumeric)
    .take(len)
    .map(char::from)
    .collect()
}

pub fn gen_api_key(len: usize) -> (String, String) {
  (random_string(24), random_string(128))
}

#[derive(Debug, Clone, PartialEq)]
pub struct ApiKey {
  pub id: i64,
  pub key: String,
  pub secret: String,
}

impl ApiKey {
  pub fn unmarshal(row: &tokio_postgres::Row) -> Result<ApiKey, store::error::Error> {
    Ok(ApiKey{
      id: row.try_get(0)?,
      key: row.try_get(1)?,
      secret: row.try_get(1)?,
    })
  }
}
