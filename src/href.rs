use url::Url;

pub fn stringify(url: &Url) -> Option<String> {
  if let Some(domain) = url.domain() {
    return Some(format!("{}://{}{}",
      url.scheme(),
      domain,
      url.path(),
    ))
  }
  else { None }
}