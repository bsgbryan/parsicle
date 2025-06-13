use url::Url;

use crate::{
  fetch::execute as fetch_article,
  parse_fail::ParseFail,
  sources::{
    cnn,
    huffpost,
    pcmag,
  },
};

pub async fn parse<'a>(url: &str) -> Result<(), ParseFail> {
  if let Ok  (url ) = Url::parse(url) &&
     let Some(html) = fetch_article(&url).await.ok()
  {
    if let Some(domain) = url.domain() {
      let result = match domain {
        "www.cnn.com"      =>      cnn::process(&html),
        "www.huffpost.com" => huffpost::process(&html),
        "www.pcmag.com"    =>    pcmag::process(&html),
        _ => {
          eprintln!("{domain} is not currently supported");
          Vec::with_capacity(0)
        }
      };

      for r in result { println!("{r}");}
    }
    else { return Err(ParseFail) }
  
    Ok(())
  }
  else { return Err(ParseFail) }
}
