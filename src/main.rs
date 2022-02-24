mod error;
mod store;
mod model;

use warp::{http, Filter};

#[tokio::main]
async fn main() -> Result<(), error::Error> {
  let store = store::Store::new("localhost", "monotron_development").await?;
  let store_filter = warp::any().map(move || store.clone());
  
  let v1 = warp::path!("v1")
    .and(store_filter.clone())
    .and_then(handle_v1);
  
  let inc_entry = warp::path!("v1" / String / String)
    .and(store_filter.clone())
    .and_then(handle_inc_entry);
  
  let routes = warp::get().and(
    v1
      .or(inc_entry)
      .recover(handle_rejection),
  );
  
  warp::serve(routes)
    .run(([127, 0, 0, 1], 3030))
    .await;
  
  Ok(())
}

async fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, std::convert::Infallible> {
  if err.is_not_found() {
    Ok(warp::reply::with_status("NOT_FOUND", http::StatusCode::NOT_FOUND))
  } else {
    Ok(warp::reply::with_status("INTERNAL_SERVER_ERROR", http::StatusCode::INTERNAL_SERVER_ERROR))
  }
}

async fn handle_v1(store: store::Store) -> Result<impl warp::Reply, warp::Rejection> {
  Ok(warp::reply::with_status("API v1", http::StatusCode::OK))
}

async fn handle_inc_entry(key: String, token: String, store: store::Store) -> Result<impl warp::Reply, warp::Rejection> {
  let value = match store.check().await {
    Ok(v) => v,
    Err(err) => return Err(err.into()),
  };
  println!(">>> KEY TOK: {} {}", key, token);
  Ok(warp::reply::with_status(value, http::StatusCode::OK))
}
