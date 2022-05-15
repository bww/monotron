use bb8;
use tokio_postgres;
use futures::{pin_mut, TryStreamExt};
use tokio::io::{AsyncRead, AsyncWrite};

use crate::upgrade;
use crate::upgrade::error;
use crate::upgrade::version;

pub struct Driver<'a, M: bb8::ManageConnection> {
  conn: bb8::PooledConnection<'a, M>,
}

impl<'a, M> Driver<'a, M>
where
  M: bb8::ManageConnection,
{
  pub fn new(conn: bb8::PooledConnection<'a, M>) -> Driver<'a, M> {
    Driver{
      conn: conn,
    }
  }
}

impl<'a, M, R> upgrade::Driver<R> for Driver<'a, M>
where
  M: bb8::ManageConnection,
  R: upgrade::io::IntoRead,
{
  fn version(&self) -> Result<usize, error::Error> {
    Ok(0)
  }
  
  fn apply(&self, version: version::Version<R>) -> Result<(), error::Error> {
    Ok(())
  }
}
