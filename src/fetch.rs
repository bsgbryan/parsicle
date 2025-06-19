use std::time::Duration;

use reqwest::{
  Client,
  Error,
};
use url::Url;

pub async fn execute(
  url: &Url
) -> Result<String, Error> {
  Ok(CLIENT.get(url.as_str()).send().await?.text().await?)
}

lazy_static! {
  pub static ref CLIENT: Client = {
    let client = Client::builder()
      .timeout(Duration::from_secs(5))
      .user_agent("Mozilla/5.0 (compatible; Readiebot/1.0; +https://readie.news/bot)")
      .build();

    match client {
      Ok (c) => c,
      Err(_) => panic!("Oops"),
    }
  };
}