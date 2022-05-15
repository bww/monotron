use crate::update;
use crate::update::error;

pub struct Driver {
  version: usize
}

impl Driver {
  pub fn new(version: usize) -> Driver {
    Driver{
      version: version,
    }
  }
}

impl update::Driver for Driver {
  fn version(&self) -> Result<usize, error::Error> {
    Ok(self.version)
  }
}
