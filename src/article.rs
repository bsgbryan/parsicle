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
  pub alternate:   Option<Vec<(String, Url)>>,
  pub authors:     Vec<Author>,
  pub canonical:   Url,
  pub content:     Option<Vec<String>>,
  pub description: Option<String>,
  pub hero_image:  Option<Url>,
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

    if let Some(hero)   = &self.hero_image &&
       let Some(domain) = hero.domain()
    {
      let url = format!("{}://{}{}",
        hero.scheme(),
        domain,
        hero.path(),
      );

      let _ = writeln!(f, "  hero image: {}", url);
    }

    if let Some(domain) = self.canonical.domain() {
      let url = format!("{}://{}{}",
        self.canonical.scheme(),
        domain,
        self.canonical.path(),
      );

      let _ = writeln!(f, "  canonical url: {}", url);
    }

    if let Some(alternates) = &self.alternate {
      for (l, a) in alternates {
        if let Some(domain) = a.domain() {
          let url = format!("{}://{}{}",
            a.scheme(),
            domain,
            a.path(),
          );
  
          let _ = writeln!(f, "  alternate url -> lang: {l}, href: {url}");
        }
      }
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