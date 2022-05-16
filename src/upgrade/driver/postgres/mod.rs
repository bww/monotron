use core::ops::DerefMut;

use bb8;
use tokio_postgres;
use futures::{pin_mut, TryStreamExt};

use tokio::task;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::runtime;

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
    let version = self.handle.block_on(move || {
      let client = match self.pool.get().await {
        Ok(client) => client,
        Err(err) => return Err(error::Error::DriverError),
      };
      let res = match client.query_one(
        "SELECT 1;",
        &[]
      ).await {
        Ok(res) => res,
        Err(err) => return Err(error::Error::DriverError),
      };
      match res.try_get(0) {
        Ok(version) => version,
        Err(err) => return Err(error::Error::DriverError),
      }
    });
    Ok(version as usize)
  }
  
  fn apply(&self, version: version::Version<R>) -> Result<(), error::Error> {
    Ok(())
  }
}
