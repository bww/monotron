pub mod error;

use std::io;
use std::fs;
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

// pub trait Version: io::Read {
//   fn version(&self) -> usize;
// }

pub trait VersionProvider: iter::IntoIterator {}

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
  type Item = Result<path::PathBuf, error::Error>;
  fn next(&mut self) -> Option<Self::Item> {
    if let Some(e) = self.iter.next() {
      match e {
        Ok(e) => Some(Ok(path::PathBuf::from(e.path()))),
        Err(err) => Some(Err(err.into())),
      }
    }else{
      None
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn provide_versions() {
    let p = DirectoryProvider::new_with_path(".").unwrap();
    for v in p {
      println!(">>> {:?}", v);
    }
  }
  
}
