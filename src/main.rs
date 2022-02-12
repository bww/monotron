mod error;
mod store;
mod model;

#[tokio::main]
async fn main() -> Result<(), error::Error> {
  let store = store::Store::new("localhost").await?;
  let check = store.check().await?;
  println!(">>> {}", check);
  Ok(())
}
