#[derive(Debug, Clone)]
pub struct List {
  pub items: Vec<String>,
}

impl List {
  pub fn new() -> Self {
    List {
      items: vec![],
    }
  }
}
