pub mod provider;

use std::io;
use std::fs;
use std::cmp;
use std::iter;
use std::path;

use crate::update::error;
use crate::update::io::{IntoRead, FileIntoRead, BytesIntoRead};

#[derive(Debug, Clone)]
pub struct Version<R: IntoRead> {
  version: usize,
  descr: String,
  reader: R,
}

impl<R: IntoRead> Version<R> {
  pub fn version(&self) -> usize {
    self.version
  }
}

impl<R: IntoRead> cmp::PartialOrd for Version<R> {
  fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
    self.version.partial_cmp(&other.version)
  }
}

impl<R: IntoRead> cmp::Ord for Version<R> {
  fn cmp(&self, other: &Self) -> cmp::Ordering {
    self.version.cmp(&other.version)
  }
}

impl<R: IntoRead> cmp::Eq for Version<R> {}
impl<R: IntoRead> cmp::PartialEq for Version<R> {
  fn eq(&self, other: &Self) -> bool {
    self.version == other.version
  }
}

impl Version<BytesIntoRead> {
  pub fn new_with_bytes(version: usize, data: bytes::Bytes) -> Result<Version<BytesIntoRead>, error::Error> {
    Ok(Version{
      version: version,
      descr: format!("{}B", data.len()),
      reader: BytesIntoRead::new(data),
    })
  }
}

impl Version<FileIntoRead> {
  pub fn new_with_path<P: AsRef<path::Path>>(version: usize, path: P) -> Result<Version<FileIntoRead>, error::Error> {
    let name = match path.as_ref().file_name() {
      Some(name) => name,
      None => return Err(error::Error::VersionError(format!("No file name"))),
    };
    let name = match name.to_str() {
      Some(name) => name,
      None => return Err(error::Error::VersionError(format!("Could not convert file name"))),
    };
    Ok(Version{
      version: version,
      descr: name.to_string(),
      reader: FileIntoRead::new(path),
    })
  }
  
  pub fn from_entry(entry: fs::DirEntry) -> Result<Option<Version<FileIntoRead>>, error::Error> {
    let m = entry.metadata()?;
    if !m.is_file() {
      return Ok(None);
    }
    
    let path = entry.path();
    let ext = match path.extension() {
      Some(ext) => ext,
      None => return Ok(None),
    };
    if let Some(ext) = ext.to_str() {
      if ext != "sql" {
        return Ok(None);
      }
    }
    
    let name = match path.file_name() {
      Some(name) => name,
      None => return Ok(None),
    };
    let name = match name.to_str() {
      Some(name) => name,
      None => return Ok(None),
    };
    let chars = name.chars();
    let mut buf = String::new();
    for c in chars {
      if char::is_numeric(c) {
        buf.push(c);
      }else{
        break;
      }
    }
    if buf.len() == 0 {
      return Ok(None)
    }
    
    let v = match buf.parse::<usize>() {
      Ok(v) => v,
      Err(err) => return Err(err.into()),
    };
    
    Ok(
      Some(Version{
        version: v,
        descr: name.to_string(),
        reader: FileIntoRead::new(&path),
      })
    )
  }
}

impl IntoRead for Version<FileIntoRead> {
  type Read = fs::File;
  fn into_read(&self) -> Result<Self::Read, error::Error> {
    self.reader.into_read()
  }
}
