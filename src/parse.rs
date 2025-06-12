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
  parse_fail::ParseFail,
};

pub async fn parse<'a>(url: &str) -> Result<Content, ParseFail> {
  if let Ok  ( url       ) = Url::parse(url) &&
     let Some((title, md)) = fetch_article(&url).await.ok()
  {
    // println!("{title}\n\n{md}");

    let     arena   = Arena::new();
    let     root    = parse_document(&arena, &md, &Options::default());
    let mut content = Content::new(&title);
  
    if let Some(domain) = url.domain() {
      match domain {
        "www.huffpost.com" => crate::sources::huffington_post::process(&mut content, root),
        "www.pcmag.com"    => crate::sources::pc_mag         ::process(&mut content, root),
        _ => eprintln!("{domain} is not currently supported")
      }
    }
    else { return Err(ParseFail) }

    // println!("{content}");
  
    Ok(content)
  }
  else { return Err(ParseFail) }
}
