use std::fs;
use std::path;

use bytes;

use crate::update::error;

pub trait IntoRead: Clone {
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

#[derive(Debug, Clone)]
pub struct BytesIntoRead {
  data: bytes::Bytes,
}

impl BytesIntoRead {
  pub fn new(data: bytes::Bytes) -> BytesIntoRead {
    BytesIntoRead{
      data: data,
    }
  }
}

impl IntoRead for BytesIntoRead {
  type Read = bytes::Bytes;
  fn into_read(&self) -> Result<Self::Read, error::Error> {
    Ok(self.data.to_owned())
  }
}

