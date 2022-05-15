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
  pub fn new<P: version::Provider<R>>(driver: D, provider: P) -> Result<Self, error::Error> {
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
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn update_okay() {
    let d = mock::Driver::new(0);
    let p = version::DirectoryProvider::new_with_path("./etc/db").unwrap();
    let u = Updater::new(d, p).unwrap();
    println!(">>> CURR {}", u.current_version().unwrap());
    println!(">>> MAXX {}", u.latest_version().unwrap());
  }
  
}
