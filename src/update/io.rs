use std::fs;
use std::path;

use crate::update::error;

pub trait IntoRead {
  type Read;
  fn into_read(&self) -> Result<Self::Read, error::Error>;
}

#[derive(Debug, Clone)]
pub struct FileIntoRead {
  path: path::PathBuf,
}

impl FileIntoRead {
  pub fn new<P: AsRef<path::Path>>(path: P) -> FileIntoRead {
    FileIntoRead{
      path: path::PathBuf::from(path.as_ref()),
    }
  }
}

impl IntoRead for FileIntoRead {
  type Read = fs::File;
  fn into_read(&self) -> Result<Self::Read, error::Error> {
    Ok(fs::File::open(&self.path)?)
  }
}

