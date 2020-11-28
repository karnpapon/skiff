use super::token_type::TokenType;
// use super::worng_value::{WorngValue};

#[derive(Debug, PartialEq,Clone)]
pub enum Literal {
  // Identifier(String),
  String(String),
  Number(f64),
  Bool(bool),
  Nil,
}


impl std::fmt::Display for Literal {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
      // Literal::Identifier(ref iden) => write!(f, "{}", iden),
      Literal::Number(ref number) => write!(f, "{}", number),
      Literal::String(ref string) => write!(f, "{}", string),
      Literal::Bool(ref b) => write!(f, "{}", b),
      Literal::Nil => write!(f, "nil"),
    }
  }
}

#[derive(Debug, Clone)]
pub struct Token {
  pub token_type: TokenType,
  pub lexeme: String,
  pub literal: Option<Literal>,
  pub line: i32 
}

impl Token{

  pub fn new( token_type: TokenType, lexeme: String, literal: Option<Literal> , line: i32) -> Token {
    Token {
      token_type: token_type,
      lexeme: lexeme,
      literal: literal,
      line: line
    }
  }

  fn to_string(&self) -> String {
    return format!("{:?} {:?} {:?}", self.token_type , self.lexeme, self.literal);
  }
}


// impl Literal {
//   pub fn value(&self) -> Option<WorngValue> {
//     let v = match *self {
//       // Literal::Identifier(ref iden) => Some(WorngValue::Identifier(iden.to_string())),
//       Literal::Number(number) => Some(WorngValue::Number(number)),
//       Literal::String(ref string) => Some(WorngValue::String(string.to_string())),
//       Literal::Bool(ref boo) => Some(WorngValue::Bool(boo.clone())),
//       Literal::Nil => Some(WorngValue::Nil),
//     };

//     v
//   }
// }

  
