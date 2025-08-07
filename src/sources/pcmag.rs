use chrono::{
  DateTime,
  Utc,
};
use scraper::{
  Html,
  Selector,
};
use url::Url;

use crate::article::Article;

mod comparison;
mod news;
mod review;

pub fn process<'a>(html: &'a str) -> Vec<Article> {
  let document = Html::parse_document(html);
  let kind     = Selector::parse("html > head > meta[property=\"og:type\"]").ok();

  let kind = match kind {
    Some(kind) =>  match document.select(&kind).next() {
      Some(kind) => match kind.value().attr("content") {
        Some(kind) => kind,
        None       => "unknown",
      }
      None => "unknown",
    }
    None => "unknown",
  };

  match kind {
    "article" => {
      if let Some(    href   ) = canonical_url(&document) &&
         let Some(mut path   ) = href.path_segments()     &&
         let Some(    section) = path.next()
      {
        match section {
          "comparisons" =>   comparison::parse(&href, &document),
          "news"        =>         news::parse(&href, &document),
          "opinions"    =>         news::parse(&href, &document),
          "reviews"     =>       review::parse(&href, &document),
          _ => Vec::with_capacity(0)
        }
      }
      else {
      	#[cfg(debug_assertions)]
        eprintln!("No canonical url found");

        Vec::with_capacity(0)
      }
    }
    _ => {
    	#[cfg(debug_assertions)]
      eprintln!("{kind} is an unsupported content type for the PCMAG source");

      Vec::with_capacity(0)
    }
  }
}

fn canonical_url(context: &Html) -> Option<Url> {
  match Selector::parse("html > head > link[rel=canonical]") {
    Ok(curl) => {
      if let Some(url) = context.select(&curl).next() &&
        let Some(url) = url.value().attr("href")
      {
        match Url::parse(url) {
          Ok(u)  => Some(u),
          Err(_) => None,
        }
      }
      else { None }
    }
    Err(_) => None,
  }
}

fn description(context: &Html) -> Option<String> {
  match Selector::parse("html > head > meta[name=description]") {
    Ok(d) => {
      if let Some(desc) = context.select(&d).next() &&
        let Some(desc) = desc.value().attr("content")
      { Some(desc.to_string()) }
      else { None }
    }
    Err(_) => None,
  }
}

fn alternates_urls(context: &Html) -> Option<Vec<(String, Url)>> {
  match Selector::parse("html > head > link[rel=alternate]") {
    Ok(alt) => {
      let mut out = vec![];
      for a in context.select(&alt) {
        if let Some(lang) = a.value().attr("hreflang") &&
          let Some(href) = a.value().attr("href")     &&
          let Ok  (url)  = Url::parse(href)
        { out.push((lang.to_string(), url)) }
      }
      if out.len() > 0 { Some(out) }
      else             { None      }
    }
    Err(_) => None
  }
}

fn published(context: &Html) -> Option<DateTime<Utc>> {
  let published = Selector::parse("html > head > meta[name=\"article:published_time\"]").ok();
  if let Some(published) = published &&
    let Some(time) = context.select(&published).next() &&
    let Some(time) = time.value().attr("content")
  { return time.parse::<DateTime<Utc>>().ok() }

  None
}
