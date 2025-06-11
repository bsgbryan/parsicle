use std::cell::RefCell;

use crate::paragraph::Paragraph;

#[derive(Debug, Clone)]
pub struct Section {
  pub content: Vec<String>,
  pub heading: String,
  pub paragraphs: Vec<RefCell<Paragraph>>,
}

impl Section {
  pub fn new(text: &str) -> Self {
    Section {
      content: vec![],
      heading: text.to_string(),
      paragraphs: vec![RefCell::new(Paragraph {
        tokens: vec![],
      })],
    }
  }
}
