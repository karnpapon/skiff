use std::rc::Rc;

// use super::worng_value::Worng;
use super::token::{ Literal, Token};
use super::token_type::TokenType;
use super::expr::Expr;
use super::statement::Stmt;
use super::error::{ ParsingError};

use std::result;
use std::error::{Error};

// type Result<T> = result::Result<T, Box<dyn Error>>;

const KEY_BUF_LEN: usize = 32;
const STR_BUF_LEN: usize = 255;
const LOG_BUF_LEN: usize = 64;
const TERM_DICT_BUFFER: usize = 16;
const TERM_LIST_BUFFER: usize = 16;
const TERM_BODY_BUFFER: usize = 24;
const LEXICON_BUFFER: usize = 512;
const LOGS_RANGE: usize = 56;

const LIST_ITEMS: usize = 50;

const NAME: &str = "karnpapon";
const DOMAIN: &str = "https://karnpapon.com/";
const LOCATION: &str = "Bangkok, Thailand";
const REPOPATH: &str = "https://github.com/karnpapon/skiff/";


#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct List {
  name: String,
  keys: Vec<String>,
  vals: Vec<String>,
  len: i32,
  routes: i32
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct Term {
  name: String,
  host: String,
  bref: String,
  r#type: String,
  body:  Vec<String>,
  body_len: i32,
  link: List,
  list: Vec<String>,
	list_len: i32,
	/* generated */
	filename: String,
	date_from: String,
	date_last: String,
	parent: Box<Term>, // re-check pointer
	children: Box<Vec<Term>>, // re-check pointer
	children_len: i32,
  docs: Vec<List>, 
	docs_len: i32,
  incoming: Box<Vec<Term>>, 
	incoming_len: i32,
	outgoing_len: i32
}


#[derive(Clone, Debug)]
pub struct Log {
	date: String,
	rune: String,
	code: i32,
	host: String,
	pict: i32,
	name: String,
  term: Term // re-check pointer
}

#[derive(Debug)]
pub struct Glossary {
	len: i32,
	lists: Vec<List>
}

impl Glossary {
  pub fn set_name(&mut self, index: usize, name: String) {
    self.lists[index].name = name
  }
}

#[derive(Debug)]
pub struct Lexicon {
	len: i32,
	terms: Vec<Term> 
}

#[derive(Debug)]
pub struct Journal {
	len: i32,
  logs: Vec<Log>
}

impl List {
  fn new() -> List{
    List{
      name: String::with_capacity(KEY_BUF_LEN),
      keys: vec![String::with_capacity(LIST_ITEMS); 64],
      vals: vec![String::with_capacity(LIST_ITEMS); STR_BUF_LEN],
      len: 0,
      routes: 0 
    }
  }
  fn findlist(){}
}


impl Term {
  fn new() -> Term {
    Term{
      name:  String::with_capacity(KEY_BUF_LEN),
      host: String::with_capacity(KEY_BUF_LEN),
      bref: String::with_capacity(STR_BUF_LEN),
      r#type: String::with_capacity(KEY_BUF_LEN),
      body: vec![String::with_capacity(STR_BUF_LEN); 750],
      body_len: 0,
      link: List::new(),
      list: vec![],
      list_len: 0,
      filename: String::with_capacity(KEY_BUF_LEN),
      date_from: String::with_capacity(6),
      date_last: String::with_capacity(6),
      parent: Box::new(Term::new()),
      children: Box::new(vec![]),
      children_len: 0,
      docs: vec![List::new(); 20],
      docs_len: 0,
      incoming: Box::new(vec![]),
      incoming_len: 0,
      outgoing_len: 0,
    }
  }
  fn findterm(){}
}

impl Log {
  pub fn new() -> Log{
    Log{
      date: String::with_capacity(6),
      rune: String::new(),
      code: 0,
      host: String::with_capacity(KEY_BUF_LEN),
      pict: 0,
      name: String::with_capacity(LOG_BUF_LEN),
      term: Term::new(),
    }
  }
  fn finddiary(){}
}

impl Glossary {
  fn new() -> Glossary{
    Glossary{
      len: 0,
      lists: vec![List::new(); 100]
    } 
  }
}


impl Lexicon {
  fn new() -> Lexicon {
    Lexicon{
      len: 0,
      terms: vec![Term::new(); 350]
    }
  }
}

impl Journal {
  fn new() -> Journal{
    Journal{
      len: 0,
      logs: vec![Log::new(); 3500 ]
    }
  }
}

#[derive(Debug)]
pub struct Parser {
  tokens: Vec<Token>,
  current: usize
}

impl Parser{
  pub fn new(tokens: Vec<Token>) -> Self {
    Parser{
      tokens: tokens,
      current: 0
    }
  }

