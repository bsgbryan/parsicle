use std::fmt::Display;

use url::Url;

#[derive(Debug)]
pub struct Author {
  pub href: Url,
  pub name: String,
}

impl Display for Author {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if let Some(domain) = self.href.domain() {
      let url = format!("{}://{}{}",
        self.href.scheme(),
        domain,
        self.href.path(),
      );

      let _ = writeln!(f, "AUTHOR -> {}: {}", self.name, url);
    }

    Ok(())
  }
}