mod acl;
mod error;
mod store;
mod model;

use chrono;
use warp::{http, Filter};
use envconfig::Envconfig;
use once_cell::sync;

use crate::model::apikey;

static DEBUG: sync::OnceCell<bool> = sync::OnceCell::new();

const HEADER_AUTHORIZATION: &str = "Authorization";

#[derive(Envconfig)]
pub struct Config {
  #[envconfig(from = "DB_DSN", default = "postgresql://postgres@localhost/monotron_development?connect_timeout=5")]
  pub db_dsn: String,
  #[envconfig(from = "LISTEN", default = "3030")]
  pub listen: u16,
  #[envconfig(from = "ROOT_API_KEY")]
  pub root_api_key: Option<String>,
  #[envconfig(from = "ROOT_API_SECRET")]
  pub root_api_secret: Option<String>,
  #[envconfig(from = "DEBUG", default = "false")]
  pub debug: bool,
}

#[tokio::main]
async fn main() -> Result<(), error::Error> {
  println!("----> Monotron is starting @ {}", chrono::Utc::now());
  let conf = match Config::init_from_env() {
    Ok(conf) => conf,
    Err(err) => panic!("*** Could not load configuration from environment: {}", err),
  };
  
  DEBUG.set(conf.debug).expect("Could not set global debug state");
  
  let root = match conf.root_api_key {
    Some(key) => Some(apikey::Authorization{
      account_id: 0,
      api_key: apikey::ApiKey{
        id: 0,
        key: key,
        secret: if let Some(secret) = conf.root_api_secret { secret } else { String::new() },
        scopes: acl::scope::Scopes::new(vec!(
          acl::scope::Scope::new(acl::scope::Operation::Every, acl::scope::Resource::System),
          acl::scope::Scope::new(acl::scope::Operation::Every, acl::scope::Resource::Entry),
        )),
      },
    }),
    None => None,
  };
  
  println!("----> Connecting to DB...");
  let store = store::Store::new(&conf.db_dsn).await?;
  let store_filter = warp::any().map(move || store.clone());
  let root_filter = warp::any().map(move || root.clone());
  
  let auth_filter = warp::any()
    .and(store_filter.clone())
    .and(root_filter.clone())
    .and(warp::header::<String>(HEADER_AUTHORIZATION))
    .and_then(handle_auth);
  
  let v1 = warp::path!("v1")
    .and(store_filter.clone())
    .and_then(handle_v1);
  
  let get_entry = warp::path!("v1" / "series" / String)
    .and(store_filter.clone())
    .and(auth_filter.clone())
    .and_then(handle_get_entry);
  
  let get_entry_version = warp::path!("v1" / "series" / String / String)
    .and(store_filter.clone())
    .and(auth_filter.clone())
    .and_then(handle_get_entry_version);
  
  let inc_entry = warp::path!("v1" / "series" / String / String)
    .and(store_filter.clone())
    .and(auth_filter.clone())
    .and_then(handle_inc_entry);
  
  let delete_entry = warp::path!("v1" / "series" / String)
    .and(store_filter.clone())
    .and(auth_filter.clone())
    .and_then(handle_delete_entry);
  
  let delete_entry_version = warp::path!("v1" / "series" / String / String)
    .and(store_filter.clone())
    .and(auth_filter.clone())
    .and_then(handle_delete_entry_version);
  
  let gets = warp::get().and(
    v1
      .or(get_entry)
      .or(get_entry_version)
      .recover(handle_rejection),
  );
  let puts = warp::put().and(
    inc_entry
      .recover(handle_rejection),
  );
  let dels = warp::delete().and(
    delete_entry
      .or(delete_entry_version)
      .recover(handle_rejection),
  );
  
  println!("----> Running on :{}", conf.listen);
  warp::serve(gets.or(puts).or(dels))
    .run(([0, 0, 0, 0], conf.listen))
    .await;
  
  Ok(())
}

async fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, std::convert::Infallible> {
  if *DEBUG.get().unwrap() {
    println!("*** {:?}", &err);
  }
  if err.is_not_found() {
    Ok(warp::reply::with_status("NOT_FOUND", http::StatusCode::NOT_FOUND))
  } else if let Some(cause) = err.find::<warp::reject::MissingHeader>() {
    handle_missing_header(cause)
  } else if let Some(cause) = err.find::<acl::scope::Error>() {
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

fn handle_scope_error(err: &acl::scope::Error) -> Result<warp::reply::WithStatus<&'static str>, std::convert::Infallible> {
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

async fn handle_auth(store: store::Store, root: Option<apikey::Authorization>, header: String) -> Result<apikey::Authorization, warp::Rejection> {
  let (key, secret) = model::apikey::parse_apikey(&header)?;
  if let Some(root) = root {
    if root.auth(&key, &secret) {
      return Ok(root);
    }
  }
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
  auth.assert_allows(acl::scope::Operation::Read, acl::scope::Resource::Entry)?;
  let entry = match store.fetch_entry(&auth, key).await {
    Ok(v) => v,
    Err(err) => return Err(err.into()),
  };
  Ok(warp::reply::with_status(entry, http::StatusCode::OK))
}

async fn handle_delete_entry(key: String, store: store::Store, auth: apikey::Authorization) -> Result<impl warp::Reply, warp::Rejection> {
  auth.assert_allows(acl::scope::Operation::Delete, acl::scope::Resource::Entry)?;
  match store.delete_entry(&auth, key).await {
    Ok(_) => Ok(warp::reply::reply()),
    Err(err) => Err(err.into()),
  }
}

async fn handle_get_entry_version(key: String, token: String, store: store::Store, auth: apikey::Authorization) -> Result<impl warp::Reply, warp::Rejection> {
  auth.assert_allows(acl::scope::Operation::Read, acl::scope::Resource::Entry)?;
  let entry = match store.fetch_entry_version(&auth, key, token).await {
    Ok(v) => v,
    Err(err) => return Err(err.into()),
  };
  Ok(warp::reply::with_status(entry, http::StatusCode::OK))
}

async fn handle_inc_entry(key: String, token: String, store: store::Store, auth: apikey::Authorization) -> Result<impl warp::Reply, warp::Rejection> {
  auth.assert_allows(acl::scope::Operation::Write, acl::scope::Resource::Entry)?;
  let entry = match store.inc_entry(&auth, key, Some(token)).await {
    Ok(v) => v,
    Err(err) => return Err(err.into()),
  };
  Ok(warp::reply::with_status(entry, http::StatusCode::OK))
}

async fn handle_delete_entry_version(key: String, token: String, store: store::Store, auth: apikey::Authorization) -> Result<impl warp::Reply, warp::Rejection> {
  auth.assert_allows(acl::scope::Operation::Delete, acl::scope::Resource::Entry)?;
  match store.delete_entry_version(&auth, key, token).await {
    Ok(_) => Ok(warp::reply::reply()),
    Err(err) => Err(err.into()),
  }
}

