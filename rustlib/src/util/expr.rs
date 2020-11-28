use super::token::{Literal, Token};

use std::fmt;

#[derive(Debug, Clone)]
pub enum Expr{
  Literal(Literal),
}


impl fmt::Display for Expr {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match *self{
      Expr::Literal(ref expression) => {
        write!(f, "{}", expression) 
      },
    }
  }
}