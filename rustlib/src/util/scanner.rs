use std::collections::{HashMap};
use std::convert::TryInto;

use super::token_type::*;
use super::token::*;

#[derive(Debug)]
pub struct Scanner {
  pub source: Vec<char>,
  tokens: Vec<Token>,
  start: usize ,
  current: usize,
  line: i32
}

impl Scanner{
  pub fn new(source: &str) -> Self {
    Scanner{
      source: source.chars().collect(),
      tokens: Vec::<Token>::new(),
      line: 1,
      current: 0,
      start: 0
    }
  }

  pub fn scan_tokens(&mut self) -> Vec<Token> {
    while !self.is_at_end() {
      self.start = self.current;
      self.scan_token();
    }

    self.tokens.push(Token::new(TokenType::EOF, String::from(""), None, self.line));
    self.tokens.clone()
    
    // Vec::new()
  }

  fn scan_token(&mut self) {
    let c: char = self.advance();
    match c {
      '+' =>  {
        if self.is_match(' '){
          self.get_name();
        } else {
          self.add_token(TokenType::PLUS, None);
        }
      },
      ';' =>  { 
        if self.is_match(';') {
          while self.peek() != '\n' && !self.is_at_end() { 
            self.advance();
          }
        } 
      },
      ':' =>  self.add_token(TokenType::SEMICOLON, None),
      '|' =>  self.add_token(TokenType::VERTICALLINE, None),
      '[' =>  self.add_token(TokenType::LEFTSQBRACKET, None),
      ']' => { 
        if self.is_match(';') { 
          self.add_token(TokenType::EOS, None);
        } else {
          self.add_token(TokenType::RIGHTSQBRACKET, None);
        } 
      }  
      ' ' | '\r' | '\t' => {},
      '\n' => self.line += 1,
      any => {}
    } 
  }

  fn get_name(&mut self) {
    while self.peek() != '[' && !self.is_at_end() {
      self.advance();
    }

    let value = self.source[self.start + 2..self.current - 1].iter().collect::<String>();
    self.add_token(TokenType::NAME, Some(Literal::String(value)));
  }

  fn is_match(&mut self, expected: char) -> bool {
    if self.is_at_end() { return false };
    if self.source[self.current] != expected {
      return false;
    }; 
    self.current += 1;
    return true;
  }

  fn is_at_end(&self) -> bool {
    // return self.is_match('\n');
    self.current >= self.source.iter().count().try_into().unwrap()
  }

  fn advance(&mut self) -> char {
    self.current += 1;
    self.source[self.current - 1]
  }

  fn peek(&mut self) -> char {
    if self.is_at_end() { return '\0'} ;
    return self.source[self.current];
  }

  fn peek_next(&self) -> char {
    if self.current + 1 >= self.source.len() { 
      return '\0';
    };

    self.source[self.current + 1]
  } 

  fn add_token(&mut self, kind: TokenType, literal: Option<Literal>) {
    let text: String = self.source[self.start..self.current].iter().collect();
    self.tokens.push(Token::new(kind, text, literal, self.line));
  }
}