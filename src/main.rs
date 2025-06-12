#![feature(let_chains)]

use parsicle::parse::parse;

#[tokio::main]
async fn main() {
    // let url = "https://time.com/6971144/campus-protests-professors-essay/";
    let url = "https://www.pcmag.com/reviews/samsung-galaxy-book4-ultra";
    let _parsed = parse(url).await.ok().unwrap();

    // println!("{parsed}");
  }
