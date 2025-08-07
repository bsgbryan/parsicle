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
      Heading,
      Image as ContentImage,
      Paragraph,
      Subheading,
      self,
    },
  },
  author::Author,
  image::Image,
};

pub fn process<'a>(html: &'a str) -> Vec<Article> {
  let document = Html::parse_document(html);
  let kind     = Selector::parse("html > head > meta[name=type]").ok();

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
    "live-story" => {
      let post = Selector::parse("article.live-story-post").ok();
      if let Some(post) = post {
        for el in document.select(&post) {        
          if let Ok  (title) = Selector::parse("h2.live-story-post__headline") &&
             let Some(title) = el.select(&title).next()                        &&
             let Some(href)  = canonical_url(&document)
          {
            let title = title.text().collect::<Vec<_>>().join(" ");

            if title.len() > 0 {
              if let Some(at) = el.value().attr("data-last-updated") {
                let published = match DateTime::parse_from_rfc2822(at) {
                  Ok(at) => Some(at.to_utc()),
                  Err(_) => None,
                };

                out.push(Article {
                  alternate:   alternates_urls(&document),
                  authors:     authors        (&document),
                  canonical:   href,
                  content:     content(&el),
                  description: None,
                  hero_image:  hero  (&document),
                  images:      images(&el),
                  published,
                  title,
                })
              }
            }
          }
        }
      }
    }
    "article" => {
      match Selector::parse("main.article__main") {
        Ok(article) => {
          if let Some(article) = document.select(&article).next() &&
             let Some(href)    = canonical_url(&document)         &&
             let Some(title)   = headline(&document)
          {
            out.push(Article {
              alternate:   alternates_urls(&document),
              authors:     authors        (&document),
              canonical:   href,
              content:     content  (&article),
              description: description (&document),
              hero_image:  hero        (&document),
              images:      images      (&article),
              published:   published   (&document),
              title,
            });
          }
        }
        Err(_) => {
	        #[cfg(debug_assertions)]
	        eprintln!("Couldn't find article to parse")
        }
      }
    }
    _ => {
    	#[cfg(debug_assertions)]
	    eprintln!("{kind} is an unsupported content type for the CNN source")
    }
  }

  out
}

fn headline(context: &Html) -> Option<String> {
  match Selector::parse("div.headline h1.headline__text") {
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
  match Selector::parse("a.byline__link") {
    Ok(byline) => {
      for el in context.select(&byline) {
        if let Some(href) = el.value().attr("href") {
          let name = el.text().collect::<Vec<_>>().join(" ");
          out.push(Author::new(&name, href));
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

fn content(context: &ElementRef) -> Option<Vec<Content>> {
  match Selector::parse("p.paragraph, p.pull-quote_block-quote__text, h2.subheader:not(:has(+ div > table)), h3.subheader, div.image") {
    Ok(p) => {
      let mut out = vec![];
      for p in context.select(&p) {
        let text = p.text().collect::<Vec<_>>().join(" ");
        let text = text.trim();
        let text = text.replace("  ", " ");
        let tag  = p.value().name();
 
        if      tag == "h2"  { out.push(Heading     (text     )); }
        else if tag == "h3"  { out.push(Subheading  (text     )); }
        else if tag == "div" { out.push(ContentImage(image(&p))); }
        else                 { out.push(Paragraph   (text     )); }
      }
      Some(out)
    }
    Err(_) => None
  }
}

fn images(context: &ElementRef) -> Option<Vec<Image>> {
  match Selector::parse("div.image") {
    Ok(img) => {
      let mut out = vec![];      
      for i in context.select(&img) {
        if let Some(img) = image(&i) { out.push(img) }
      }

      if out.len() > 0 { Some(out) }
      else             { None      }
    }
    Err(_) => None
  }
}

fn image(context: &ElementRef) -> Option<Image> {
  let mut src    = String::new();
  let mut alt    = String::new();
  let mut credit = None;

  if let Ok  (img) = Selector::parse("picture.image__picture > img") &&
     let Some(img) = context.select(&img).next()                     &&
     let Some(s)   = img.value().attr("src")                         &&
     let Some(a)   = img.value().attr("alt")
  {
    src = s.to_string();
    alt = a.to_string();
  }

  if let Ok  (c) = Selector::parse("figcaption.image__credit") &&
     let Some(c) = context.select(&c).next()
  {
    let text = c.text().collect::<Vec<_>>().join(" ");
    credit = Some(text);
  }

  if let Some(credit) = credit {
    Some(Image::new(&alt, &credit, &src))
  }
  else { None }
}
