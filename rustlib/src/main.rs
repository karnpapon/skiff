use std::io::{self, Read, Write, BufReader };
use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use std::io::LineWriter;
use std::io::BufWriter;
use std::io::prelude::*;
use std::env;
use std::fs::File;
use std::path::Path;


mod util;
use util::error::SkiffError;
use util::helpers;
use util::scanner::{ Scanner, };
use util::token::Token;
use util::parser::Parser;

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
pub struct Term<'a> {
  name: String, // title
  host: String, // HOST, nothing fancy.
  bref: String,
  r#type: String,
  body:  Vec<String>, // BODY list item (no linking in parsing process ).
  body_len: usize, // BODY list item counter.
  link: List, // LINK item ( empty `.name` field eg. link.name = "" )
  list: Vec<String>, // LIST field.
	list_len: i32,
	/* generated */
	filename: String, // generate from name field.
	date_from: RefCell<String>, // not in parsing process.
	date_last: RefCell<String>,
	// parent: Box<Term>, // re-check pointer
	// children: Box<Vec<Term>>, // re-check pointer
	children_len: i32,
  docs: Vec<&'a List>, 
	docs_len: i32,
  // incoming: Box<Vec<Term>>, 
	incoming_len: i32,
	outgoing_len: i32
}


#[derive(Debug)]
pub struct Log<'a> {
	date: String,
	// rune: String,
	code: i32,
	host: String,
	pict: i32,
	name: String,
  term: RefCell<Option<&'a Term<'a>>>
}

#[derive(Debug)]
pub struct Glossary {
	len: i32,
	lists: Vec<List>
}

#[derive(Debug)]
pub struct Lexicon<'l> {
	len: i32,
	terms: Vec<Term<'l>> 
}

#[derive(Debug)]
pub struct Journal<'b> {
	len: i32,
  logs: Vec<Log<'b>>
}

impl List {
  fn new() -> List{
    List{
      name: String::with_capacity(KEY_BUF_LEN),
      keys: vec![],
      vals: vec![],
      len: 0,
      routes: 0 
    }
  }
  fn findlist(){}
}


impl<'a> Term<'a> {
  fn new() -> Term<'a> {
    Term{
      name:  String::with_capacity(KEY_BUF_LEN),
      host: String::with_capacity(KEY_BUF_LEN),
      bref: String::with_capacity(STR_BUF_LEN),
      r#type: String::with_capacity(KEY_BUF_LEN),
      // body: vec![String::with_capacity(STR_BUF_LEN); 750],
      body: vec![],
      body_len: 0,
      link: List::new(),
      list: vec![],
      list_len: 0,
      filename: String::with_capacity(KEY_BUF_LEN),
      date_from: RefCell::new(String::with_capacity(6)),
      date_last: RefCell::new(String::with_capacity(6)),
      // parent: Box::new(Term::new()),
      // children: Box::new(vec![Term::new()]),
      children_len: 0,
      // docs: vec![List::new(); 20],
      docs: vec![],
      docs_len: 0,
      // incoming: Box::new(vec![]),
      incoming_len: 0,
      outgoing_len: 0,
    }
  }

  fn findterm(){}
}

impl <'a>Log<'a> {
  pub fn new() -> Log<'a>{
    Log{
      date: String::with_capacity(6),
      // rune: String::new(),
      code: 0,
      host: String::with_capacity(KEY_BUF_LEN),
      pict: 0,
      name: String::with_capacity(LOG_BUF_LEN),
      term: RefCell::new(None),
    }
  }
  fn finddiary(){}
}

impl Glossary {
  fn new() -> Glossary{
    Glossary{
      len: 0,
      // lists: RefCell::new(vec![List::new(); 100])
      lists: vec![List::new()]
    } 
  }

  fn update_len(&mut self) {
    self.len += 1;
  }

  // fn get_lists(&self) -> &List {
  //   self.lists.borrow();
  // }
}


impl <'a>Lexicon<'a> {
  fn new() -> Lexicon<'a> {
    Lexicon{
      len: 0,
      terms: vec![Term::new()]
    }
  }
}

