#[derive(Debug, Clone)]
pub struct Image {
  pub caption: Option<String>,
  pub credit: Option<String>,
  pub url: String,
}

impl Image {
  pub fn new(url: &str) -> Self {
    Image {
        caption: None,
        credit: None,
        url: url.to_string(),
    }
  }
}