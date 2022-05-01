pub mod error;

use bb8_postgres;
use tokio_postgres;
use futures::{pin_mut, TryStreamExt};

use crate::model::entry;
use crate::model::apikey;

const MAX_RESULTS: usize = 500;

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
         scopes     VARCHAR(64)[] NOT NULL,
         PRIMARY KEY (account_id, api_key_id)
      )",
      &[]
    )
    .await?;
    
    client.execute( // this is the implied system account
      "INSERT INTO mn_account (id) VALUES (0)
       ON CONFLICT (id) DO NOTHING",
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
    
    client.execute(
      "CREATE TABLE IF NOT EXISTS mn_entry_version (
         key        VARCHAR(256) NOT NULL,
         creator_id BIGINT NOT NULL REFERENCES mn_account (id),
         token      VARCHAR(256), -- not nullable in version history
         value      BIGINT NOT NULL,
         created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
         updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
         PRIMARY KEY (key, creator_id, token)
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
  
  pub async fn store_authorization(&self, auth: &apikey::Authorization) -> Result<apikey::Authorization, error::Error> {
    let mut client = self.pool.get().await?;
    let tx = client.transaction().await?;
    
    let account_id = match auth.account_id {
      Some(account_id) => account_id,
      None => return Err(error::Error::MarshalError),
    };
    
    let api_key_id: i64 = match tx.query_one(
      "INSERT INTO mn_api_key (key, secret) VALUES ($1, $2) RETURNING id",
      &[
        &auth.api_key.key,
        &auth.api_key.secret,
      ]
    ).await {
      Ok(row) => row.try_get(0)?,
      Err(err) => return Err(err.into()),
    };
    
    tx.execute("
      INSERT INTO mn_account_r_api_key (account_id, api_key_id, scopes) VALUES ($1, $2, $3)
      ON CONFLICT (account_id, api_key_id) DO UPDATE SET scopes = $3",
      &[
        &account_id,
        &api_key_id,
        &auth.scopes.scopes(),
      ]
    ).await?;
    
    tx.commit().await?;
    Ok(apikey::Authorization{
      account_id: Some(account_id),
      scopes: auth.scopes.clone(),
      api_key: auth.api_key.with_id(api_key_id),
    })
  }
  
  pub async fn delete_authorization(&self, account_id: i64, key: String) -> Result<(), error::Error> {
    let mut client = self.pool.get().await?;
    let tx = client.transaction().await?;
    
    let api_key = self.fetch_api_key_for_account(account_id, key).await?;
    
    tx.execute("
      DELETE FROM mn_account_r_api_key WHERE account_id = $1 AND api_key_id = $2",
      &[
        &account_id,
        &api_key.id,
      ]
    ).await?;
    
    tx.execute("
      DELETE FROM mn_api_key AS k WHERE k.id = $1 AND (
        SELECT COUNT(*) FROM mn_account_r_api_key AS r
        WHERE r.api_key_id = $1
      ) = 0",
      &[
        &api_key.id,
      ]
    ).await?;
    
    tx.commit().await?;
    Ok(())
  }
  
  pub async fn verify_authorization(&self, key: String, secret: String) -> Result<apikey::Authorization, error::Error> {
    let client = self.pool.get().await?;
    
    // NOTE: this is modeled as M:N, but we expect a single result and always
    // return the first record. we should reevalute this handling at some point.
    let stream = client.query_raw("
      SELECT k.id, k.key, k.secret, r.account_id, r.scopes FROM mn_api_key AS k
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
  
  pub async fn fetch_every_authorization_for_account(&self, account_id: i64) -> Result<Vec<apikey::Authorization>, error::Error> {
    let client = self.pool.get().await?;
    
    let rows = client.query("
      SELECT k.id, k.key, k.secret, r.account_id, r.scopes FROM mn_api_key AS k
      INNER JOIN mn_account_r_api_key AS r ON r.api_key_id = k.id
      WHERE r.account_id = $1
      ORDER BY k.created_at
      LIMIT $2",
      &[
        &account_id,
        &(MAX_RESULTS as i64),
      ]
    )
    .await?;
    
    let mut res: Vec<apikey::Authorization> = Vec::new();
    for row in rows {
      res.push(apikey::Authorization::unmarshal(&row)?);
    }
    
    Ok(res)
  }
  
  pub async fn fetch_authorization_for_account(&self, account_id: i64, key: String) -> Result<apikey::Authorization, error::Error> {
    let client = self.pool.get().await?;
    
    let stream = client.query_raw("
      SELECT k.id, k.key, k.secret, r.account_id, r.scopes FROM mn_api_key AS k
      INNER JOIN mn_account_r_api_key AS r ON r.api_key_id = k.id
      WHERE r.account_id = $1 AND k.key = $2",
      slice_iter(&[
        &account_id,
        &key,
      ])
    )
    .await?;
    pin_mut!(stream);
    
    match stream.try_next().await? {
      Some(row) => Ok(apikey::Authorization::unmarshal(&row)?),
      None => Err(error::Error::NotFoundError),
    }
  }
  
  async fn fetch_api_key_for_account(&self, account_id: i64, key: String) -> Result<apikey::ApiKey, error::Error> {
    let client = self.pool.get().await?;
    
    let stream = client.query_raw("
      SELECT k.id, k.key, k.secret FROM mn_api_key AS k
      INNER JOIN mn_account_r_api_key AS r ON r.api_key_id = k.id
      WHERE k.key = $1 AND r.account_id = $2",
      slice_iter(&[
        &key,
        &account_id,
      ])
    )
    .await?;
    pin_mut!(stream);
    
    match stream.try_next().await? {
      Some(row) => Ok(apikey::ApiKey::unmarshal(&row)?),
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
    
    if let Some(token) = &ent.token {
      client.execute("
        INSERT INTO mn_entry_version (key, creator_id, token, value) VALUES ($1, $2, $3, $4)
        ON CONFLICT (key, creator_id, token) DO UPDATE SET value = $4, updated_at = now()",
        &[
          &ent.key, &ent.creator_id, &token, &ent.value,
        ]
      )
      .await?;
    }
    
    Ok(())
  }
  
  pub async fn fetch_entry(&self, auth: &apikey::Authorization, key: String) -> Result<entry::Entry, error::Error> {
    let client = self.pool.get().await?;
    
    let stream = client.query_raw("
      SELECT key, creator_id, token, value FROM mn_entry
      WHERE key = $1 AND creator_id = $2",
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
  
  pub async fn fetch_entry_version(&self, auth: &apikey::Authorization, key: String, token: String) -> Result<entry::Entry, error::Error> {
    let client = self.pool.get().await?;
    
    let stream = client.query_raw("
      SELECT key, creator_id, token, value FROM mn_entry_version
      WHERE key = $1 AND creator_id = $2 AND token = $3",
      slice_iter(&[
        &key,
        &auth.account_id,
        &token,
      ])
    )
    .await?;
    pin_mut!(stream);
    
    match stream.try_next().await? {
      Some(row) => Ok(entry::Entry::unmarshal(&row)?),
      None => Err(error::Error::NotFoundError),
    }
  }
  
  pub async fn delete_entry(&self, auth: &apikey::Authorization, key: String) -> Result<(), error::Error> {
    let mut client = self.pool.get().await?;
    let tx = client.transaction().await?;
    
    tx.execute("DELETE FROM mn_entry WHERE key = $1 AND creator_id = $2", &[&key, &auth.account_id]).await?;
    tx.execute("DELETE FROM mn_entry_version WHERE key = $1 AND creator_id = $2", &[&key, &auth.account_id]).await?;
    
    tx.commit().await?;
    Ok(())
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
    
    if let Some(token) = &token {
      tx.execute("
        INSERT INTO mn_entry_version (key, creator_id, token, value) VALUES ($1, $2, $3, $4)
        ON CONFLICT (key, creator_id, token) DO UPDATE SET value = $4, updated_at = now()",
        &[
          &key,
          &auth.account_id,
          &token,
          &update.value,
        ]
      )
      .await?;
    }
    
    tx.commit().await?;
    Ok(update)
  }
  
}

fn slice_iter<'a>(
    s: &'a [&'a (dyn tokio_postgres::types::ToSql + Sync)],
) -> impl ExactSizeIterator<Item = &'a dyn tokio_postgres::types::ToSql> + 'a {
    s.iter().map(|s| *s as _)
}