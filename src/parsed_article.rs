use crate::{
  image::Image,
  list::List,
  section::Section,
};

#[derive(Debug)]
pub struct ParsedArticle {
  pub content: Vec<Section>,
  pub images: Vec<Image>,
  pub links: Vec<String>,
  pub lists: Vec<List>,
  pub title: String,
}
