use tokio_postgres;
use futures::{pin_mut, TryStreamExt};
use tokio::io::{AsyncRead, AsyncWrite};

use crate::upgrade;
use crate::upgrade::error;
use crate::upgrade::version;

pub struct Driver<S, T> {
  conn: tokio_postgres::Connection<S, T>,
}

impl<S, T> Driver<S, T>
where
  S: AsyncRead + AsyncWrite + Unpin,
  T: AsyncRead + AsyncWrite + Unpin,
{
  pub fn new(conn: tokio_postgres::Connection<S, T>) -> Driver<S, T> {
    Driver{
      conn: conn,
    }
  }
}

impl<S, T, R> upgrade::Driver<R> for Driver<S, T>
where
  S: AsyncRead + AsyncWrite + Unpin,
  T: AsyncRead + AsyncWrite + Unpin,
  R: upgrade::io::IntoRead,
{
  fn version(&self) -> Result<usize, error::Error> {
    Ok(0)
  }
  
  fn apply(&self, version: version::Version<R>) -> Result<(), error::Error> {
    Ok(())
  }
}
