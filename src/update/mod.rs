pub mod error;

use std::io;
use std::fs;
use std::iter;
use std::path;

pub trait Driver {
  fn version(&self) -> Result<usize, error::Error>;
}

pub struct Updater<D: Driver> {
  driver: D,
}

impl<D> Updater<D> {
  pub fn new(driver: D) -> Self {
    Self{
      driver: driver,
    }
  }
}

pub trait Version: io::Read {
  fn version(&self) -> usize;
}

pub trait VersionProvider: IntoIterator {}

pub struct DirectoryProvider<I: iter::Iterator<Item=io::Result<fs::DirEntry>>> {
  dir: path::Path,
  paths: I,
}

impl<I> DirectoryProvider<I> {
  pub fn new_with_path<P: AsRef<path::Path>>(path: P) -> Result<Self, error::Error> {
    Self{
      dir: path.to_owned(),
      paths: fs::read_dir(path)?,
    }
  }
}

impl<I> IntoIterator for DirectoryProvider<I> {
  type Item = Result<dyn Version, error::Error>;
  type IntoIter = Result<dyn Version, error::Error>;
  fn next(&mut self) -> Option<Self::Item> {
    let path = self.paths.next()?;
    println!(">>> {}", path);
    None
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn provide_versions() {
    let p = DirectoryProvider::new_with_path().unwrap();
    for v in p {
      println!(">>> {:?}", v);
    }
  }
  
}
