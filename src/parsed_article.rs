use url::Url;

use crate::{
  content::Content,
  image::Image,
  section::Section,
};

#[derive(Debug)]
pub struct ParsedArticle {
  pub content: Vec<Section>,
  pub images: Vec<Image>,
  pub links: Vec<Url>,
}

impl ParsedArticle {
  pub fn new(content: &Content) -> Self {
    Self {
      content: content.sections.to_owned(),
      images:  content.images.to_owned(),
      links:   content.links.to_owned(),
    }
  }

  pub fn empty() -> Self {
    Self {
      content: Vec::with_capacity(0),
      images:  Vec::with_capacity(0),
      links:   Vec::with_capacity(0),
    }
  }
}
