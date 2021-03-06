mod acl;
mod error;
mod store;
mod model;
mod debug;
mod upgrade;

use std::collections;

use bytes;
use chrono;
use warp::{http, Filter};
use envconfig::Envconfig;
use serde_json::json;

use crate::model::apikey::{self, Authenticate, AccessControl};

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
}

fn root_authorization(key: String, secret: String) -> apikey::Authorization {
  apikey::Authorization{
    account_id: None, // global, no account binding
    scopes: acl::scope::Scopes::new(vec!(
      acl::scope::Scope::new(acl::scope::Operation::Every, acl::scope::Resource::System),
      acl::scope::Scope::new(acl::scope::Operation::Every, acl::scope::Resource::ACL),
      acl::scope::Scope::new(acl::scope::Operation::Every, acl::scope::Resource::Account),
      acl::scope::Scope::new(acl::scope::Operation::Every, acl::scope::Resource::Series),
    )),
    api_key: apikey::ApiKey{
      id: 0,
      key: key,
      secret: secret,
    },
  }
}

#[tokio::main]
async fn main() -> Result<(), error::Error> {
  println!("----> Monotron is starting @ {}", chrono::Utc::now());
  let conf = match Config::init_from_env() {
    Ok(conf) => conf,
    Err(err) => panic!("*** Could not load configuration from environment: {}", err),
  };
  
  let root = match conf.root_api_key {
    Some(key) => Some(root_authorization(key, conf.root_api_secret.unwrap())),
    None => None,
  };
  
  if debug::debug() {
    println!("----> Connecting to database: {}", conf.db_dsn);
  }else{
    println!("----> Connecting to database");
  }
  
  let store = store::Store::new(&conf.db_dsn).await?;
  let applied = store.migrate("./etc/db").await?;
  if applied.len() > 0 {
    println!("----> Applied migrations: {:?}", applied);
  }else{
    println!("----> Applied migrations: none");
  }
  
  let store_filter = warp::any().map(move || store.clone());
  let root_filter = warp::any().map(move || root.clone());
  
  let json_content = warp::reply::with::header("Content-Type", "application/json");
  
  let auth_filter = warp::any()
    .and(store_filter.clone())
    .and(root_filter.clone())
    .and(warp::header::<String>(HEADER_AUTHORIZATION))
    .and_then(handle_auth);
  
  let v1 = warp::path!("v1")
    .and(store_filter.clone())
    .and_then(handle_v1)
    .with(&json_content);
  
  let fetch_account = warp::path!("v1" / "accounts" / i64)
    .and(store_filter.clone())
    .and(auth_filter.clone())
    .and_then(handle_fetch_account)
    .with(&json_content);
  
  let create_authorization = warp::path!("v1" / "accounts" / i64 / "grants")
    .and(store_filter.clone())
    .and(auth_filter.clone())
    .and(warp::body::json())
    .and_then(handle_create_authorization)
    .with(&json_content);
  
  let list_authorizations = warp::path!("v1" / "accounts" / i64 / "grants")
    .and(store_filter.clone())
    .and(auth_filter.clone())
    .and_then(handle_list_authorizations)
    .with(&json_content);
  
  let fetch_authorization = warp::path!("v1" / "accounts" / i64 / "grants" / String)
    .and(store_filter.clone())
    .and(auth_filter.clone())
    .and_then(handle_fetch_authorization)
    .with(&json_content);
  
  let delete_authorization = warp::path!("v1" / "accounts" / i64 / "grants" / String)
    .and(store_filter.clone())
    .and(auth_filter.clone())
    .and_then(handle_delete_authorization)
    .with(&json_content);
  
  let fetch_entry = warp::path!("v1" / "accounts" / i64 / "series" / String)
    .and(store_filter.clone())
    .and(auth_filter.clone())
    .and_then(handle_fetch_entry)
    .with(&json_content);
  
  let fetch_entry_version = warp::path!("v1" / "accounts" / i64 / "series" / String / String)
    .and(store_filter.clone())
    .and(auth_filter.clone())
    .and_then(handle_fetch_entry_version)
    .with(&json_content);
  
  let inc_entry = warp::path!("v1" / "accounts" / i64 / "series" / String / String)
    .and(store_filter.clone())
    .and(auth_filter.clone())
    .and_then(handle_inc_entry)
    .with(&json_content);
  
  let delete_entry = warp::path!("v1" / "accounts" / i64 / "series" / String)
    .and(store_filter.clone())
    .and(auth_filter.clone())
    .and_then(handle_delete_entry)
    .with(&json_content);
  
  let store_token_attrs = warp::path!("v1" / "accounts" / i64 / "tokens" / String / String / "attrs")
    .and(store_filter.clone())
    .and(auth_filter.clone())
    .and(warp::body::json())
    .and_then(handle_store_token_attrs)
    .with(&json_content);
  
  let fetch_token_attrs = warp::path!("v1" / "accounts" / i64 / "tokens" / String / String / "attrs")
    .and(store_filter.clone())
    .and(auth_filter.clone())
    .and_then(handle_fetch_token_attrs)
    .with(&json_content);
  
  let delete_token_attrs = warp::path!("v1" / "accounts" / i64 / "tokens" / String / String / "attrs")
    .and(store_filter.clone())
    .and(auth_filter.clone())
    .and_then(handle_delete_token_attrs)
    .with(&json_content);
  
  let store_token_attr = warp::path!("v1" / "accounts" / i64 / "tokens" / String / String / "attrs" / String)
    .and(store_filter.clone())
    .and(auth_filter.clone())
    .and(warp::body::bytes())
    .and_then(handle_store_token_attr)
    .with(&json_content);
  
  let fetch_token_attr = warp::path!("v1" / "accounts" / i64 / "tokens" / String / String / "attrs" / String)
    .and(store_filter.clone())
    .and(auth_filter.clone())
    .and_then(handle_fetch_token_attr)
    .with(&json_content);
  
  let delete_token_attr = warp::path!("v1" / "accounts" / i64 / "tokens" / String / String / "attrs" / String)
    .and(store_filter.clone())
    .and(auth_filter.clone())
    .and_then(handle_delete_token_attr)
    .with(&json_content);
  
  let gets = warp::get().and(
    v1
      .or(fetch_account)
      .or(list_authorizations)
      .or(fetch_authorization)
      .or(fetch_entry)
      .or(fetch_entry_version)
      .or(fetch_token_attr)
      .or(fetch_token_attrs)
      .recover(handle_rejection),
  );
  let puts = warp::put().and(
    inc_entry
      .or(store_token_attr)
      .or(store_token_attrs)
      .recover(handle_rejection),
  );
  let posts = warp::post().and(
    create_authorization
      .recover(handle_rejection),
  );
  let dels = warp::delete().and(
    delete_authorization
      .or(delete_entry)
      .or(delete_token_attr)
      .or(delete_token_attrs)
      .recover(handle_rejection),
  );
  
  println!("----> Running on :{}", conf.listen);
  warp::serve(gets.or(puts).or(posts).or(dels))
    .run(([0, 0, 0, 0], conf.listen))
    .await;
  
  Ok(())
}

async fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, std::convert::Infallible> {
  if debug::verbose() {
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
  match store.verify_authorization(key, secret).await {
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

async fn handle_fetch_account(account_id: i64, store: store::Store, auth: apikey::Authorization) -> Result<impl warp::Reply, warp::Rejection> {
  auth.assert_allows_in_account(account_id, acl::scope::Operation::Read, acl::scope::Resource::Account)?;
  match store.fetch_account(account_id).await {
    Ok(account) => Ok(warp::reply::with_status(warp::reply::Response::new(json!(account).to_string().into()), http::StatusCode::OK)),
    Err(err) => Err(err.into()),
  }
}

async fn handle_create_authorization(account_id: i64, store: store::Store, auth: apikey::Authorization, scopes: acl::scope::Scopes) -> Result<impl warp::Reply, warp::Rejection> {
  auth.assert_allows_in_account(account_id, acl::scope::Operation::Write, acl::scope::Resource::ACL)?;
  let (key, secret) = apikey::gen_apikey();
  let create = apikey::Authorization{
    account_id: Some(account_id),
    scopes: scopes,
    api_key: apikey::ApiKey{
      id: 0,
      key: key,
      secret: secret,
    },
  };
  match store.store_authorization(&create).await {
    Ok(create) => Ok(warp::reply::with_status(create, http::StatusCode::OK)),
    Err(err) => return Err(err.into()),
  }
}

async fn handle_delete_authorization(account_id: i64, key: String, store: store::Store, auth: apikey::Authorization) -> Result<impl warp::Reply, warp::Rejection> {
  auth.assert_allows_in_account(account_id, acl::scope::Operation::Delete, acl::scope::Resource::ACL)?;
  match store.delete_authorization(account_id, key).await {
    Ok(_) => Ok(warp::reply::reply()),
    Err(err) => Err(err.into()),
  }
}

async fn handle_fetch_authorization(account_id: i64, key: String, store: store::Store, auth: apikey::Authorization) -> Result<impl warp::Reply, warp::Rejection> {
  auth.assert_allows_in_account(account_id, acl::scope::Operation::Read, acl::scope::Resource::ACL)?;
  match store.fetch_authorization_for_account(account_id, key).await {
    Ok(azn) => Ok(warp::reply::with_status(warp::reply::Response::new(json!(azn).to_string().into()), http::StatusCode::OK)),
    Err(err) => Err(err.into()),
  }
}

async fn handle_list_authorizations(account_id: i64, store: store::Store, auth: apikey::Authorization) -> Result<impl warp::Reply, warp::Rejection> {
  auth.assert_allows_in_account(account_id, acl::scope::Operation::Read, acl::scope::Resource::ACL)?;
  match store.fetch_every_authorization_for_account(account_id).await {
    Ok(azns) => Ok(warp::reply::with_status(warp::reply::Response::new(json!(azns).to_string().into()), http::StatusCode::OK)),
    Err(err) => Err(err.into()),
  }
}

async fn handle_fetch_entry(account_id: i64, key: String, store: store::Store, auth: apikey::Authorization) -> Result<impl warp::Reply, warp::Rejection> {
  auth.assert_allows_in_account(account_id, acl::scope::Operation::Read, acl::scope::Resource::Series)?;
  let entry = match store.fetch_entry(account_id, key).await {
    Ok(v) => v,
    Err(err) => return Err(err.into()),
  };
  Ok(warp::reply::with_status(entry, http::StatusCode::OK))
}

async fn handle_delete_entry(account_id: i64, key: String, store: store::Store, auth: apikey::Authorization) -> Result<impl warp::Reply, warp::Rejection> {
  auth.assert_allows_in_account(account_id, acl::scope::Operation::Delete, acl::scope::Resource::Series)?;
  match store.delete_entry(account_id, key).await {
    Ok(_) => Ok(warp::reply::reply()),
    Err(err) => Err(err.into()),
  }
}

async fn handle_fetch_entry_version(account_id: i64, key: String, token: String, store: store::Store, auth: apikey::Authorization) -> Result<impl warp::Reply, warp::Rejection> {
  auth.assert_allows_in_account(account_id, acl::scope::Operation::Read, acl::scope::Resource::Series)?;
  let entry = match store.fetch_entry_version(account_id, key, token).await {
    Ok(v) => v,
    Err(err) => return Err(err.into()),
  };
  Ok(warp::reply::with_status(entry, http::StatusCode::OK))
}

async fn handle_inc_entry(account_id: i64, key: String, token: String, store: store::Store, auth: apikey::Authorization) -> Result<impl warp::Reply, warp::Rejection> {
  auth.assert_allows_in_account(account_id, acl::scope::Operation::Write, acl::scope::Resource::Series)?;
  let entry = match store.inc_entry(account_id, key, Some(token)).await {
    Ok(v) => v,
    Err(err) => return Err(err.into()),
  };
  Ok(warp::reply::with_status(entry, http::StatusCode::OK))
}

async fn handle_fetch_token_attrs(account_id: i64, key: String, token: String, store: store::Store, auth: apikey::Authorization) -> Result<impl warp::Reply, warp::Rejection> {
  auth.assert_allows_in_account(account_id, acl::scope::Operation::Read, acl::scope::Resource::Series)?;
  let attrs = match store.fetch_token_attrs(account_id, key, token).await {
    Ok(v) => v,
    Err(err) => return Err(err.into()),
  };
  Ok(warp::reply::with_status(model::attrs::Attrs::new(attrs), http::StatusCode::OK))
}

async fn handle_store_token_attrs(account_id: i64, key: String, token: String, store: store::Store, auth: apikey::Authorization, attrs: collections::HashMap<String, String>) -> Result<impl warp::Reply, warp::Rejection> {
  auth.assert_allows_in_account(account_id, acl::scope::Operation::Write, acl::scope::Resource::Series)?;
  match store.store_token_attrs(account_id, key, token, &attrs).await {
    Ok(_) => Ok(warp::reply::with_status(model::attrs::Attrs::new(attrs), http::StatusCode::OK)),
    Err(err) => Err(err.into()),
  }
}

async fn handle_delete_token_attrs(account_id: i64, key: String, token: String, store: store::Store, auth: apikey::Authorization) -> Result<impl warp::Reply, warp::Rejection> {
  auth.assert_allows_in_account(account_id, acl::scope::Operation::Write, acl::scope::Resource::Series)?;
  match store.delete_token_attrs(account_id, key, token).await {
    Ok(_) => Ok(warp::reply::reply()),
    Err(err) => Err(err.into()),
  }
}

async fn handle_store_token_attr(account_id: i64, key: String, token: String, name: String, store: store::Store, auth: apikey::Authorization, value: bytes::Bytes) -> Result<impl warp::Reply, warp::Rejection> {
  auth.assert_allows_in_account(account_id, acl::scope::Operation::Write, acl::scope::Resource::Series)?;
  let value = match String::from_utf8(value.to_vec()) {
    Ok(value) => value,
    Err(err) => return Err(error::Error::Utf8Error(err.utf8_error()).into()),
  };
  match store.store_token_attr(account_id, key, token, &name, &value).await {
    Ok(_) => Ok(warp::reply::with_status(model::attrs::Attrs::singleton(name, value), http::StatusCode::OK)),
    Err(err) => Err(err.into()),
  }
}

async fn handle_fetch_token_attr(account_id: i64, key: String, token: String, name: String, store: store::Store, auth: apikey::Authorization) -> Result<impl warp::Reply, warp::Rejection> {
  auth.assert_allows_in_account(account_id, acl::scope::Operation::Read, acl::scope::Resource::Series)?;
  let val = match store.fetch_token_attr(account_id, key, token, name).await {
    Ok(v) => v,
    Err(err) => return Err(err.into()),
  };
  Ok(warp::reply::with_status(val, http::StatusCode::OK))
}

async fn handle_delete_token_attr(account_id: i64, key: String, token: String, name: String, store: store::Store, auth: apikey::Authorization) -> Result<impl warp::Reply, warp::Rejection> {
  auth.assert_allows_in_account(account_id, acl::scope::Operation::Write, acl::scope::Resource::Series)?;
  match store.delete_token_attr(account_id, key, token, name).await {
    Ok(_) => Ok(warp::reply::reply()),
    Err(err) => Err(err.into()),
  }
}
