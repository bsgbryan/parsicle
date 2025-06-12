use article_scraper::ArticleScraper;
use html2md::parse_html;
use reqwest::Client;
use url::Url;

use crate::parse_fail::ParseFail;

pub async fn execute(
  url: &Url
) -> Result<(String, String), ParseFail> {
  let scraper = ArticleScraper::new(None);
  
  if let Ok(article) = scraper.await.parse(url, false, &Client::new(), None).await {
    if let Some(html) = article.html { Ok((article.title.unwrap(), parse_html(html.as_str()))) }
    else                             { Err(ParseFail) }
  }
  else { Err(ParseFail) }
}