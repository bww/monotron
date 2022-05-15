pub mod io;
pub mod error;
pub mod version;
pub mod driver;

use crate::update;
use crate::update::driver::mock;

pub trait Driver {
  fn version(&self) -> Result<usize, error::Error>;
}

pub struct Updater<D: Driver, R: update::io::IntoRead> {
  driver: D,
  versions: Vec<version::Version<R>>,
}

impl<D: Driver, R: update::io::IntoRead> Updater<D, R> {
  pub fn new<P: version::provider::Provider<R>>(driver: D, provider: P) -> Result<Self, error::Error> {
    Ok(Self{
      driver: driver,
      versions: provider.versions()?,
    })
  }
  
  pub fn current_version(&self) -> Result<usize, error::Error> {
    self.driver.version()
  }
  
  pub fn latest_version(&self) -> Result<usize, error::Error> {
    if let Some(max) = self.versions.iter().max() {
      Ok(max.version())
    }else{
      Ok(0)
    }
  }

  pub fn upgrade_latest(&self) -> Result<Vec<usize>, error::Error> {
    self.upgrade(self.latest_version()?)
  }
  
  pub fn upgrade(&self, target: usize) -> Result<Vec<usize>, error::Error> {
    let curr = self.current_version()?;
    let mut applied: Vec<usize> = Vec::new();
    
    for v in &self.versions {
      let candidate = v.version();
      if candidate > curr && candidate <= target {
        println!(">>> >>> >>> APPLY VERSION: {}", candidate);
        applied.push(candidate);
      }
    }
    
    Ok(applied)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn update_okay() {
    let d = mock::Driver::new(0);
    let p = version::provider::DirectoryProvider::new_with_path("./etc/db").unwrap();
    let u = Updater::new(d, p).unwrap();
    println!(">>> CURR {}", u.current_version().unwrap());
    println!(">>> MAXX {}", u.latest_version().unwrap());
    assert_eq!(vec![1, 2], u.upgrade(2).unwrap());
  }
  
}
