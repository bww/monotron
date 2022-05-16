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

impl<R: upgrade::io::IntoRead> upgrade::Driver<R> for Driver {
  fn version(&self) -> Result<usize, error::Error> {
    let (tx, rx) = crossbeam_channel::bounded(1);
    let pool = self.pool.clone();
    self.handle.spawn(async move {
      let client = match pool.get().await {
        Ok(client) => client,
        Err(err) => {
          tx.send(Err(error::Error::DriverError("Could not create client".to_string()))).unwrap();
          return;
        },
      };
      let res = match client.query_one(
        "SELECT 1::BIGINT;",
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
