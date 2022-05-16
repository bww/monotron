use core::ops::DerefMut;

use bb8;
use tokio_postgres;
use futures::{pin_mut, TryStreamExt};

use tokio::task;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::runtime;
use crossbeam_channel;

use crate::upgrade;
use crate::upgrade::error;
use crate::upgrade::version;

const VERSION_TABLE: &str = "schema_version";

pub struct Driver {
  handle: runtime::Handle,
  pool: bb8::Pool<bb8_postgres::PostgresConnectionManager<tokio_postgres::NoTls>>,
}

impl Driver {
  pub fn new(handle: runtime::Handle, pool: bb8::Pool<bb8_postgres::PostgresConnectionManager<tokio_postgres::NoTls>>) -> Driver {
    Driver{
      handle: handle,
      pool: pool,
    }
  }
}

impl Driver {
  async fn create_version_table(pool: bb8::Pool<bb8_postgres::PostgresConnectionManager<tokio_postgres::NoTls>>, table: &str) -> Result<(), error::Error> {
    let client = match pool.get().await {
      Ok(client) => client,
      Err(err) => return Err(error::Error::DriverError("Could not create client".to_string())),
    };
    let res = match client.execute(
      &format!("CREATE TABLE IF NOT EXISTS {} (version BIGINT NOT NULL PRIMARY KEY)", table),
      &[]
    ).await {
      Ok(res) => res,
      Err(err) => return Err(error::Error::DriverError("Could not create client".to_string())),
    };
    Ok(())
  }

  async fn current_version(pool: bb8::Pool<bb8_postgres::PostgresConnectionManager<tokio_postgres::NoTls>>, table: &str) -> Result<usize, error::Error> {
    let client = match pool.get().await {
      Ok(client) => client,
      Err(err) => return Err(error::Error::DriverError("Could not create client".to_string())),
    };
    
    let res = match client.query_one(
      &format!("SELECT COALESCE(MAX(version), 0::BIGINT) FROM {}", VERSION_TABLE),
      &[]
    ).await {
      Ok(res) => res,
      Err(err) => return Err(error::Error::DriverError("Could not query version".to_string())),
    };
    
    let version: i64 = match res.try_get(0) {
      Ok(version) => version,
      Err(err) => return Err(error::Error::DriverError(format!("Could not read result: {}", err))),
    };
    
    Ok(version as usize)
  }

  async fn apply_version(pool: bb8::Pool<bb8_postgres::PostgresConnectionManager<tokio_postgres::NoTls>>, table: &str, version: version::Version<upgrade::io::FileIntoRead>) -> Result<(), error::Error> {
    let reader = version.into_read()?;
    let mut sql = String::new();
    reader.read_to_string(sql)?;
    
    println!(">>> BUF: {}", sql);
    
    // let client = match pool.get().await {
    //   Ok(client) => client,
    //   Err(err) => return Err(error::Error::DriverError("Could not create client".to_string())),
    // };
    
    // let res = match client.query_one(
    //   &format!("SELECT COALESCE(MAX(version), 0::BIGINT) FROM {}", VERSION_TABLE),
    //   &[]
    // ).await {
    //   Ok(res) => res,
    //   Err(err) => return Err(error::Error::DriverError("Could not query version".to_string())),
    // };
    
    // let version: i64 = match res.try_get(0) {
    //   Ok(version) => version,
    //   Err(err) => return Err(error::Error::DriverError(format!("Could not read result: {}", err))),
    // };
    
    Ok(())
  }
}

impl upgrade::Driver<upgrade::io::FileIntoRead> for Driver {
  fn version(&self) -> Result<usize, error::Error> {
    let (tx, rx) = crossbeam_channel::bounded(1);
    let pool = self.pool.clone();
    self.handle.spawn(async move {
      if let Err(err) = Driver::create_version_table(pool.clone(), VERSION_TABLE).await {
        tx.send(Err(err)).unwrap();
        return;
      }
      tx.send(Driver::current_version(pool.clone(), VERSION_TABLE).await).unwrap();
    });
    Ok(rx.recv()??)
  }
  
  fn apply(&self, version: version::Version<upgrade::io::FileIntoRead>) -> Result<(), error::Error> {
    let (tx, rx) = crossbeam_channel::bounded(1);
    let pool = self.pool.clone();
    self.handle.spawn(async move {
      tx.send(Driver::apply_version(pool.clone(), VERSION_TABLE, version).await).unwrap();
    });
    Ok(rx.recv()??)
  }
}
