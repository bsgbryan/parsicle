// mod lib;

use article_scraper::ArticleScraper;
use html2md::parse_html;
use url::Url;
use reqwest::Client;

extern crate comrak;
use comrak::{parse_document, Arena, Options};
use comrak::nodes::{AstNode, NodeValue};

use rust_bert::pipelines::ner::NERModel;

// use crate::lib::sources;

#[derive(Debug)]
struct ParsedArticle {
    title: String,
    content: Vec<String>,
    published: Option<String>,
    author: Option<String>,
    links: Vec<String>,
}

fn nlp(input: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let ner_model = NERModel::new(Default::default())?;
    let output = ner_model.predict(&input);

    println!("{:#?}", output);

    Ok(())
}

async fn test_parse() {
    // let src = sources();

    let scraper = ArticleScraper::new(None);
    let url = Url::parse("https://time.com/6971144/campus-protests-professors-essay/");
    let client = Client::new();
    let article = scraper.await.parse(&url.unwrap(), false, &client, None).await.unwrap();
    let md = parse_html(article.html.unwrap().as_str());
    
    let arena = Arena::new();

    let root = parse_document(
        &arena,
        &md,
        &Options::default());

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

    let parsed = ParsedArticle {
        title: article.title.unwrap(),
        content: paragraphs,
        published: None,
        author: article.author,
        links,
    };

    println!("{:#?}", parsed);

    let blocking_task = tokio::task::spawn_blocking(|| {
        let _ = nlp(parsed.content);
    });

    blocking_task.await.unwrap();
}

#[tokio::main]
async fn main() {
    test_parse().await;
}
