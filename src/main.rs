mod error;
mod store;
mod model;

use warp::{http, Filter};
use envconfig::Envconfig;
use crate::model::apikey;

const HEADER_AUTHORIZATION: &str = "Authorization";

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
    .and(warp::header::<String>(HEADER_AUTHORIZATION))
    .and_then(handle_auth);
  
  let v1 = warp::path!("v1")
    .and(store_filter.clone())
    .and_then(handle_v1);
  
  let get_entry = warp::path!("v1" / "entry" / String)
    .and(store_filter.clone())
    .and(auth_filter.clone())
    .and_then(handle_get_entry);
  
  let inc_entry = warp::path!("v1" / "entry" / String / String)
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
  } else if let Some(cause) = err.find::<warp::reject::MissingHeader>() {
    handle_missing_header(cause)
  } else if let Some(cause) = err.find::<model::scope::Error>() {
    handle_scope_error(cause)
  } else if let Some(cause) = err.find::<model::apikey::Error>() {
    handle_apikey_error(cause)
  } else if let Some(cause) = err.find::<store::error::Error>() {
    handle_persist_error(cause)
  } else if let Some(cause) = err.find::<error::Error>() {
    handle_general_error(cause)
  }else{
    Ok(warp::reply::with_status("INTERNAL_SERVER_ERROR", http::StatusCode::INTERNAL_SERVER_ERROR))
  }
}

fn handle_missing_header(err: &warp::reject::MissingHeader) -> Result<warp::reply::WithStatus<&'static str>, std::convert::Infallible> {
  match err.name() {
    HEADER_AUTHORIZATION => Ok(warp::reply::with_status("UNAUTHORIZED", http::StatusCode::UNAUTHORIZED)),
    _ => Ok(warp::reply::with_status("MISSING_HEADER", http::StatusCode::BAD_REQUEST)),
  }
}

fn handle_scope_error(err: &model::scope::Error) -> Result<warp::reply::WithStatus<&'static str>, std::convert::Infallible> {
  match err {
    _ => Ok(warp::reply::with_status("ACL_ERROR", http::StatusCode::INTERNAL_SERVER_ERROR)),
  }
}

fn handle_apikey_error(err: &model::apikey::Error) -> Result<warp::reply::WithStatus<&'static str>, std::convert::Infallible> {
  match err {
    model::apikey::Error::Unauthorized(_) => Ok(warp::reply::with_status("UNAUTHORIZED", http::StatusCode::UNAUTHORIZED)),
    model::apikey::Error::Forbidden(_) => Ok(warp::reply::with_status("FORBIDDEN", http::StatusCode::FORBIDDEN)),
    _ => Ok(warp::reply::with_status("ACL_ERROR", http::StatusCode::INTERNAL_SERVER_ERROR)),
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
    error::Error::DecodeBase64Error(_) => Ok(warp::reply::with_status("BAD_REQUEST", http::StatusCode::BAD_REQUEST)),
    _ => Ok(warp::reply::with_status("INTERNAL_SERVER_ERROR", http::StatusCode::INTERNAL_SERVER_ERROR)),
  }
}

async fn handle_auth(store: store::Store, header: String) -> Result<apikey::Authorization, warp::Rejection> {
  let (key, secret) = model::apikey::parse_apikey(&header)?;
  match store.fetch_authorization(key, secret).await {
    Ok(auth) => Ok(auth),
    Err(err) => match err {
      store::error::Error::NotFoundError => Err(model::apikey::Error::Unauthorized("Invalid API Key".to_string()).into()),
      err => Err(err.into()),
    },
  }
}

async fn handle_v1(_store: store::Store) -> Result<impl warp::Reply, warp::Rejection> {
  Ok(warp::reply::with_status("API v1", http::StatusCode::OK))
}

async fn handle_get_entry(key: String, store: store::Store, auth: apikey::Authorization) -> Result<impl warp::Reply, warp::Rejection> {
  auth.assert_allows(model::scope::Operation::Read, model::scope::Resource::Entry)?;
  let entry = match store.fetch_entry(&auth, key).await {
    Ok(v) => v,
    Err(err) => return Err(err.into()),
  };
  Ok(warp::reply::with_status(entry, http::StatusCode::OK))
}

async fn handle_inc_entry(key: String, token: String, store: store::Store, auth: apikey::Authorization) -> Result<impl warp::Reply, warp::Rejection> {
  auth.assert_allows(model::scope::Operation::Write, model::scope::Resource::Entry)?;
  let entry = match store.inc_entry(&auth, key, Some(token)).await {
    Ok(v) => v,
    Err(err) => return Err(err.into()),
  };
  Ok(warp::reply::with_status(entry, http::StatusCode::OK))
}
