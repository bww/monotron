use crate::update;
use crate::update::error;
use crate::update::version;

pub struct Driver {
  version: usize,
  error: Option<String>,
}

impl Driver {
  pub fn new(version: usize, error: Option<&str>) -> Driver {
    let error = match error {
      Some(error) => Some(error.to_owned()),
      None => None,
    };
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
    match &self.error {
      Some(msg) => Err(error::Error::UpgradeError(version.version(), version.description(), msg.to_owned())),
      None => Ok(()),
    }
  }
}
