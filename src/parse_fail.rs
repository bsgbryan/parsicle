use std::{
  error::Error,
  fmt::Display,
};

#[derive(Clone, Debug)]
pub struct ParseFail;

impl Display for ParseFail {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "Parse Failed")
  }
}

impl Error for ParseFail {}