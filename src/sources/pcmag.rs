use chrono::{
  DateTime,
  Utc,
};
use scraper::{
  ElementRef,
  Html,
  Selector,
};
use url::Url;

use crate::{
  article::Article,
  author::Author,
  image::Image,
};

pub fn process<'a>(html: &'a str) -> Vec<Article> {
  let document = Html::parse_document(html);
  let kind     = Selector::parse("html > head > meta[property=\"og:type\"]").ok();

  let kind = match kind {
    Some(kind) => document.
      select(&kind)
      .next()
      .unwrap()
      .value()
      .attr("content")
      .unwrap(),
    None => "unknown",
  };

  println!("KIND: {kind}");

  let mut out = vec![];

  match kind {
    "live-story" => {}
    "article" => {
      match Selector::parse("article#article") {
        Ok(article) => {
          if let Some(article) = document.select(&article).next() &&
             let Some(href)    = canonical_url(&document)         &&
             let Some(title)   = headline(&document)
          {
            out.push(Article {
              authors:     authors    (&document),
              content:     paragraphs (&article),
              description: description(&document),
              href,
              images:    images   (&article),
              published: published(&document),
              title,
            });
          }
        }
        Err(_) => ()
      }
    }
    _ => ()
  }

  out
}

fn headline(context: &Html) -> Option<String> {
  match Selector::parse("main#main header#content-header h1") {
    Ok(h) => {
      if let Some(head) = context.select(&h).next() {
        let text = head.text().collect::<Vec<_>>().join(" ");
        let text = text.trim();
        let text = text.replace("  ", " ");
        Some(text)
      }
      else { None }
    }
    Err(_) => None,
  }
}

fn authors(context: &Html) -> Vec<Author> {
  let mut out = vec![];
  match Selector::parse("header#content-header > div > div > div > a[data-module=\"author-byline\"]") {
    Ok(byline) => {
      for el in context.select(&byline) {
        if let Some(href) = el.value().attr("href") {
          match Url::parse(&format!("https://www.pcmag.com{}", href)) {
            Ok(href) => {
              if let Some(name) = el.value().attr("aria-label") {
                out.push(Author { href, name: name.to_string() });
              }
            }
            Err(e) => eprintln!("{e:?}")
          }
        }
      }
    }
    Err(_) => ()
  }
  out
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

fn published(context: &Html) -> Option<DateTime<Utc>> {
  let published = Selector::parse("html > head > meta[name=\"article:published_time\"]").ok();
  if let Some(published) = published &&
     let Some(time) = context.select(&published).next() &&
     let Some(time) = time.value().attr("content")
  { return time.parse::<DateTime<Utc>>().ok() }

  None
}

fn _modified(context: &Html) -> Option<DateTime<Utc>> {
  let modified = Selector::parse("html > head > meta[property=\"article:modified_time\"]").ok();
  if let Some(modified) = modified &&
     let Some(time) = context.select(&modified).next() &&
     let Some(time) = time.value().attr("content")
  { return time.parse::<DateTime<Utc>>().ok() }

  None
}

fn paragraphs(context: &ElementRef) -> Option<Vec<String>> {
  match Selector::parse("article#article > p") {
    Ok(p) => {
      let mut out = vec![];
      for p in context.select(&p) {
        let text = p.text().collect::<Vec<_>>().join(" ");
        let text = text.trim();
        let text = text.replace("  ", " ");
 
        out.push(text);
      }
      Some(out)
    }
    Err(_) => None
  }
}

fn images(context: &ElementRef) -> Option<Vec<Image>> {
  match Selector::parse("section > div > div.article-image > img") {
    Ok(image) => {
      let mut out = vec![];      
      for i in context.select(&image) {
        let mut src    = String::new();
        let mut alt    = String::new();
        let mut credit = None;

        if let Ok  (img) = Selector::parse("picture.image__picture > img") &&
           let Some(img) = i.select(&img).next()                           &&
           let Some(s)   = img.value().attr("src")                         &&
           let Some(a)   = img.value().attr("alt")
        {
          src = s.to_string();
          alt = a.to_string();
        }
  
        if let Ok   (c) = Selector::parse("figcaption.image__credit") &&
            let Some(c) = i.select(&c).next()
        {
          let text = c.text().collect::<Vec<_>>().join(" ");
          credit = Some(text);
        }

        if let Some(credit) = credit &&
           let Some(href)   = Url::parse(&src).ok()
        { out.push(Image { href, caption: alt, credit }) }
      }

      if out.len() > 0 { Some(out) }
      else             { None      }
    }
    Err(_) => None
  }
}
