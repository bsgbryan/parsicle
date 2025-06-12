use comrak::{
  Arena,
  Options,
  parse_document,
};
use url::Url;

extern crate comrak;

use crate::{
  content::Content,
  fetch::execute as fetch_article,
  parsed_article::ParsedArticle,
};

pub async fn parse<'a>(url: &str) -> Result<ParsedArticle, Box<dyn std::error::Error>> {
  if let Ok  ( url       ) = Url::parse(url) &&
     let Some((title, md)) = fetch_article(&url).await.ok()
  {
    let     arena   = Arena::new();
    let     root    = parse_document(&arena, &md, &Options::default());
    let mut content = Content::new(&title);
  
    if let Some(domain) = url.domain() {
      match domain {
        "www.pcmag.com" => crate::sources::pc_mag::process(&mut content, root),
        _ => ()
      }
    }

    println!("{content}");
  
    Ok(ParsedArticle::new(&content))
  }
  else { Ok(ParsedArticle::empty()) }
}
