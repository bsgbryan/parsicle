use std::cell::RefCell;

use crate::{
  image::Image,
  list::List,
  paragraph::Paragraph,
  section::Section,
};

#[derive(Debug)]
pub struct Content {
    pub ignore: bool,
    pub mode: Mode,
    // pub new_heading: bool,
    // pub new_image: bool,
    // pub new_list_item: bool,
    // pub new_paragraph: bool,
    pub images: Vec<RefCell<Image>>,
    pub links: Vec<String>,
    pub lists: Vec<RefCell<List>>,
    pub sections: Vec<RefCell<Section>>,
}

impl Content {
    pub fn new() -> Self {
        Content {
            ignore: false,
            mode: Mode::Unknown,
            images: vec![],
            // new_heading: false,
            // new_image: false,
            // new_paragraph: false,
            // new_list_item: false,
            links: vec![],
            lists: vec![],
            sections: vec![RefCell::new(Section {
                content: vec![],
                heading: String::new(),
                paragraphs: vec![RefCell::new(Paragraph {
                  tokens: vec![],
                })],
            })],
        }
    }
}

#[derive(Debug)]
pub enum Mode {
  Unknown,
  Heading,
  Image,
  Paragraph,
  ListItem,
}
