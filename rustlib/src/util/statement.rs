use std::rc::Rc;
use super::expr::Expr;
use super::error::ParsingError;

#[derive(Debug, Clone)]
pub enum Stmt {
  Block(Vec<Stmt>),
  Expr(Expr)
}

impl std::fmt::Display for Stmt {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self{
      Stmt::Block(ref stmt) => {
        write!(f, "block = {:?}", &stmt)
      },
      Stmt::Expr(ref expr) => {
        write!(f, "expr = {:?}", &expr)
      },
    }
  }
}