  pub fn parse(&mut self) -> Result<Vec<Stmt>, Vec<ParsingError>> {
    let mut statements: Vec<Stmt> = Vec::new();
    let mut errors: Vec<ParsingError> = Vec::new();

    while !self.is_at_end() {
      match self.declaration() {
        Ok(stmt) => statements.push(stmt),
        Err(err) => errors.push(err),
      }
    }

    if errors.len() == 0 {
      Ok(statements)
    } else {
      Err(errors)
    }

  }

  pub fn declaration(&mut self) -> Result<Stmt, ParsingError> {
    
    let statement;
    statement = self.statement();

    match statement {
      Ok(stmt) => Ok(stmt),
      Err(err) => {
        // self.synchronize();
        Err(err)
      }
    }
  }

  
  fn statement(&mut self) -> Result<Stmt, ParsingError> {
     if self.is_match(vec![TokenType::LEFTSQBRACKET]){
      self.block_statement()
    } else {
      let name = self.consume(TokenType::NAME, format!("Expect name.").as_ref() )?;
      println!("name = {:#?}", &name);
      self.expression_statement()
    }
    // else if self.is_match(vec![TokenType::If]){
    //   self.if_statement()
    // } else if self.is_match(vec![TokenType::While]){
    //   self.while_statement()
    // } else if self.is_match(vec![TokenType::For]){
    //   self.for_statement()
    // } else if self.is_match(vec![TokenType::Return]) {
    //   self.return_statement()
    // } else {
    //   self.expression_statement()
    // }
  }

  fn block_statement(&mut self) -> Result<Stmt, ParsingError> {
    let mut statements : Vec<Stmt> = Vec::new();

    while !self.check(&TokenType::EOS) && !self.is_at_end() {
      statements.push(self.declaration()?);
    }

    self.consume(TokenType::EOS, "Expect '];' after block.")?;
    return Ok(Stmt::Block(statements));
  }

  fn expression_statement(&mut self) -> Result<Stmt, ParsingError> {
    let expr = self.primary()?;

    match self.consume( TokenType::EOF, "Expect ';' after expression.") {
      Ok(_) => Ok(Stmt::Expr(expr)),
      Err(err) => Err(err),
    }
  }

  // fn expression_statement(&mut self) -> Result<Stmt, ParsingError> {
  //   let expr = self.expression()?;

  //   match self.consume( TokenType::Semicolon, "Expect ';' after expression.") {
  //     Ok(_) => Ok(Stmt::Expr(expr)),
  //     Err(err) => Err(err),
  //   }
  // }


  // fn expression(&mut self) -> Result<Expr, ParsingError> {
  //  self.assignment()
  // }

  // fn assignment(&mut self) -> Result<Expr, ParsingError>{
  //   let expr = self.or()?;

  //   if self.is_match(vec![TokenType::Equal]) {
  //     let equals =  self.previous().clone();
  //     let value = self.assignment()?;

  //     match expr {
  //       Expr::Var( token, _) => {
  //         return Ok(Expr::Assign(token, Box::new(value), None ));
  //       },
  //       Expr::Get( ref object, ref name) => {
  //         return Ok( Expr::Set(object.clone(), name.clone(), Box::new(value)))
  //       },
  //       _ => return Err(ParsingError::InvalidAssignmentError(equals))
  //     }
  //   }
  //   return Ok(expr);
  // }

  // fn  or(&mut self) -> Result<Expr, ParsingError> {
  //   let mut expr = self.and()?;

  //   while self.is_match(vec![TokenType::Or]) {
  //     // not sure why using this expression directly is not working.
  //     // eg. Expr::Logical(Box::new(expr), self.previous().clone(), Box::new(right) ); is not working.
  //     let operator = self.previous().clone(); 
  //     let right = self.and()?;
  //     expr = Expr::Logical(Box::new(expr), operator, Box::new(right) );
  //   }

  //   return Ok(expr);
  // }

  // fn and(&mut self) -> Result<Expr, ParsingError> {
  //   let mut expr = self.equality()?;

  //   while self.is_match(vec![ TokenType::And ]) {
  //     let operator = self.previous().clone();
  //     let right = self.equality()?;
  //     expr = Expr::Logical(Box::new(expr), operator, Box::new(right));
  //   }

  //   return Ok(expr);
  // }

  // fn equality(&mut self) -> Result<Expr, ParsingError> {
  //   let mut expr = self.comparison().unwrap();

  //   while self.is_match(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
  //     let operator = self.previous().clone();
  //     let right = self.comparison()?;
  //     expr = Expr::Binary(Box::new(expr), operator, Box::new(right) );
  //   }

