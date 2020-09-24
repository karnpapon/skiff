
#[derive(Debug)]
pub enum SkiffError{
  RuntimeError 
}



impl std::fmt::Display for SkiffError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
      SkiffError::RuntimeError => {
          write!(f, "skiff error! something went wrong")
        },
    }
  }
}
