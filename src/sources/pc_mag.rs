use std::cell::RefCell;

use comrak::{
  arena_tree::Node,
  nodes::{
    Ast,
    AstNode,
    NodeValue,
  },
};

extern crate comrak;

use crate::content::{
  Content,
  Mode,
};

pub fn process<'a>(
  content: &mut Content,
  node:    &'a  AstNode<'a>,
) { traverse(content, node, &iterate) }

fn traverse<'a, F>(
  content: &mut Content,
  node:    &'a  AstNode<'a>,
  f:       &    F,
)
where F: Fn(&mut Content, &'a AstNode<'a>) {
  f(content, node);

  for c in node.children() { traverse(content, c, f) }
}

pub fn iterate(
  content: &mut Content,
  node:    &    Node<'_, RefCell<Ast>>,
) {
  match &node.data.borrow().value {
    NodeValue::Heading(_) => content.set(Mode::Heading),
    NodeValue::Paragraph  => {
      if !content.is_in(Mode::ImageCredit) {
        content.set(Mode::Paragraph)
      }
    }
    NodeValue::Link (l)   => content.add_link (&l.url),
    NodeValue::Image(i)   => content.add_image(&i.url),
    NodeValue::Text (t)   => {
      match content.mode() {
        Mode::Heading     => content.add_section  (t),
        Mode::Image       => content.add_caption  (t),
        Mode::ImageCredit => {
          if t.starts_with("(Credit: ") { content.add_credit   (t) }
          else                          { content.add_paragraph(t) }
        }
        Mode::Paragraph => content.add_paragraph(t),
        Mode::Text      => content.add_text     (t),
      }
    }
    _ => (),
  }
}