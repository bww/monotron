use std::io;
use std::fs;
use std::cmp;
use std::iter;
use std::path;

use crate::update::error;
use crate::update::io::{IntoRead, FileIntoRead};

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

impl Version<FileIntoRead> {
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

pub trait Provider<R: IntoRead> {
  fn versions(&self) -> Result<Vec<Version<R>>, error::Error>;
}

pub struct DirectoryProvider {
  path: path::PathBuf,
  iter: fs::ReadDir,
}

impl DirectoryProvider {
  pub fn new_with_path<P: AsRef<path::Path>>(path: P) -> Result<Self, error::Error> {
    Ok(Self{
      path: path::PathBuf::from(path.as_ref()),
      iter: fs::read_dir(path)?,
    })
  }
  
  fn load_versions<P: AsRef<path::Path>>(path: P) -> Result<Vec<Version<FileIntoRead>>, error::Error> {
    let mut ver: Vec<Version<FileIntoRead>> = Vec::new();
    for e in fs::read_dir(path)? {
      match e {
        Ok(e) => match Version::from_entry(e) {
          Ok(v) => if let Some(v) = v { ver.push(v); },
          Err(err) => return Err(err.into()),
        },
        Err(err) => return Err(err.into()),
      }
    }
    ver.sort();
    Ok(ver)
  }
}

impl Provider<FileIntoRead> for DirectoryProvider {
  fn versions(&self) -> Result<Vec<Version<FileIntoRead>>, error::Error> {
    Ok(DirectoryProvider::load_versions(&self.path)?)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn provide_versions() {
    let p = DirectoryProvider::new_with_path("./etc/db").unwrap();
    for v in p.versions().unwrap() {
      println!("### {:?}", v);
      let mut r = match v.into_read() {
        Ok(r) => r,
        Err(err) => panic!("*** {}", err),
      };
      match io::copy(&mut r, &mut io::stdout()) {
        Ok(n) => println!(">>> {} bytes", n),
        Err(err) => panic!("*** {}", err),
      };
    }
    
  }
  
}
