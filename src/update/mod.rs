pub mod error;
pub mod version;

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

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn provide_versions() {
  }
  
}
