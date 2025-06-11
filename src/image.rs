#[derive(Debug, Clone)]
pub struct Image {
  pub caption: Option<String>,
  pub title: Option<String>,
  pub url: String,
}

impl Image {
  pub fn new(url: String) -> Self {
    Image {
        caption: None,
        title: None,
        url,
    }
  }
}