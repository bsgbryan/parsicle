use url::Url;

pub fn sanitize(url: &str) -> Option<String> {
  if let Ok   (url)   = Url::parse(url) &&
     let Some(domain) = url.domain()
  {
    return Some(format!("{}://{}{}",
      url.scheme(),
      domain,
      url.path(),
    ))
  }
  else { None }
}