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

pub trait IntoRead<R: io::Read> {
  fn into_read(&self) -> Result<R, error::Error>;
}

pub struct FileIntoRead {
  path: path::PathBuf,
}

impl IntoRead<fs::File> for FileIntoRead {
  fn into_read(&self) -> Result<fs::File, error::Error> {
    Ok(fs::File::open(&self.path)?)
  }
}

#[derive(Debug, Clone)]
pub struct Version<T: io::Read, R: IntoRead<T>> {
  version: usize,
  descr: String,
  reader: R,
  dummy: Option<T>,
}

impl<T: io::Read, R: IntoRead<T>> Version<T, R> {
  pub fn new(version: usize, descr: String, reader: R) -> Version<T, R> {
    Version{
      version: version,
      descr: descr,
      reader: reader,
      dummy: None,
    }
  }
}

impl<T: io::Read, R: IntoRead<T>> IntoRead<T> for Version<T, R> {
  fn into_read(&self) -> Result<T, error::Error> {
    self.reader.into_read()
  }
}

/*
impl<R: io::Read> io::Read for Version<R> {
  fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
    self.resource.read(buf)
  }
}
*/

impl<T: io::Read, R: IntoRead<T>> cmp::PartialOrd for Version<T, R> {
  fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
    self.version.partial_cmp(&other.version)
  }
}

impl<T: io::Read, R: IntoRead<T>> cmp::Ord for Version<T, R> {
  fn cmp(&self, other: &Self) -> cmp::Ordering {
    self.version.cmp(&other.version)
  }
}

impl<T: io::Read, R: IntoRead<T>> cmp::Eq for Version<T, R> {}
impl<T: io::Read, R: IntoRead<T>> cmp::PartialEq for Version<T, R> {
  fn eq(&self, other: &Self) -> bool {
    self.version == other.version
  }
}

impl Version<fs::File, FileIntoRead> {
  pub fn from_entry(entry: fs::DirEntry) -> Result<Option<Version<fs::File, FileIntoRead>>, error::Error> {
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
        dummy: None,
      })
    )
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
  type Item = Result<Version<fs::File, FileIntoRead>, error::Error>;
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
    
    let mut versions: Vec<Version<fs::File, IntoRead<fs::File>>> = Vec::new();
    for v in iter {
      match v {
        Ok(v) => versions.push(v),
        Err(err) => panic!("*** {}", err),
      };
    }
    
    versions.sort();
    for mut v in versions {
      println!("### {:?}", v);
      match io::copy(&mut v.into_read()?, &mut io::stdout()) {
        Ok(n) => println!(">>> {} bytes", n),
        Err(err) => panic!("*** {}", err),
      };
    }
    
  }
  
}
