use std::error::Error;

use article_scraper::ArticleScraper;
use html2md::parse_html;
use reqwest::Client;
use url::Url;

pub async fn execute(
  url: &Url
) -> Result<(String, String), Box<dyn Error>> {
  let scraper = ArticleScraper::new(None);
  let client = Client::new();
  let article = scraper.
    await.
    parse(url, false, &client, None).
    await.
    unwrap();

  Ok({
    if let Some(html) = article.html { (article.title.unwrap(), parse_html(html.as_str())) }
    else                             { ("".to_string(),         "".to_string())            }
  })
}