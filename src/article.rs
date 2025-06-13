use std::fmt::Display;

use chrono::{
  DateTime,
  Utc,
};
use url::Url;

use crate::{
  author::Author,
  image::Image,
};

#[derive(Debug)]
pub struct Article {
  pub authors:     Vec<Author>,
  pub content:     Option<Vec<String>>,
  pub description: Option<String>,
  pub href:        Url,
  pub images:      Option<Vec<Image>>,
  pub published:   Option<DateTime<Utc>>,
  pub title:       String,
}

impl Display for Article {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let _ = writeln!(f, "ARTICLE: {}", self.title);

    if let Some(published) = self.published {
      let _ = writeln!(f, "  published: {}", published);
    }

    if let Some(domain) = self.href.domain() {
      let url = format!("{}://{}{}",
        self.href.scheme(),
        domain,
        self.href.path(),
      );

      let _ = writeln!(f, "  canonical url: {}", url);
    }
    
    match &self.description {
      Some(d) => { let _ = writeln!(f, "  description: {d}" ); }
      None    => { let _ = writeln!(f, "  description: None"); }
    }

    for a in &self.authors { let _ = write!(f, "{a}"); }
    
    if let Some(images) = &self.images {
      for i in images  { let _ = write!(f, "{i}"); }
    }

    let _ = writeln!(f, "CONTENT:");
    if let Some(content) = &self.content {
      for c in content { let _ = writeln!(f, "  {c}"); }
    }

    Ok(())
  }
}