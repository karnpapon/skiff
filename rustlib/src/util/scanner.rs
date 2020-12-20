use std::collections::HashMap;
use std::convert::TryInto;

#[derive(Debug)]
pub struct Scanner {
  pub source: Vec<char>,
  current: usize,
  line: i32,
}

impl Scanner {
  pub fn new(source: &str) -> Self {
    Scanner {
      source: source.chars().collect(),
      line: 1,
      current: 0,
    }
  }
}
