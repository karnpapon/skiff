#[derive(Debug)]
pub enum SkiffError {
  ParseError(String),
  RuntimeError,
}

#[derive(Debug)]
pub enum ParsingError {
  ParsingError,
}

impl std::fmt::Display for SkiffError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
      SkiffError::ParseError(ref msg) => {
        write!(f, "ParseError: {}", msg)
      }
      SkiffError::RuntimeError => {
        write!(f, "skiff error! something went wrong")
      }
    }
  }
}

impl std::fmt::Display for ParsingError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
      ParsingError::ParsingError => {
        write!(f, "Error: Parsing error")
      }
    }
  }
}
