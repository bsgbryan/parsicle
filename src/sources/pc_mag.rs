use comrak::nodes::AstNode;

use crate::content::Content;

pub fn process<'a, F>(
  content: &mut Content,
  node: &'a AstNode<'a>,
  f: &F,
) where F: Fn(&mut Content, &'a AstNode<'a>) {
  f(content, node);

  for c in node.children() { process(content, c, f); }
}