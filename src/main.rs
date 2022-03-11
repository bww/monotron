mod error;
mod store;
mod model;

use warp::{http, Filter};
use envconfig::Envconfig;
use crate::model::apikey;

#[derive(Envconfig)]
pub struct Config {
  #[envconfig(from = "DB_DSN", default = "postgresql://postgres@localhost/monotron_development?connect_timeout=5")]
  pub db_dsn: String,
}

#[tokio::main]
async fn main() -> Result<(), error::Error> {
  let conf = match Config::init_from_env() {
    Ok(conf) => conf,
    Err(err) => panic!("*** Could not load configuration from environment: {}", err),
  };
  
  let store = store::Store::new(&conf.db_dsn).await?;
  let store_filter = warp::any().map(move || store.clone());
  
  let auth_filter = warp::any()
    .and(store_filter.clone())
    .and_then(handle_auth);
  
  let v1 = warp::path!("v1")
    .and(store_filter.clone())
    .and_then(handle_v1);
  
  let get_entry = warp::path!("v1" / String)
    .and(store_filter.clone())
    .and(auth_filter.clone())
    .and_then(handle_get_entry);
  
  let inc_entry = warp::path!("v1" / String / String)
    .and(store_filter.clone())
    .and(auth_filter.clone())
    .and_then(handle_inc_entry);
  
  let gets = warp::get().and(
    v1
      .or(get_entry)
      .recover(handle_rejection),
  );
  let puts = warp::put().and(
    inc_entry
      .recover(handle_rejection),
  );
  
  warp::serve(gets.or(puts))
    .run(([127, 0, 0, 1], 3030))
    .await;
  
  Ok(())
}

async fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, std::convert::Infallible> {
  println!("*** {:?}", &err);
  if err.is_not_found() {
    Ok(warp::reply::with_status("NOT_FOUND", http::StatusCode::NOT_FOUND))
  } else if let Some(cause) = err.find::<store::error::Error>() {
    handle_persist_error(cause)
  } else if let Some(cause) = err.find::<error::Error>() {
    handle_general_error(cause)
  }else{
    Ok(warp::reply::with_status("INTERNAL_SERVER_ERROR", http::StatusCode::INTERNAL_SERVER_ERROR))
  }
}

fn handle_persist_error(err: &store::error::Error) -> Result<warp::reply::WithStatus<&'static str>, std::convert::Infallible> {
  match err {
    store::error::Error::NotFoundError => Ok(warp::reply::with_status("NOT_FOUND", http::StatusCode::NOT_FOUND)),
    _ => Ok(warp::reply::with_status("PERSISTENCE_ERROR", http::StatusCode::INTERNAL_SERVER_ERROR)),
  }
}

fn handle_general_error(err: &error::Error) -> Result<warp::reply::WithStatus<&'static str>, std::convert::Infallible> {
  match err {
    error::Error::NotFoundError(_) => Ok(warp::reply::with_status("NOT_FOUND", http::StatusCode::NOT_FOUND)),
    _ => Ok(warp::reply::with_status("INTERNAL_SERVER_ERROR", http::StatusCode::INTERNAL_SERVER_ERROR)),
  }
}

async fn handle_auth(store: store::Store) -> Result<apikey::Authorization, warp::Rejection> {
  match store.fetch_authorization("bootstrap".to_string(), "ztLvoY6IKyxA".to_string()).await {
    Ok(auth) => Ok(auth),
    Err(err) => match err {
      store::error::Error::NotFoundError => Err(error::Error::Unauthorized.into()),
      err => Err(err.into()),
    }
  }
}

async fn handle_v1(_store: store::Store) -> Result<impl warp::Reply, warp::Rejection> {
  Ok(warp::reply::with_status("API v1", http::StatusCode::OK))
}

async fn handle_get_entry(key: String, store: store::Store, auth: apikey::Authorization) -> Result<impl warp::Reply, warp::Rejection> {
  let entry = match store.fetch_entry(&auth, key).await {
    Ok(v) => v,
    Err(err) => return Err(err.into()),
  };
  Ok(warp::reply::with_status(entry, http::StatusCode::OK))
}

async fn handle_inc_entry(key: String, token: String, store: store::Store, auth: apikey::Authorization) -> Result<impl warp::Reply, warp::Rejection> {
  let entry = match store.inc_entry(&auth, key, Some(token)).await {
    Ok(v) => v,
    Err(err) => return Err(err.into()),
  };
  Ok(warp::reply::with_status(entry, http::StatusCode::OK))
}
