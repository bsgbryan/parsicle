use article_scraper::ArticleScraper;
use html2md::parse_html;
use url::Url;
use reqwest::Client;
use comrak::{parse_document, Arena, Options};
use comrak::nodes::{AstNode, NodeValue};

extern crate comrak;

#[derive(Debug)]
pub struct ParsedArticle {
    pub title: String,
    pub content: Vec<String>,
    // published: Option<String>,
    // author: Option<String>,
    pub links: Vec<String>,
}

pub async fn parse(url: &str) -> Result<ParsedArticle, Box<dyn std::error::Error>> {
  // let src = sources();

  let scraper = ArticleScraper::new(None);
  let url = Url::parse(url);
  let client = Client::new();
  let article = scraper.await.parse(&url.unwrap(), false, &client, None).await.unwrap();
  let md = parse_html(article.html.unwrap().as_str());
  
  let arena = Arena::new();

  let root = parse_document(
      &arena,
      &md,
      &Options::default(),
  );

  fn iter_nodes<'a, F>(ignor: &mut bool, bf: &mut Vec<String>, tok: &mut Vec<String>, lnks: &mut Vec<String>, node: &'a AstNode<'a>, f: &F)
      where F : Fn(&mut bool, &mut Vec<String>, &mut Vec<String>, &mut Vec<String>, &'a AstNode<'a>) {
      f(ignor, bf, tok, lnks, node);
      for c in node.children() {
          iter_nodes(ignor, bf, tok, lnks, c, f);
      }
  }

  let mut buf = vec![];
  let mut paragraphs = vec![];
  let mut links = vec![];
  let mut ignore = false;

  iter_nodes(&mut ignore, &mut buf, &mut paragraphs, &mut links, root, &|ignore, buf, tokens, links, node| {
      match node.data.borrow().value {
          NodeValue::Paragraph => {
              for c in node.children() {
                  match c.data.borrow().value {
                      NodeValue::Text(ref text) => {
                          if text.to_lowercase().starts_with("read more") {
                              *ignore = true;
                          } else {
                              // println!("TEXT:\"{}\"", text.trim());

                              buf.push(text.trim().to_string());
              
                              if text.ends_with(".") {
                                  tokens.push(buf.concat());
                                  buf.clear();
                              }
                          }
                      }
                      NodeValue::Link(ref link) => {
                          if *ignore == false {
                              links.push(link.url.clone());

                              for c in c.children() {
                                  match c.data.borrow().value {
                                      NodeValue::Text(ref text) => {
                                          // println!("BUF:\"{}\"", text);
          
                                          if text.len() > 0 {
                                              buf.push(text.trim().to_string());
          
                                              if text.ends_with(".") {
                                                  tokens.push(buf.concat());
                                                  buf.clear();
                                              }
                                          }
                                      }
                                      _ => ()
                                  }
                              }
                          } else {
                              *ignore = false;
                          }
                      }
                      _ => ()
                  }
              }
          }
          NodeValue::Heading(_) => (),
          _ => (),
      }
  });

  Ok(ParsedArticle {
      title: article.title.unwrap(),
      content: paragraphs,
      // published: None,
      // author: article.author,
      links,
  })
}
