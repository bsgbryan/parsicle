use std::fmt::Display;

use chrono::{
  DateTime,
  Utc,
};
use url::Url;

use crate::{
  author::Author,
  href::sanitize,
  image::Image,
};

#[derive(Debug)]
pub enum Content {
  Heading(String),
  Image(Option<Image>),
  Paragraph(String),
  Subheading(String),
}

#[derive(Debug)]
pub struct Article {
  pub alternate:   Option<Vec<(String, Url)>>,
  pub authors:     Vec<Author>,
  pub canonical:   Url,
  pub content:     Option<Vec<Content>>,
  pub description: Option<String>,
  pub hero_image:  Option<Url>,
  pub images:      Option<Vec<Image>>,
  pub published:   Option<DateTime<Utc>>,
  pub title:       String,
}

impl Display for Article {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "{}", self.markdown_with_frontmatter())
  }
}

impl Article {
  pub fn frontmatter(&self) -> String {
    let mut out = String::new();

    out += "---\n";

    if let Some(published) = &self.published {
      out += &format!("published: {}\n", published.to_rfc2822());
    }

    if let Some(desc) = &self.description {
      out += &format!("summary: {desc}\n");
    }

    out += "authors:\n";

    for a in &self.authors {
      if let Some(url) = &a.href {
        out += &format!("  - {}: {url}\n", &a.name);
      }
    }

    if let Some(img) = &self.hero_image {
      out += &format!("hero: {img}\n");
    }

    if let Some(url) = sanitize(&self.canonical.as_str()) {
      out += &format!("canonical_url: {url}\n");
    }

    if let Some(alternates) = &self.alternate {
      out += "alternate_urls:\n";

      for (lang, url) in alternates {
        out += &format!("  - {lang}: {url}\n");
      }
    }

    out += "---\n\n";

    out
  }

  pub fn markdown(&self) -> String {
    let mut out = format!("# {} #\n\n", self.title);
  
    if let Some(content) = &self.content {
      for c in content {
        match c {
          Content::Heading(h) => { out += &format!("## {h} ##\n\n")   }
          Content::Image  (i) => {
            if let Some(img) = i &&
               let Some(url) = &img.href
            { out += &format!("![{}; credit: {}]({url})\n\n", img.caption, img.credit); }
          }
          Content::Subheading(s) => { out += &format!("### {s} ###\n\n") }
          Content::Paragraph (p) => { out += &format!("{p}\n\n")         }
        }
      }
    }

    out
  }

  pub fn markdown_with_frontmatter(&self) -> String {
    format!("{}{}", &self.frontmatter(), &self.markdown())
  }
}

impl Article {
  pub fn html(&self) -> String {
    let mut out = String::new();

    out += "<article>\n";

    if let Some(url) = sanitize(&self.canonical.as_str()) {
      out += &format!("<h1>\n<a href=\"{url}\">{}</a>\n</h1>\n", &self.title);
    }
    else {
      out += &format!("<h1>{}</h1>\n", &self.title);
    }

    if let Some(desc) = &self.description {
      out += &format!("<p id=\"summary\">{desc}</p>\n");
    }

    out += "<section>\n";

    if let Some(published) = &self.published {
      out += &format!("<time id=\"published\" datetime=\"{}\">{}</time>\n",
        &published.to_rfc3339(),
        &published.to_rfc2822(),
      );
    }

    out += "<ul id\"authors\">\n";
    for a in &self.authors {
      if let Some(url) = &a.href {
        out += &format!("<li>\n<a href=\"{url}\">{}</a>\n</li>\n", &a.name);
      }
    }
    out += "</ul>\n";

    if let Some(img) = &self.hero_image &&
       let Some(url) = sanitize(img.as_str())
    { out += &format!("<img id=\"hero\" src=\"{}\">\n", url); }

    if let Some(alternates) = &self.alternate {
      out += "<ul class\"alternate-urls\">\n";
      for (lang, url) in alternates {
        out += &format!("<li>\n<a href=\"{url}\">{lang}</a>\n</li>\n");
      }
      out += "</ul>\n";
    }

    out += "</section>\n";
    out += "<article id=\"content\">\n";

    if let Some(content) = &self.content {
      for c in content {
        match c {
          Content::Heading(h)    => { out += &format!("<h2>{h}</h2>\n") }
          Content::Image(i) => {
            if let Some(img) = i &&
               let Some(url) = &img.href
            {
              out += "<figure>\n";
              out += &format!("<img src=\"{url}\" alt=\"{}\">\n", img.caption);
              out += "<figcaption>\n";
              out += &format!("<p>{}</p>\n", img.caption);
              out += &format!("<i clas=\"credit\">{}</i>\n", img.credit);
              out += "</figcaption>\n";
              out += "</figure>\n";
            }
          }
          Content::Subheading(s) => { out += &format!("<h3>{s}</h3>\n") }
          Content::Paragraph(p)  => { out += &format!("<p>{p}</p>\n")   }
        }
      }
    }

    out += "</article>\n";
    out += "</article>\n";

    out
  }
}