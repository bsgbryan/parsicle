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
      Paragraph,
      self,
    },
  },
  author::Author,
  image::Image,
};

pub(crate) fn parse(href: &Url, html: &Html) -> Vec<Article> {
  match Selector::parse("main#main") {
    Ok(article) => {
      if let Some(article) = html.select(&article).next() &&     
          let Some(title)   = headline(html)
      {
        let out = vec![Article {
          alternate:   super::alternates_urls(html),
          authors:     authors(html),
          canonical:   href.clone(),
          content:     content(&article),
          description: super::description(html),
          hero_image:  hero(html),
          images:      images(&article),
          published:   super::published(html),
          title,
        }];

        return out
      }
    }
    Err(_) => eprintln!("Couldn't find article to parse")
  }
  Vec::with_capacity(0)
}

fn headline(context: &Html) -> Option<String> {
  match Selector::parse("main#main > header.container h1") {
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
  match Selector::parse("main#main > header.container div#author-byline a[data-module=\"author-byline\"]") {
    Ok(byline) => {
      for el in context.select(&byline) {
        if let Some(href) = el.value().attr("href") {
          match Url::parse(&format!("https://www.pcmag.com{}", href)) {
            Ok(href) => {
              if let Some(name) = el.value().attr("aria-label")           &&
                 let Some(name) = name.strip_suffix("'s Full Author Bio") &&
                 out.iter().find(|a: &&Author| a.href.as_ref().expect("No href for Author") == href.as_str()).is_none()
              { out.push(Author::new(name, href.as_str())); }
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

fn content(context: &ElementRef) -> Option<Vec<Content>> {
  match Selector::parse("section.container article[data-parent-group=\"content-body\"] > section > p, section.container article[data-parent-group=\"content-body\"] > section > button > h2, section.container article[data-parent-group=\"content-body\"] > section > button+div > p") {
    Ok(p) => {
      let mut out = vec![];
      for p in context.select(&p) {
        let text = p.text().collect::<Vec<_>>().join(" ");
        let text = text.trim();
        let text = text.replace("  ", " ");
        let tag  = p.value().name();

        if tag == "h2" { out.push(Heading  (text)); }
        else           { out.push(Paragraph(text)); }
      }
      Some(out)
    }
    Err(_) => None
  }
}

fn images(context: &ElementRef) -> Option<Vec<Image>> {
  match Selector::parse("section > div[x-show=\"show\"]") {
    Ok(image) => {
      let mut out = vec![];      
      for i in context.select(&image) {
        if let Ok (img) = Selector::parse("img") {
          for img in i.select(&img) {
            let mut src    = String::new();
            let mut alt    = String::new();
            let mut credit = None;

            if let Some(s)   = img.value().attr("data-image-loader") &&
                let Some(a)   = img.value().attr("alt")
            {
              src = s.to_string();
              alt = a.to_string();
            }
      
            if let Ok  (c) = Selector::parse("small")                                         &&
               let Some(c) = i.select(&c).next()                                              &&
               let Some(t) = c.text().collect::<Vec<_>>().join(" ").strip_prefix("(Credit: ") &&
               let Some(t) = t.strip_suffix(")")
            { credit = Some(t.to_string()); }
  
            if let Some(credit) = credit {
              out.push(Image::new(&alt, &credit, &src));
            }
          }
        }
      }

      if out.len() > 0 { Some(out) }
      else             { None      }
    }
    Err(_) => None
  }
}
