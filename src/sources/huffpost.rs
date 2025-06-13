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
  article::{
    Article,
    Content::{
      Paragraph,
      self,
    },
  },
  author::Author,
  image::Image,
};

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

  let mut out = vec![];

  match kind {
    "article" => {
      match Selector::parse("article.entry__content") {
        Ok(article) => {
          if let Some(article) = document.select(&article).next() &&
             let Some(href)    = canonical_url(&document)         &&
             let Some(title)   = headline(&document)
          {
            out.push(Article {
              alternate:   alternates_urls(&document),
              authors:     authors        (&document),
              canonical:   href,
              content:     content     (&article),
              description: description (&document),
              hero_image:  hero        (&document),
              images:      images      (&article),
              published:   published   (&document),
              title,
            });
          }
        }
        Err(_) => eprintln!("Couldn't find article to parse")
      }
    }
    _ => eprintln!("{kind} is an unsupported content type for the HuffPost source")
  }

  out
}

fn headline(context: &Html) -> Option<String> {
  match Selector::parse("#main header h1.headline") {
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
  match Selector::parse("header.entry__header .entry__byline .entry__byline__author a") {
    Ok(byline) => {
      for el in context.select(&byline) {
        if let Some(href) = el.value().attr("href") {
          match Url::parse(href) {
            Ok(href) => {
              let name = el.text().collect::<Vec<_>>().join(" ");
              if name.len() > 0 { out.push(Author { href, name }); }
            }
            Err(_) => ()
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

fn hero(context: &Html) -> Option<Url> {
  match Selector::parse("html > head > meta[property=\"og:image\"]") {
    Ok(curl) => {
      if let Some(url) = context.select(&curl).next() &&
         let Some(url) = url.value().attr("content")
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
  let published = Selector::parse("html > head > meta[property=\"article:published_time\"]").ok();
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

fn content(context: &ElementRef) -> Option<Vec<Content>> {
  match Selector::parse("article section.entry__content-list div.cli-text p") {
    Ok(p) => {
      let mut out = vec![];
      for p in context.select(&p) {
        let text = p.text().collect::<Vec<_>>().join(" ");
        let text = text.trim();
        let text = text.replace("  ", " ");
 
        out.push(Paragraph(text));
      }
      Some(out)
    }
    Err(_) => None
  }
}

fn images(context: &ElementRef) -> Option<Vec<Image>> {
  match Selector::parse("div.image") {
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
