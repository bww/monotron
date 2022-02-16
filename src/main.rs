mod error;
mod store;
mod model;

#[tokio::main]
async fn main() -> Result<(), error::Error> {
  let store = store::Store::new("localhost", "monotron_development").await?;
  store.store_entry(&model::entry::Entry::new("foo1", 1, 1)).await?;
  Ok(())
}
