use std::fmt::Display;

use postgres_types::{
  FromSql,
  ToSql,
};

use crate::href::sanitize;

#[derive(Debug, ToSql, FromSql)]
#[postgres(name = "author")]
pub struct Author {
  pub href: Option<String>,
  pub name: String,
}

impl Author {
  pub fn new(name: &str, href: &str) -> Self {
    Self {
      href: sanitize(href),
      name: name.to_owned(),
    }
  }
}

impl Display for Author {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if let Some(url) = &self.href {
      writeln!(f, "AUTHOR -> {}: {url}", self.name).ok();
    }

    Ok(())
  }
}