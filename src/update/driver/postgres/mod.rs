use tokio_postgres;
use futures::{pin_mut, TryStreamExt};

use crate::update;
use crate::update::error;

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

impl update::Driver for Driver {
  fn version(&self) -> Result<usize, error::Error> {
    Ok(0)
  }
}