impl <'a>Journal<'a> {
  fn new() -> Journal<'a>{
    Journal{
      len: 0,
      logs: vec![Log::new()]
    }
  }
}

fn scan_glossary(content: &str) {
  let mut tokens: Vec<Token> = Vec::new();
  let mut scanner = Scanner::new(&content);
  tokens = scanner.scan_tokens();
  // let mut parser = Parser::new(tokens);
  // let mut expression = parser.parse().unwrap();
  
	
  // Ok(())
}


// ------------------methods-----------------------

pub fn parse(all_lists: &mut Glossary, all_terms: &mut Lexicon, all_logs: &mut Journal) {
  println!("Parsing  | ");
  parse_glossary(String::from("./database/glossary.ndtl"), all_lists ).unwrap();
  parse_lexicon(String::from("./database/lexicon.ndtl"), all_terms ).unwrap();
  parse_horaire(String::from("./database/horaire.ndtl"), all_logs ).unwrap();
}


// ------------------main-----------------------

fn parse_glossary(path: String, glossary: &mut Glossary) -> Result<(), SkiffError>  {

  let mut f = File::open(path).expect("Glossary Parsing: file not found");
  let mut f_reader = BufReader::new(f);
  let mut len: usize = 0;
  let mut line = String::new();
  let mut depth: usize;
  let mut split: i32 = 0;
  let mut l = &mut glossary.lists[glossary.len as usize];
  let mut scanner: Scanner;

  while f_reader.read_line(&mut line).unwrap() > 0 {
    scanner = Scanner::new(&line.trim_end());
    depth = helpers::cpad(&scanner.source, ' ');

    // skip blank line case.
    if scanner.source.len() == 0 as usize  {
      line.clear(); 
      continue; 
    }

    match helpers::strm(&scanner.source){
      Some(string) => len = helpers::slen(string.trim_end().chars().collect::<Vec<char>>().as_ref()),
      None => len = 0
    }
    
    // skip if it's comments or blank line.
    if len < 4 || &scanner.source[0] == &';' { 
      line.clear(); 
      continue; 
    }
    if len > 400 { return Err(SkiffError::ParseError("Glossary Parsing: Line is too long".to_string())); }
    
    if depth == 0 {
      if l.len > 0 {
        glossary.lists.insert(glossary.len as usize, List::new());
        l = &mut glossary.lists[glossary.len as usize];
      }
      l.name = scanner.source.into_iter().collect();
      helpers::slca(&mut l.name.chars().collect::<Vec<char>>()); // string to lowercase (eg. DICTIONARY -> dictionary ).
      glossary.len += 1;
    } else if depth == 2 { // in case of list item( 2 spaces at beginning of line).
			if l.len >= LIST_ITEMS as i32 {
        return Err(SkiffError::ParseError("Glossary Parsing: Reached LIST_ITEMS limit".to_string()));
      }
      split = helpers::cpos(&scanner.source, ':'); // find index of `:` return -1 if not found. 
			if split < 0 { // handle only list which not include `:` in sentence.
        // return normal string.
        l.vals.insert(l.len as usize, helpers::sstr(&scanner.source, 2, len + 2));
			} else {
        l.keys.insert(l.len as usize, helpers::sstr(&scanner.source,2, ( split - 3 ) as usize));// title of list line.
        l.vals.insert(l.len as usize, helpers::sstr(&scanner.source, ( split + 2 ) as usize, len - split as usize)); // details of list line.
			}
      l.len += 1;
		}
    
    // clear to reuse the buffer 
    // in other words, get freshly new line, 
    // without cancating with prev line.
    line.clear(); 
  }
  
  // println!("glossary = {:#?}", &glossary);
  Ok(())
}

