pub mod error;

use tokio_postgres;

use crate::model::entry;

#[derive(Debug)]
pub struct Store {
  client: tokio_postgres::Client,
}

impl Store {
  
  pub async fn new(host: &str, db: &str) -> Result<Store, error::Error> {
    let (client, conn) = tokio_postgres::connect(&format!("postgresql://postgres@{}/{}", host, db), tokio_postgres::NoTls).await?;
    
    tokio::spawn(async move {
      if let Err(e) = conn.await {
        eprintln!("connection error: {}", e);
      }
    });
    
    let store = Store{client};
    store.init().await?;
    return Ok(store);
  }
  
  async fn init(&self) -> Result<(), error::Error> {
    self.client.execute(
      "CREATE TABLE IF NOT EXISTS api_key (
         id         SERIAL PRIMARY KEY,
         key        VARCHAR(256) NOT NULL UNIQUE,
         secret     VARCHAR(1024) NOT NULL,
         created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
         updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now()
      )",
      &[]
    )
    .await?;
    
    self.client.execute(
      "CREATE TABLE IF NOT EXISTS entry (
         key        VARCHAR(256) NOT NULL PRIMARY KEY,
         creator_id BIGINT NOT NULL REFERENCES api_key (id),
         token      VARCHAR(256), -- nullable
         value      BIGINT NOT NULL,
         created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
         updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now()
      )",
      &[]
    )
    .await?;
    
    Ok(())
  }
  
  pub async fn check(&self) -> Result<String, error::Error> {
    let row = self.client
      .query_one("SELECT $1::TEXT", &[&"Check"])
      .await?;
    let val: String = row.get(0);
    Ok(val)
  }
  
  pub async fn store_entry(&self, ent: &entry::Entry) -> Result<(), error::Error> {
    let row = self.client.execute("
      INSERT INTO entry (key, creator_id, token, value) VALUES ($1, $2, $3, $4)
      ON CONFLICT (key) DO UPDATE SET creator_id = $2, token = $3, value = $4, updated_at = now()",
      &[
        &ent.key, &ent.creator_id, &ent.token, &ent.value,
      ]
    )
    .await?;
    Ok(())
  }
  
}
