use tokio_postgres;
use futures::{pin_mut, TryStreamExt};

use crate::update;
use crate::update::error;
use crate::update::version;

pub struct Driver {
  // conn: tokio_postgres::Connection,
}

impl Driver {
  // pub fn new(conn: tokio_postgres::Connection) -> Driver {
  //   Driver{
  //     conn: conn,
  //   }
  // }
  pub fn new() -> Driver {
    Driver{
    }
  }
}

impl<R: update::io::IntoRead> update::Driver<R> for Driver {
  fn version(&self) -> Result<usize, error::Error> {
    Ok(0)
  }
  
  fn apply(&self, version: version::Version<R>) -> Result<(), error::Error> {
    Ok(())
  }
}
