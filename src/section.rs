#[derive(Debug, Clone)]
pub struct Section {
  pub heading: String,
  pub paragraphs: Vec<String>,
}

impl Default for Section {
  fn default() -> Self {
    Section {
      heading: String::new(),
      paragraphs: vec![],
    }
  }
}

impl Section {
  pub fn with_heading(text: &str) -> Self {
    Section {
      heading: text.to_string(),
      paragraphs: vec![],
    }
  }

  pub fn add_paragraph(&mut self, text: &str) {
    self.paragraphs.push(text.to_string());
  }

  pub fn add_text(&mut self, value: &str) {
    if let Some(p) = self.paragraphs.last_mut() {
      *p += value;
    }
  }
}
