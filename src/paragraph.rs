#[derive(Debug, Clone)]
pub struct Paragraph {
  pub tokens: Vec<String>,
}

impl Paragraph {
  pub fn new(text: &str) -> Self {
    Paragraph {
      tokens: vec![text.to_string()],
    }
  }
}
