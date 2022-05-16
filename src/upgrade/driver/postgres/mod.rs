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
  async fn create_version_table(pool: bb8::Pool<bb8_postgres::PostgresConnectionManager<tokio_postgres::NoTls>>, name: &str) -> Result<(), error::Error> {
    let client = match pool.get().await {
      Ok(client) => client,
      Err(err) => return Err(error::Error::DriverError("Could not create client".to_string())),
    };
    let res = match client.execute(
      &format!("CREATE TABLE IF NOT EXISTS {} (version BIGINT NOT NULL PRIMARY KEY)", name),
      &[]
    ).await {
      Ok(res) => res,
      Err(err) => return Err(error::Error::DriverError("Could not create client".to_string())),
    };
    Ok(())
  }
}

impl<R: upgrade::io::IntoRead> upgrade::Driver<R> for Driver {
  fn version(&self) -> Result<usize, error::Error> {
    let (tx, rx) = crossbeam_channel::bounded(1);
    let pool = self.pool.clone();
    self.handle.spawn(async move {
      if let Err(err) = Driver::create_version_table(pool.clone(), VERSION_TABLE).await {
        tx.send(Err(err)).unwrap();
        return;
      }
      
      let client = match pool.get().await {
        Ok(client) => client,
        Err(err) => {
          tx.send(Err(error::Error::DriverError("Could not create client".to_string()))).unwrap();
          return;
        },
      };
      
      let res = match client.query_one(
        &format!("SELECT COALESCE(MAX(version), 0::BIGINT) FROM {}", VERSION_TABLE),
        &[]
      ).await {
        Ok(res) => res,
        Err(err) => {
          tx.send(Err(error::Error::DriverError("Could not query version".to_string()))).unwrap();
          return;
        },
      };
      
      tx.send(match res.try_get(0) {
        Ok(version) => Ok(version),
        Err(err) => Err(error::Error::DriverError(format!("Could not read result: {}", err))),
      }).unwrap();
    });
    let version: i64 = rx.recv()??;
    Ok(version as usize)
  }
  
  fn apply(&self, version: version::Version<R>) -> Result<(), error::Error> {
    Ok(())
  }
}
