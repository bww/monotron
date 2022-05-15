pub mod error;

use std::io;
use std::fs;
use std::cmp;
use std::iter;
use std::path;

// pub trait Driver {
//   fn version(&self) -> Result<usize, error::Error>;
// }

// pub struct Updater<D: Driver> {
//   driver: D,
// }

// impl<D> Updater<D> {
//   pub fn new(driver: D) -> Self {
//     Self{
//       driver: driver,
//     }
//   }
// }

pub trait IntoRead {
  type Read;
  fn into_read(&self) -> Result<Self::Read, error::Error>;
}

#[derive(Debug, Clone)]
pub struct FileIntoRead {
  path: path::PathBuf,
}

impl IntoRead for FileIntoRead {
  type Read = fs::File;
  fn into_read(&self) -> Result<fs::File, error::Error> {
    Ok(fs::File::open(&self.path)?)
  }
}

#[derive(Debug, Clone)]
pub struct Version<R: IntoRead> {
  version: usize,
  descr: String,
  reader: R,
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
        reader: FileIntoRead{
          path: path::PathBuf::from(path),
        },
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

pub trait VersionProvider: iter::Iterator {}

pub struct DirectoryProvider {
  dir: path::PathBuf,
  iter: fs::ReadDir,
}

impl DirectoryProvider {
  pub fn new_with_path<P: AsRef<path::Path>>(path: P) -> Result<Self, error::Error> {
    Ok(Self{
      dir: path::PathBuf::from(path.as_ref()),
      iter: fs::read_dir(path)?,
    })
  }
}

impl iter::Iterator for DirectoryProvider {
  type Item = Result<Version<FileIntoRead>, error::Error>;
  fn next(&mut self) -> Option<Self::Item> {
    loop {
      if let Some(e) = self.iter.next() {
        match e {
          Ok(e) => match Version::from_entry(e) {
            Ok(v) => if let Some(v) = v {
              return Some(Ok(v));
            },
            Err(err) => return Some(Err(err.into())),
          },
          Err(err) => return Some(Err(err.into())),
        }
      }else{
        return None;
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn provide_versions() {
    let iter = DirectoryProvider::new_with_path("./etc/db").unwrap();
    
    let mut versions: Vec<Version<FileIntoRead>> = Vec::new();
    for v in iter {
      match v {
        Ok(v) => versions.push(v),
        Err(err) => panic!("*** {}", err),
      };
    }
    
    versions.sort();
    for v in versions {
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
