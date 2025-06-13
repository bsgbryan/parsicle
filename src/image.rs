use std::fmt::Display;

use url::Url;

#[derive(Clone, Debug)]
pub struct Image {
  pub href:    Url,
  pub caption: String,
  pub credit:  String,
}

impl Display for Image {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let _ = writeln!(f, "IMAGE");

    if let Some(domain) = self.href.domain() {
      let url = format!("{}://{}{}",
        self.href.scheme(),
        domain,
        self.href.path(),
      );

      let _ = writeln!(f, "  href: {}", url);
    }

    let _ = writeln!(f, "  credit: {}",  self.credit);
    let _ = writeln!(f, "  caption: {}", self.caption);

    Ok(())
  }
}