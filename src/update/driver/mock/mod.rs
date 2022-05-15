use crate::update;
use crate::update::error;

pub struct Driver {
  version: usize,
  error: Option<error::Error>,
}

impl Driver {
  pub fn new(version: usize) -> Driver {
    Driver{
      version: version,
      error: None,
    }
  }
  
  pub fn new_with_error(version: usize, err: error::Error) -> Driver {
    Driver{
      version: version,
      error: Some(err),
    }
  }
}

impl update::Driver for Driver {
  fn version(&self) -> Result<usize, error::Error> {
    Ok(self.version)
  }
}
