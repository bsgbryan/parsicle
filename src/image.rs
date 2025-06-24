use std::fmt::Display;

use postgres_types::{
  FromSql,
  ToSql,
};

use crate::href::sanitize;

#[derive(Clone, Debug, ToSql, FromSql)]
#[postgres(name = "image")]
pub struct Image {
  pub href:    Option<String>,
  pub caption: String,
  pub credit:  String,
}

impl Image {
  pub fn new(caption: &str, credit: &str, href: &str) -> Self {
    Self {
      href: sanitize(href),
      caption: caption.to_owned(),
      credit: credit.to_owned(),
    }
  }
}

impl Display for Image {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "IMAGE").ok();

    if let Some(url) = &self.href {
      writeln!(f, "  href: {}", url).ok();
    }

    writeln!(f, "  credit: {}",  self.credit).ok();
    writeln!(f, "  caption: {}", self.caption).ok();

    Ok(())
  }
}