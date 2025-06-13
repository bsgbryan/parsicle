use std::fmt::Error;
use url::Url;

pub async fn execute(
  url: &Url
) -> Result<String, Error> {
  let html = reqwest::get(url.as_str())
    .await
    .expect("Failed making get request")
    .text()
    .await
    .expect("Failed extracting text");

  Ok(html)
}