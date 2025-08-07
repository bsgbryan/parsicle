use url::Url;

use crate::{
  article::Article,
  fetch::execute as fetch_article,
  sources::{
    cnn,
    huffpost,
    pcmag,
  }
};

pub async fn parse<'a>(url: &str) -> Vec<Article> {
  let mut out = vec![];
  if let Ok  (url ) = Url::parse(url) &&
     let Some(html) = fetch_article(&url).await.ok()
  {
    if let Some(domain) = url.domain() {
      let result = match domain {
        "www.cnn.com"      =>      cnn::process(&html),
        "www.huffpost.com" => huffpost::process(&html),
        "www.pcmag.com"    =>    pcmag::process(&html),
        _ => {
        	#[cfg(debug_assertions)]
          eprintln!("{domain} is not currently supported");

          Vec::with_capacity(0)
        }
      };

      for r in result { out.push(r); }
    }
  }
  out
}
