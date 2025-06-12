use std::fmt::Display;

use url::Url;

use crate::{
  image::Image,
  section::Section,
};

#[derive(PartialEq)]
pub enum Mode {
  Heading,
  Image,
  ImageCredit,
  Paragraph,
  Text,
}

impl Display for Mode {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Heading     => write!(f, "Heading"),
      Self::Image       => write!(f, "Image"),
      Self::ImageCredit => write!(f, "ImageCredit"),
      Self::Paragraph   => write!(f, "Paragraph"),
      Self::Text        => write!(f, "Text"),
    }
  }
}

pub struct Content {
        mode: Mode,
    pub images:   Vec<Image>,
    pub links:    Vec<Url>,
    pub sections: Vec<Section>,
}

impl Default for Content {
  fn default() -> Self {
    Content {
      mode: Mode::Text,
      images: vec![],
      links: vec![],
      sections: vec![Section::default()],
    }
  }
}

impl Display for Content {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let _ = writeln!(f, "SECTIONS");
    for s in self.sections.clone() {
      let _ = writeln!(f, "\nHeading: {}", s.heading);

      for p in s.paragraphs { let _ = writeln!(f, "  {p}"); }
    }

    let _ = writeln!(f, "\nLINKS");
    for l in self.links.clone() { let _ = writeln!(f, "url: {l}"); }

    let _ = writeln!(f, "\nIMAGES");
    for i in self.images.clone() {
      let _ = writeln!(f, "url: {}", i.url);

      if let Some(caption) = i.caption {
        let _ = writeln!(f, "  caption: {caption}");
      }

      if let Some(credit) = i.credit {
        let _ = writeln!(f, "  credit:  {credit}");
      }
    }

    Ok(())
  }
}

impl Content {
  pub fn new(title: &str) -> Self {
    Self {
      sections: vec![Section::with_heading(title)],
      ..Default::default()
    }
  }

  pub fn set(&mut self, mode: Mode) {
    self.mode = mode;
  }

  pub fn is_in(&self, mode: Mode) -> bool {
    self.mode == mode
  }

  pub fn mode(&self) -> &Mode {
    &self.mode
  }

  pub fn add_link(&mut self, url: &str) {
    if let Ok(parsed) = Url::parse(url) {
      self.links.push(parsed);
    }
  }

  pub fn add_image(&mut self, url: &str) {
    self.images.push(Image::new(url));
    self.mode = Mode::Image;
  }

  pub fn add_caption(&mut self, text: &str) {
    if let Some(img) = self.images.last_mut() {
      img.caption = Some(text.to_string());
      self.mode = Mode::ImageCredit;
    }
  }

  pub fn add_credit(&mut self, name: &str) {
    if let Some(img) = self.images.last_mut() {
      let length = name.len();
      if let Some(credit) = name.get(9..length - 2) {
        img.credit = Some(credit.to_string());
        self.mode = Mode::Paragraph;
      }
    }
  }

  pub fn add_section(&mut self, text: &str) {
    self.sections.push(Section::with_heading(text));
    self.mode = Mode::Text;
  }

  pub fn add_paragraph(&mut self, text: &str) {
    if let Some(section) = self.sections.last_mut() {
      section.add_paragraph(text);
      self.mode = Mode::Text;
    }
  }

  pub fn add_text(&mut self, value: &str) {
    if let Some(section) = self.sections.last_mut() {
      section.add_text(value);
      self.mode = Mode::Text;
    }
  }
}
