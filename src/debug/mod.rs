use std::env;

use once_cell::sync;

static DEBUG: sync::OnceCell<bool> = sync::OnceCell::new();
static VERBOSE: sync::OnceCell<bool> = sync::OnceCell::new();

pub fn is_set(key: &str) -> bool {
  match env::var(key) {
    Ok(val) => val != "",
    Err(_) => false,
  }
}

pub fn debug() -> bool {
  if let Some(v) = DEBUG.get() {
    return *v;
  }
  let v = is_set("DEBUG");
  DEBUG.set(v).expect("Could not set global debug flag");
  v
}

pub fn verbose() -> bool {
  if let Some(v) = VERBOSE.get() {
    return *v || debug();
  }
  let v = is_set("VERBOSE");
  VERBOSE.set(v).expect("Could not set global verbose flag");
  v || debug()
}
