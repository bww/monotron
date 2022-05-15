pub mod io;
pub mod error;
pub mod version;
pub mod driver;

use crate::update;
use crate::update::driver::mock;
use crate::update::io::{IntoRead, BytesIntoRead};

pub trait Driver<R: IntoRead> {
  fn version(&self) -> Result<usize, error::Error>;
  fn apply(&self, version: version::Version<R>) -> Result<(), error::Error>;
}

pub struct Updater<R: IntoRead, D: Driver<R>> {
  driver: D,
  versions: Vec<version::Version<R>>,
}

impl<R: IntoRead, D: Driver<R>> Updater<R, D> {
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
        self.driver.apply(v.to_owned())?;
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
  fn update_success() {
    let d = mock::Driver::new(0, None);
    let v: Vec<version::Version<BytesIntoRead>> = vec![
      version::Version::new_with_bytes(1, bytes::Bytes::from("1")).unwrap(),
      version::Version::new_with_bytes(2, bytes::Bytes::from("2")).unwrap(),
      version::Version::new_with_bytes(3, bytes::Bytes::from("3")).unwrap(),
    ];
    
    let p = version::provider::ConstantProvider::new(v);
    let u = Updater::new(d, p).unwrap();
    assert_eq!(0, u.current_version().unwrap());
    assert_eq!(3, u.latest_version().unwrap());
    
    assert_eq!(vec![1, 2], u.upgrade(2).unwrap()); // mock driver doesn't update state
    assert_eq!(vec![1, 2, 3], u.upgrade_latest().unwrap());
  }
  
  #[test]
  fn update_failure() {
    let errmsg = "upgrade failed";
    let d = mock::Driver::new(0, Some(errmsg));
    let v: Vec<version::Version<BytesIntoRead>> = vec![
      version::Version::new_with_bytes(1, bytes::Bytes::from("1")).unwrap(),
      version::Version::new_with_bytes(2, bytes::Bytes::from("2")).unwrap(),
      version::Version::new_with_bytes(3, bytes::Bytes::from("3")).unwrap(),
    ];
    
    let p = version::provider::ConstantProvider::new(v);
    let u = Updater::new(d, p).unwrap();
    
    let err = match u.upgrade(2) {
      Ok(v) => panic!("Expected an error"),
      Err(err) => err,
    };
    
    println!("*** {}", err);
    assert_eq!(format!("{}", error::Error::UpgradeError(1, "1B".to_owned(), errmsg.to_owned())), format!("{}", err));
  }
  
  // #[test]
  // fn update_okay() {
  //   let d = mock::Driver::new(0, false);

  //   let p = version::provider::DirectoryProvider::new_with_path("./etc/db").unwrap();
  //   let u = Updater::new(d, p).unwrap();
  //   println!(">>> CURR {}", u.current_version().unwrap());
  //   println!(">>> MAXX {}", u.latest_version().unwrap());
  //   assert_eq!(vec![1, 2], u.upgrade(2).unwrap());
  // }
  
}
