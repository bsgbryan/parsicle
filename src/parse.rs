use article_scraper::ArticleScraper;
use comrak::nodes::{AstNode, NodeValue};
use comrak::{parse_document, Arena, Options};
use html2md::parse_html;
use reqwest::Client;
use std::cell::{RefCell, RefMut};
use std::error::Error;
use url::Url;

extern crate comrak;

#[derive(Debug)]
pub struct ParsedArticle {
    pub content: Vec<Section>,
    pub images: Vec<Image>,
    pub links: Vec<String>,
    pub lists: Vec<List>,
    pub title: String,
}

#[derive(Debug, Clone)]
struct Paragraph {
    tokens: Vec<String>,
}

impl Paragraph {
    fn new(text: &str) -> Self {
        Paragraph {
            tokens: vec![text.to_string()],
        }
    }
}

#[derive(Debug, Clone)]
pub struct Image {
    caption: Option<String>,
    title: Option<String>,
    url: String,
}

impl Image {
    fn new(url: String) -> Self {
        Image {
            caption: None,
            title: None,
            url,
        }
    }
}

#[derive(Debug, Clone)]
pub struct List {
    items: Vec<String>,
}

impl List {
    fn new() -> Self {
        List {
            items: vec![],
        }
    }
}

#[derive(Debug, Clone)]
pub struct Section {
    pub content: Vec<String>,
    pub heading: String,
    paragraphs: Vec<RefCell<Paragraph>>,
}

impl Section {
    fn new(text: &str) -> Self {
        Section {
            content: vec![],
            heading: text.to_string(),
            paragraphs: vec![RefCell::new(Paragraph {
                tokens: vec![],
            })],
        }
    }
}

#[derive(Debug)]
struct Content {
    ignore: bool,
    new_heading: bool,
    new_image: bool,
    new_list_item: bool,
    new_paragraph: bool,
    images: Vec<RefCell<Image>>,
    links: Vec<String>,
    lists: Vec<RefCell<List>>,
    sections: Vec<RefCell<Section>>,
}

impl Content {
    fn new() -> Self {
        Content {
            ignore: false,
            images: vec![],
            new_heading: false,
            new_image: false,
            new_paragraph: false,
            new_list_item: false,
            links: vec![],
            lists: vec![],
            sections: vec![RefCell::new(Section {
                content: vec![],
                heading: String::new(),
                paragraphs: vec![RefCell::new(Paragraph {
                    tokens: vec![],
                })],
            })],
        }
    }
}

fn iter_nodes<'a, F>(content: &mut Content, node: &'a AstNode<'a>, f: &F)
where
    F: Fn(&mut Content, &'a AstNode<'a>),
{
    f(content, node);

    for c in node.children() {
        iter_nodes(content, c, f);
    }
}

fn append_text(content: &mut Content, text: &str) {
    if text.split(" ").count() > 1 {
        let mut section = content.
            sections.
            last().
            unwrap().
            borrow_mut();
    
        let paras: &mut Vec<RefCell<Paragraph>> = section.
            paragraphs.
            as_mut();
    
        if text.to_lowercase().starts_with("read more") {
            content.ignore = true;
        } else {
            paras.
                last().
                unwrap().
                borrow_mut().
                tokens.
                push(text.to_string());
        }
    }
}

fn finish(section: &mut RefMut<Section>) {
    section.content = section.
        paragraphs.
        iter().
        filter(|p| {
            let tokens = &p.borrow().tokens;

            tokens.len() > 1 || (tokens.len() > 0 && tokens.last().unwrap().ends_with("."))
        }).
        map(|p| p.
            borrow().
            tokens.
            concat()
        ).
        filter(|c| c.len() > 0).
        collect();

    section.paragraphs.clear();
}

fn create_new_section(content: &mut Content, text: &str) {
    content.sections.push(RefCell::new(Section::new(text)));

    content.new_heading = false;

    finish(&mut content.
        sections.
        get(content.sections.len() - 2).
        unwrap().
        borrow_mut()
    );
}

fn create_new_paragraph(content: &mut Content, text: &str) {
    if text.split(" ").count() > 1 {
        content.
            sections.
            last().
            unwrap().
            borrow_mut().
            paragraphs.
            push(RefCell::new(Paragraph::new(text))
        );
    }

    content.new_paragraph = false;
}

async fn fetch_and_parse(url: &str) -> Result<(String, String), Box<dyn Error>> {
    let scraper = ArticleScraper::new(None);
    let url = Url::parse(url);
    let client = Client::new();
    let article = scraper.
        await.
        parse(&url.unwrap(), false, &client, None).
        await.
        unwrap();

    Ok((article.title.unwrap(), parse_html(article.html.unwrap().as_str())))
}

pub async fn parse<'a>(url: &str) -> Result<ParsedArticle, Box<dyn std::error::Error>> {
    let (title, md) = fetch_and_parse(url).
        await.
        ok().
        unwrap();

    let arena = Arena::new();
    let root = parse_document(&arena, &md, &Options::default());

    let mut content = Content::new();

    iter_nodes(
        &mut content,
        root,
        &|
            content,
            node,
        | match &node.data.borrow().value {
            NodeValue::Image(img) => {
                // println!("IMAGE: {} -> {}", img.title, img.url);
                content.images.push(RefCell::new(Image::new(img.url.clone())));
                content.new_image = true;
                content.new_paragraph = false;
            }
            NodeValue::Heading(_) => {
                // println!("HEADING");
                content.new_heading = true;
            }
            NodeValue::Text(ref text) => {
                // println!("TEXT: {}", text);
                if content.new_heading {
                    create_new_section(content, text);
                } else {
                    if content.new_paragraph {
                        if content.new_image {
                            let mut image = content.
                                images.
                                last().
                                unwrap().
                                borrow_mut();

                            image.caption = Some(text.to_string());

                            content.new_image = false;
                        } else if content.new_list_item {
                            let mut list = content.
                                lists.
                                last().
                                unwrap().
                                borrow_mut();

                            list.items.push(text.to_string());

                            content.new_list_item = false;
                        } else {
                            create_new_paragraph(content, &text);
                        }
                    } else {
                        if content.new_image {
                            let mut image = content.
                                images.
                                last().
                                unwrap().
                                borrow_mut();

                            image.title = Some(text.to_string());
                        } else {
                            append_text(content, text);
                        }
                    }
                }
            }
            NodeValue::List(_) => {
                content.new_image = false;

                content.lists.push(RefCell::new(List::new()));

                // println!("LIST");
            }
            NodeValue::Item(item) => {
                // println!("ITEM: {:#?}", item.list_type);

                content.new_list_item = true;
            }
            NodeValue::Link(ref link) => {
                if content.ignore == false {
                    content.links.push(link.url.clone());
                } else {
                    content.ignore = false;
                }
            }
            NodeValue::Paragraph => {
                // println!("PARAGRAPH");
                content.new_paragraph = true;
            }
            _ => (),
        },
    );

    finish(&mut content.
        sections.
        last().
        unwrap().
        borrow_mut()
    );

    Ok(ParsedArticle {
        content: content.
            sections.
            iter().
            map(|s| s.
                borrow().
                clone()
            ).
            collect(),
        images: content.
            images.
            iter().
            map(|i| i.
                borrow().
                clone()
            ).
            collect(),
        links: content.links,
        lists: content.
            lists.
            iter().
            map(|l| l.
                borrow().
                clone()
            ).
            collect(),
        title,
    })
}
