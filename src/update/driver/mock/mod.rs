use crate::update;
use crate::update::error;
use crate::update::version;

pub struct Driver {
  version: usize,
  error: bool,
}

impl Driver {
  pub fn new(version: usize, error: bool) -> Driver {
    Driver{
      version: version,
      error: error,
    }
  }
}

impl<R: update::io::IntoRead> update::Driver<R> for Driver {
  fn version(&self) -> Result<usize, error::Error> {
    Ok(self.version)
  }
  
  fn apply(&self, version: version::Version<R>) -> Result<(), error::Error> {
    if self.error {
      Err(error::Error::DriverError)
    }else{
      Ok(())
    }
  }
}
