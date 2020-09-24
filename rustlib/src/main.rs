use std::io::{self, Read, Write };
use std::io::LineWriter;
use std::io::prelude::*;
use std::env;
use std::fs::File;
use std::path::Path;

mod util;
use util::error::SkiffError;

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
      parent: Box::new(vec![]),
      children: Box::new(vec![]),
      children_len: 0,
      docs: vec![List::new(); 20],
      docs_len: 0,
      incoming: vec![],
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

// impl File

fn main() {
  let all_terms = Lexicon::new();
  let all_lists = Glossary::new();
  let all_logs = Journal::new();

  println!("Glossaryl ----> {:?}", &all_lists.lists[0]);
}