// TODO: make it less C-ish style.
fn parse_lexicon(path: String, lexicon: &mut Lexicon) -> Result<(), SkiffError> {

  let mut f = File::open(path).expect("lexicon parsing: file not found");
  let mut f_reader = BufReader::new(f);
  let mut key_len: usize;
  let mut val_len: usize; 
  let mut len: usize = 0;
  let count = 0;
  let mut catch_body = false; 
  let mut catch_link = false;
  let mut catch_list = false;
  let mut t = &mut lexicon.terms[lexicon.len as usize];
  let mut line = String::new();
  let mut scanner: Scanner;
  let mut depth: usize;

  while f_reader.read_line(&mut line).unwrap() > 0 {

    scanner = Scanner::new(&line.trim_end());
    depth = helpers::cpad(&scanner.source, ' ');

    if scanner.source.len() == 0 as usize  {
      line.clear(); 
      continue; 
    }

    match helpers::strm(&scanner.source){
      Some(string) => len = string.len(),
      None => len = 0
    }

    // len < 3 = skip 'newline' eg. '\n';
    if len < 3 || &scanner.source[0] == &';'{
      line.clear(); 
      continue;
    }

    if len > 750 { return Err(SkiffError::ParseError("Lexicon Parsing: Line is too long".to_string())); }
    
    if depth == 0 {
      
      if lexicon.len > 0 {
        lexicon.terms.insert(lexicon.len as usize, Term::new());
        t = &mut lexicon.terms[lexicon.len as usize];
      }
      if !helpers::sans(&scanner.source) != 0 { 
        println!("Lexicon warning: {}", SkiffError::ParseError("Lexicon key is not alphanum".to_string()));
      } 
      t.name = helpers::sstr(&scanner.source, 0, len).to_lowercase();
      t.filename = helpers::sstr(&scanner.source, 0, len).replace(" ", "_").to_lowercase();
      lexicon.len += 1;
    } else if depth == 2 {
      t = &mut lexicon.terms[(lexicon.len - 1) as usize];
      if helpers::spos(&scanner.source, "HOST : ") >= 0{
        t.host = helpers::sstr(&scanner.source, 9, len - 9);
      }
      if helpers::spos(&scanner.source, "BREF : ") >= 0{
        t.bref = helpers::sstr(&scanner.source, 9, len - 9);
      }
      if helpers::spos(&scanner.source, "TYPE : ") >= 0{
        t.r#type = helpers::sstr(&scanner.source, 9, len - 9); 
      }
      catch_body = helpers::spos(&scanner.source, "BODY") >= 0;
      catch_link = helpers::spos(&scanner.source, "LINK") >= 0;
      catch_list = helpers::spos(&scanner.source, "LIST") >= 0;
    } else if depth == 4 { // BODY item ( 4 indent spaces.)
        t = &mut lexicon.terms[(lexicon.len - 1) as usize];
        /* Body */
        if catch_body {
          t.body.insert(t.body_len, helpers::sstr(&scanner.source,  4, len - 4));
          t.body_len += 1;
        }
        /* Link */
        if catch_link {
          key_len = (helpers::cpos(&scanner.source, ':') - 5 ) as usize;
          t.link.keys.insert(t.link.len as usize, helpers::sstr(&scanner.source, 4, key_len));
          val_len = len - key_len - 5;
          t.link.vals.insert(t.link.len as usize, helpers::sstr(&scanner.source,  key_len + 7, val_len));
          t.link.len += 1;
        }
        /* List */
        if catch_list {
          t.list.insert(t.list_len as usize,helpers::sstr(&scanner.source, 4, len - 4));
          t.list_len += 1;
        }
        // t.list_len += 1;
    }
    // count += 1;
    line.clear(); 
  }
  
  // println!("lexicon = {:#?}", &lexicon);
  Ok(())
}