  //   return Ok(expr);
  // }

  // fn comparison(&mut self) -> Result<Expr, ParsingError> {
  //   let mut expr = self.addition().unwrap();

  //   while self.is_match(vec![
  //     TokenType::Greater, 
  //     TokenType::GreaterEqual, 
  //     TokenType::Less, 
  //     TokenType::LessEqual
  //     ]) {
  //     let operator = self.previous().clone();
  //     let right = self.addition()?;
  //     expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
  //   }

  //   return Ok(expr);
  // }

  // fn addition(&mut self) -> Result<Expr, ParsingError>  {
  //   let mut expr = self.multiplication().unwrap();

  //   while self.is_match(vec![TokenType::Minus, TokenType::Plus]) {
  //     let operator = self.previous().clone();
  //     let right = self.multiplication().unwrap();
  //     expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
  //   }

  //   return Ok(expr);
  // }

  // fn multiplication(&mut self) -> Result<Expr, ParsingError>  {
  //   let mut expr = self.unary().unwrap();

  //   while self.is_match(vec![TokenType::Slash, TokenType::Star]) {
  //     let operator = self.previous().clone();
  //     let right = self.unary().unwrap();
  //     expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
  //   }

  //   return Ok(expr);
  // }

  // fn unary(&mut self) -> Result<Expr, ParsingError>  {
  //   if self.is_match(vec![TokenType::Bang, TokenType::Minus]) {
  //     let operator = self.previous().clone();
  //     let right = self.unary().unwrap();
  //     return Ok(Expr::Unary(operator, Box::new(right)) );
  //   }

    
  //   return self.call();
  // }

  // fn call(&mut self) -> Result<Expr, ParsingError> {
    
  //   let mut expr = self.primary();

  //   loop { 
  //     if self.is_match(vec![TokenType::LeftParen]) {
  //       expr = self.finish_call(expr?);
  //     } else if self.is_match(vec![TokenType::Dot]){  
  //       let name = self.consume(TokenType::Identifier, "Expect property name after '.'." );
  //       expr = Ok(Expr::Get(Box::new(expr?), name? ));
  //     } else {
  //       break;
  //     }
  //   }

  //   return expr;
  // }

  // fn finish_call(&mut self, callee: Expr) -> Result<Expr, ParsingError>{
  //   let mut arguments: Vec<Expr> = Vec::new();
  //   if !self.check(&TokenType::RightParen) {
  //     arguments.push(self.expression()?);

  //     while self.is_match(vec![TokenType::Comma]){
  //       if arguments.len() >= 10 {
  //         return Err(ParsingError::TooManyArgumentsError); // no needs to throw en error, just report, is fine.
  //       }
  //       arguments.push(self.expression()?);
  //     }
  //   }

  //   let paren = self.consume(TokenType::RightParen, "Expect ')' after arguments.")?;

  //   return Ok(Expr::Call(Box::new(callee), paren, arguments));
  // }
  
  fn primary(&mut self) -> Result<Expr, ParsingError> {
    // if self.is_match(vec![TokenType::NAME]){
      
    //   return Ok(Expr::Var(self.previous().clone(), None));
    // }
    return  Ok(Expr::Literal(self.previous().literal.clone().unwrap()));
  }

  fn is_match(&mut self, types: Vec<TokenType>) -> bool {
    for token_type in types {
      if self.check(&token_type) {
        self.advance();
        return true;
      }
    }

    return false;
  }

  fn check(&self, token: &TokenType) -> bool {
    if self.is_at_end() { return false; }
    return self.peek().token_type == *token;
  }

  fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, ParsingError> {
    if self.check(&token_type) { 
      return Ok(self.advance().clone());
    } else {
      return Err(ParsingError::ParsingError);
    };
  }

  // fn synchronize(&mut self) {
  //   self.advance();

  //   while !self.is_at_end() {
  //     if self.previous().token_type == TokenType::Semicolon {return;};

  //     match self.peek().token_type {
  //       TokenType::Class |
  //       TokenType::Func |
  //       TokenType::Var |
  //       TokenType::For |
  //       TokenType::If |
  //       TokenType::While |
  //       TokenType::Print |
  //       TokenType::Return => return,
  //       _ => {}
  //     }

  //     self.advance();
  //   }
  // }

  

  fn advance(&mut self) -> &Token {
    if !self.is_at_end() { self.current += 1;}
    self.previous()
  }

  fn is_at_end(&self) -> bool {
    return self.peek().token_type == TokenType::EOF;
  }

  fn peek(&self) -> &Token {
    return self.tokens.get(self.current).unwrap();
  }

  fn previous(&self) -> &Token {
    return self.tokens.get(self.current - 1).unwrap();
  }
}

