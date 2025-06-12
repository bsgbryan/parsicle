#![feature(let_chains)]

use parsicle::parse::parse;

#[tokio::main]
async fn main() {
  // parse("https://www.pcmag.com/reviews/samsung-galaxy-book4-ultra").await.ok().unwrap();
  parse("https://www.huffpost.com/entry/trump-les-miserables-protest_n_6849faf0e4b03de6beafe896").await.ok().unwrap();
}
