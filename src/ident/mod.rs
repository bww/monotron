use std::fmt;
use std::str::FromStr;
use std::string::ToString;

use xid;
use serde;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct Ident(pub xid::Id);

impl Ident {
  pub fn new() -> Ident {
    Self(xid::new())
  }

  pub fn parse(s: &str) -> Result<Self, xid::ParseIdError> {
    return Self::from_str(s);
  }
}

impl fmt::Display for Ident {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0.to_string())
  }
}

impl FromStr for Ident {
  type Err = xid::ParseIdError;
  fn from_str(s: &str) -> Result<Ident, xid::ParseIdError> {
    match xid::Id::from_str(s) {
      Ok(id) => Ok(Ident(id)),
      Err(err) => Err(err),
    }
  }
}

impl serde::Serialize for Ident {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serializer.serialize_str(&self.to_string())
  }
}

impl<'de> serde::Deserialize<'de> for Ident {
  fn deserialize<D>(deserializer: D) -> Result<Ident, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    let s = String::deserialize(deserializer)?;
    let v = xid::Id::from_str(&s).map_err(serde::de::Error::custom)?;
    Ok(Ident(v))
  }
}
