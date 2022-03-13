pub mod error;

use bb8_postgres;
use tokio_postgres;
use futures::{pin_mut, TryStreamExt};

use crate::model::entry;
use crate::model::apikey;

#[derive(Debug, Clone)]
pub struct Store {
  pool: bb8::Pool<bb8_postgres::PostgresConnectionManager<tokio_postgres::NoTls>>,
}

impl Store {
  
  pub async fn new(dsn: &str) -> Result<Store, error::Error> {
    let config: tokio_postgres::config::Config = str::parse(dsn)?;
    let manager = bb8_postgres::PostgresConnectionManager::new(config, tokio_postgres::NoTls);
    let pool = bb8::Pool::builder()
      .max_size(15)
      .build(manager)
      .await?;
    
    let store = Store{pool};
    store.init().await?;
    
    return Ok(store);
  }
  
  async fn init(&self) -> Result<(), error::Error> {
    let client = self.pool.get().await?;
    
    client.execute(
      "CREATE TABLE IF NOT EXISTS mn_account (
         id         BIGSERIAL PRIMARY KEY,
         created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
         updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now()
      )",
      &[]
    )
    .await?;
    
    client.execute(
      "CREATE TABLE IF NOT EXISTS mn_api_key (
         id         BIGSERIAL PRIMARY KEY,
         key        VARCHAR(256) NOT NULL UNIQUE,
         secret     VARCHAR(1024) NOT NULL,
         scopes     VARCHAR(64)[],
         created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
         updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now()
      )",
      &[]
    )
    .await?;
    
    client.execute(
      "CREATE TABLE IF NOT EXISTS mn_account_r_api_key (
         account_id BIGINT NOT NULL REFERENCES mn_account (id),
         api_key_id BIGINT NOT NULL REFERENCES mn_api_key (id),
         PRIMARY KEY (account_id, api_key_id)
      )",
      &[]
    )
    .await?;
    
    client.execute(
      "INSERT INTO mn_api_key (id, key, secret, scopes) VALUES (1, 'bootstrap', 'ztLvoY6IKyxA', ARRAY['*:system'])
       ON CONFLICT (id) DO NOTHING",
      &[]
    )
    .await?;
    
    client.execute(
      "INSERT INTO mn_account (id) VALUES (1)
       ON CONFLICT (id) DO NOTHING",
      &[]
    )
    .await?;
    
    client.execute(
      "INSERT INTO mn_account_r_api_key (account_id, api_key_id) VALUES (1, 1)
       ON CONFLICT (account_id, api_key_id) DO NOTHING",
      &[]
    )
    .await?;
    
    client.execute(
      "CREATE TABLE IF NOT EXISTS mn_entry (
         key        VARCHAR(256) NOT NULL,
         creator_id BIGINT NOT NULL REFERENCES mn_account (id),
         token      VARCHAR(256), -- nullable
         value      BIGINT NOT NULL,
         created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
         updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
         PRIMARY KEY (key, creator_id)
      )",
      &[]
    )
    .await?;
    
    Ok(())
  }
  
  pub async fn _check(&self) -> Result<String, error::Error> {
    let client = self.pool.get().await?;
    let row = client
      .query_one("SELECT $1::TEXT", &[&"Check"])
      .await?;
    let val: String = row.get(0);
    Ok(val)
  }
  
  pub async fn fetch_authorization(&self, key: String, secret: String) -> Result<apikey::Authorization, error::Error> {
    let mut client = self.pool.get().await?;
    
    let stream = client.query_raw("
      SELECT k.id, k.key, k.secret, k.scopes, r.account_id FROM mn_api_key AS k
      INNER JOIN mn_account_r_api_key AS r ON r.api_key_id = k.id
      WHERE k.key = $1 AND k.secret = $2",
      slice_iter(&[
        &key,
        &secret,
      ])
    )
    .await?;
    pin_mut!(stream);
    
    match stream.try_next().await? {
      Some(row) => Ok(apikey::Authorization::unmarshal(&row)?),
      None => Err(error::Error::NotFoundError),
    }
  }
  
  pub async fn _store_entry(&self, ent: &entry::Entry) -> Result<(), error::Error> {
    let client = self.pool.get().await?;
    client.execute("
      INSERT INTO mn_entry (key, creator_id, token, value) VALUES ($1, $2, $3, $4)
      ON CONFLICT (key, creator_id) DO UPDATE SET token = $3, value = $4, updated_at = now()",
      &[
        &ent.key, &ent.creator_id, &ent.token, &ent.value,
      ]
    )
    .await?;
    Ok(())
  }
  
  pub async fn fetch_entry(&self, auth: &apikey::Authorization, key: String) -> Result<entry::Entry, error::Error> {
    let mut client = self.pool.get().await?;
    
    let stream = client.query_raw("
      SELECT key, creator_id, token, value FROM mn_entry
      WHERE key = $1 AND creator_id = $2
      FOR UPDATE",
      slice_iter(&[
        &key,
        &auth.account_id,
      ])
    )
    .await?;
    pin_mut!(stream);
    
    match stream.try_next().await? {
      Some(row) => Ok(entry::Entry::unmarshal(&row)?),
      None => Err(error::Error::NotFoundError),
    }
  }
  
  pub async fn inc_entry(&self, auth: &apikey::Authorization, key: String, token: Option<String>) -> Result<entry::Entry, error::Error> {
    let mut client = self.pool.get().await?;
    let tx = client.transaction().await?;
    
    let stream = tx.query_raw("
      SELECT key, creator_id, token, value FROM mn_entry
      WHERE key = $1 AND creator_id = $2
      FOR UPDATE",
      slice_iter(&[
        &key,
        &auth.account_id,
      ])
    )
    .await?;
    pin_mut!(stream);
    
    let entry = match stream.try_next().await? {
      Some(row) => entry::Entry::unmarshal(&row)?,
      None => entry::Entry::new(&key, 1, None, 0),
    };
    
    let update = if let Some(tok) = &token {
      if let Some(upd) = entry.next_with_token(tok) {
        upd
      }else{
        entry.clone()
      }
    }else{
      entry.next()
    };
    
    tx.execute("
      INSERT INTO mn_entry (key, creator_id, token, value) VALUES ($1, $2, $3, $4)
      ON CONFLICT (key, creator_id) DO UPDATE SET token = $3, value = $4",
      &[
        &key,
        &auth.account_id,
        &token,
        &update.value,
      ]
    )
    .await?;
    
    tx.commit().await?;
    Ok(update)
  }
  
}

fn slice_iter<'a>(
    s: &'a [&'a (dyn tokio_postgres::types::ToSql + Sync)],
) -> impl ExactSizeIterator<Item = &'a dyn tokio_postgres::types::ToSql> + 'a {
    s.iter().map(|s| *s as _)
}