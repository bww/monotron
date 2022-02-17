mod error;
mod store;
mod model;

use warp::{http, Filter};

#[tokio::main]
async fn main() -> Result<(), error::Error> {
  let store = store::Store::new("localhost", "monotron_development").await?;
  let store_filter = warp::any().map(move || &store);
  
  let inc_entry = warp::path!("hello" / String)
    .and(store_filter.clone())
    .and_then(handle_inc_entry);
  
  warp::serve(inc_entry)
    .run(([127, 0, 0, 1], 3030))
    .await;
  
  Ok(())
}

async fn handle_inc_entry(token: String, store: &store::Store) -> Result<impl warp::Reply, warp::Rejection> {
  Ok(warp::reply::with_status("Ok", http::StatusCode::CREATED))
}