fn parse_horaire(path: String, journal: &mut Journal) -> Result<(), SkiffError> {

  let f = File::open(path).expect("journal parsing: file not found");
  let mut f_reader = BufReader::new(f);
  let mut len;
  let mut line = String::new();
  let mut scanner: Scanner;
  let mut l = &mut journal.logs[journal.len as usize];
  // let mut count = 0;
  // let mut depth: usize;
  
  while f_reader.read_line(&mut line).unwrap() > 0 {

    scanner = Scanner::new(&line.trim_end());
    // depth = helpers::cpad(&scanner.source, ' ');

    match helpers::strm(&scanner.source){
      Some(string) => len = string.len(),
      None => len = 0
    }
    
		if len < 14 || &scanner.source[0] == &';' {
      line.clear();
			continue;
    }

    if len > 72 { return Err(SkiffError::ParseError("Log is too long".to_string()))}
    
    if journal.len > 0 {
      journal.logs.insert(journal.len as usize, Log::new());
      l = &mut journal.logs[journal.len as usize];
    }
		/* Date */
		l.date = helpers::sstr(&scanner.source , 0, 5);
		/* Rune */
		// l.rune = &scanner.source[6];
    /* Code */
		l.code = helpers::sint(&scanner.source[7..], 3) as i32;
    /* Term */
    // extract only `host` type.
    let mut split_line = line.split_whitespace().into_iter();
    let host_len = &split_line.nth(2).unwrap().len();
    l.host = helpers::sstr(&scanner.source, 11, *host_len);
    
    let _host = &l.host.chars().collect::<Vec<char>>();

    /* Name */
    if let Some(code_col) = split_line.nth(1){
      l.name = code_col.to_string().replace("_", " ");
    }
    
		if !helpers::sans(_host) != 0 {
			println!("Warning: {} is not alphanum", l.host);
    }
		/* Pict */
		if len >= 35 {
			l.pict = helpers::sint(&scanner.source[32..], 3) as i32;
    }
    journal.len += 1;
    line.clear(); 
  }

  // println!("journal = {:#?}", &journal);
  Ok(())
}

fn link<'a,'b>(glo: &'a mut Glossary, lex: &'b mut Lexicon, jou: &'b mut Journal<'b>) {
	println!("Linking  | ");
	for i in 0..jou.len { // iterate through jou list (horaire's table)
    let l = &mut jou.logs[i as usize];
    
    // match HOST in jou to lex.
    match findterm(lex, &l.host){
     Some(t) => *l.term.borrow_mut() = Some(t),
     None =>  *l.term.borrow_mut() = None 
    } 

    if let Some(_t) = *l.term.borrow_mut() {
      if _t.date_last.borrow().len() == 0 {
        _t.date_last.borrow_mut().push_str(&l.date);
      }
      _t.date_from.borrow_mut().push_str(&l.date);
    }
    
	}
  println!("lexicon({} entries) ", lex.len);
  
	// for i in 0..lex.len {
	// 	let mut t: &Term = &lex.terms[i as usize];
	// 	for j in 0..t.body_len {
	// 		// ftemplate(NULL, lex, t, t.body[j]);
  //   }
	// 	t.parent = findterm(lex, t.host);
	// 	if !t.parent {
  //     return Err(SkiffError::ParseError("Unknown term host = {}, , t->host".to_string())); 
  //   }
	// 	t.parent.children[t.parent.children_len] = t;
	// 	t.parent.children_len += 1;
	// }
	// println!("glossary({} entries) ", glo.len); 
	// find and matching title(name field in glossary.ndtl file) 
	// to lex terms ( lex->terms.list in lexicon.ndtl file)
	// for i in 0..lex.len { 
	// 	let mut t: &Term = &lex.terms[i as usize];
	// 	for j in 0..t.list_len {
	// 		let mut l: &List = helpers::findlist(glo, t.list[j]);
	// 		if(!l) {
	// 			return Err(SkiffError::ParseError("Unknown list = {}, t->list[j]".to_string())); 
  //     }
	// 		t.docs[t.docs_len as usize] = l;
	// 		t.docs_len += 1;
	// 		l.routes += 1;
	// 	}
  // }
  
  println!("log = {:#?}", jou);
}

fn findterm<'a>(lex: &'a Lexicon, name: &str) -> Option<&'a Term<'a>> {
  let mut _name = String::with_capacity(name.len());
  _name  = name.to_lowercase().replace("_", " ");
  
	for i in 0..lex.len {
    if &_name == &lex.terms[i as usize].name  {
      return Some(&lex.terms[i as usize]);
    }
  }

	return None;
}


pub fn scan(content: &str)  {
  println!("Scanner  | ");
  scan_glossary(&content);
}


fn main() {
  let mut all_terms = Lexicon::new();
  let mut all_lists = Glossary::new();
  let mut all_logs = Journal::new();

  parse(&mut all_lists, &mut all_terms, &mut all_logs);
  link(&mut all_lists, &mut all_terms, &mut all_logs);
  
}
