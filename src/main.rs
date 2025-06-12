#![feature(let_chains)]

use parsicle::parse::parse;

#[tokio::main]
async fn main() {
  parse("https://www.pcmag.com/reviews/samsung-galaxy-book4-ultra").await.ok().unwrap();
}
