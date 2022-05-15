use std::io;
use std::fs;
use std::path;

use crate::update::error;
use crate::update::version;
use crate::update::io::{IntoRead, FileIntoRead};

pub trait Provider<R: IntoRead> {
  fn versions(&self) -> Result<Vec<version::Version<R>>, error::Error>;
}

pub struct ConstantProvider<R: IntoRead> {
  versions: Vec<version::Version<R>>,
}

impl<R: IntoRead> ConstantProvider<R> {
  pub fn new(versions: Vec<version::Version<R>>) -> ConstantProvider<R> {
    ConstantProvider{
      versions: versions,
    }
  }
}

impl<R: IntoRead> Provider<R> for ConstantProvider<R> {
  fn versions(&self) -> Result<Vec<version::Version<R>>, error::Error> {
    Ok(self.versions.to_vec())
  }
}

pub struct DirectoryProvider {
  path: path::PathBuf,
}

impl DirectoryProvider {
  pub fn new_with_path<P: AsRef<path::Path>>(path: P) -> Result<Self, error::Error> {
    Ok(Self{
      path: path::PathBuf::from(path.as_ref()),
    })
  }
  
  fn load_versions<P: AsRef<path::Path>>(path: P) -> Result<Vec<version::Version<FileIntoRead>>, error::Error> {
    let mut ver: Vec<version::Version<FileIntoRead>> = Vec::new();
    for e in fs::read_dir(path)? {
      match e {
        Ok(e) => match version::Version::from_entry(e) {
          Ok(v) => if let Some(v) = v { ver.push(v); },
          Err(err) => return Err(err.into()),
        },
        Err(err) => return Err(err.into()),
      }
    }
    
    ver.sort();
    
    let mut prev: usize = 0;
    for v in &ver {
      let curr = v.version();
      prev += 1;
      if curr != prev {
        return Err(error::Error::SequenceError(prev, curr));
      }
    }
    
    Ok(ver)
  }
}

impl Provider<FileIntoRead> for DirectoryProvider {
  fn versions(&self) -> Result<Vec<version::Version<FileIntoRead>>, error::Error> {
    Ok(DirectoryProvider::load_versions(&self.path)?)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn directory_provider_001() {
    let p = DirectoryProvider::new_with_path("./test/fixture/migrate/001").unwrap();
    assert_eq!(Vec::<version::Version<FileIntoRead>>::new(), p.versions().unwrap());
  }
  
  #[test]
  fn directory_provider_002() {
    let p = DirectoryProvider::new_with_path("./test/fixture/migrate/002").unwrap();
    let v: Vec<version::Version<FileIntoRead>> = vec![
      version::Version::new_with_path(1, "./test/fixture/migrate/002/1_first.sql").unwrap(),
      version::Version::new_with_path(2, "./test/fixture/migrate/002/2_second.sql").unwrap(),
      version::Version::new_with_path(3, "./test/fixture/migrate/002/3_third.sql").unwrap(),
    ];
    assert_eq!(v, p.versions().unwrap());
  }
  
  #[test]
  fn directory_provider_003() {
    let p = DirectoryProvider::new_with_path("./test/fixture/migrate/003").unwrap();
    let err = match p.versions() {
      Ok(v) => panic!("Expected an error"),
      Err(err) => err,
    };
    assert_eq!(format!("{}", error::Error::SequenceError(2, 1)), format!("{}", err));
  }
  
  #[test]
  fn directory_provider_004() {
    let p = DirectoryProvider::new_with_path("./test/fixture/migrate/004").unwrap();
    let v: Vec<version::Version<FileIntoRead>> = vec![
      version::Version::new_with_path(1, "./test/fixture/migrate/002/001_first.sql").unwrap(),
      version::Version::new_with_path(2, "./test/fixture/migrate/002/002_second.sql").unwrap(),
      version::Version::new_with_path(3, "./test/fixture/migrate/002/003_third.sql").unwrap(),
    ];
    assert_eq!(v, p.versions().unwrap());
  }
  
  #[test]
  fn directory_provider_005() {
    let p = DirectoryProvider::new_with_path("./test/fixture/migrate/005").unwrap();
    let err = match p.versions() {
      Ok(v) => panic!("Expected an error"),
      Err(err) => err,
    };
    assert_eq!(format!("{}", error::Error::SequenceError(1, 2)), format!("{}", err));
  }
  
  #[test]
  fn directory_provider_006() {
    let p = DirectoryProvider::new_with_path("./test/fixture/migrate/006").unwrap();
    let err = match p.versions() {
      Ok(v) => panic!("Expected an error"),
      Err(err) => err,
    };
    assert_eq!(format!("{}", error::Error::SequenceError(3, 4)), format!("{}", err));
  }
  
}
