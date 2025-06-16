use parsicle::parse::parse;

#[tokio::main]
async fn main() {
  // parse("https://www.cnn.com/2025/06/11/politics/gavin-newsom-democrats-fight-trump").await.ok().unwrap();
  // parse("https://www.cnn.com/us/live-news/la-protests-ice-raids-trump-06-11-25").await.ok().unwrap();
  // parse("https://www.cnn.com/cnn-underscored/reviews/macbook-air-m4").await.ok().unwrap();
  // parse("https://www.huffpost.com/entry/alex-padilla-arrest_n_684b288fe4b03a16ecb3462f").await.ok().unwrap();
  let articles = parse("https://www.pcmag.com/opinions/did-liquid-glass-give-us-sneak-peek-at-apple-smart-glasses-wwdc-2025").await;
  // let articles = parse("https://www.pcmag.com/comparisons/bitdefender-vs-mcafee-which-premium-antivirus-protects-your-data-better").await;

  for a in articles { println!("{a}"); }
}
