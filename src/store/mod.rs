pub mod error;

use tokio_postgres;

#[derive(Debug)]
pub struct Store {
  client: tokio_postgres::Client,
}

impl Store {
  
  pub async fn new(host: &str) -> Result<Store, error::Error> {
    let (client, conn) = tokio_postgres::connect(&format!("host={} user=postgres", host), tokio_postgres::NoTls).await?;
    
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
    self.client
      .execute(
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
    
    self.client
      .execute(
        "CREATE TABLE IF NOT EXISTS version (
           id         SERIAL PRIMARY KEY,
           api_key_id BIGINT NOT NULL REFERENCES api_key (id),
           type       VARCHAR(64) NOT NULL,
           token      VARCHAR(256), -- nullable
           value      VARCHAR(256) NOT NULL,
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
  
}
