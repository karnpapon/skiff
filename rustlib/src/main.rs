#![ allow( dead_code, unused_imports, unused_variables, unused_assignments,unused_mut ) ]

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
pub struct Term{
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
	date_from: RefCell<String>, // use RefCell, since this field shared mutability.
	date_last: RefCell<String>,
	parent: Option<Box<Rc<RefCell<Term>>>>, 
	children: Vec<Option<Box<Rc<RefCell<Term>>>>>,
	children_len: i32,
  docs: Vec<Option<Rc<RefCell<List>>>>,
	docs_len: i32,
  // incoming: Box<Vec<Term>>, 
	incoming_len: i32,
	outgoing_len: i32
}


#[derive(Debug)]
pub struct Log {
	date: String,
	// rune: String,
	code: i32,
	host: String,
	pict: i32,
	name: String,
  term: Option<Rc<RefCell<Term>>>, 
}

#[derive(Debug)]
pub struct Glossary {
	len: i32,
	lists: Vec<Rc<RefCell<List>>> 
}

#[derive(Debug)]
pub struct Lexicon {
	len: i32,
	terms: Vec<Rc<RefCell<Term>>> 
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
      keys: vec![],
      vals: vec![],
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
      body: vec![],
      body_len: 0,
      link: List::new(),
      list: vec![],
      list_len: 0,
      filename: String::with_capacity(KEY_BUF_LEN),
      date_from: RefCell::new(String::with_capacity(6)),
      date_last: RefCell::new(String::with_capacity(6)),
      parent: None,
      children: vec![],
      children_len: 0,
      docs: vec![],
      docs_len: 0,
      // incoming: Box::new(vec![]),
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
      // rune: String::new(),
      code: 0,
      host: String::with_capacity(KEY_BUF_LEN),
      pict: 0,
      name: String::with_capacity(LOG_BUF_LEN),
      term: None,
    }
  }
  fn finddiary(){}
}

impl Glossary {
  fn new() -> Glossary{
    Glossary{
      len: 0,
      lists: vec![Rc::new(RefCell::new(List::new()))]
    } 
  }

  fn update_len(&mut self) {
    self.len += 1;
  }

  // fn get_lists(&self) -> &List {
  //   self.lists.borrow();
  // }
}


impl Lexicon {
  fn new() -> Lexicon {
    Lexicon{
      len: 0,
      terms: vec![Rc::new(RefCell::new(Term::new()))]
    }
  }
}

impl Journal {
  fn new() -> Journal{
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
      if l.borrow().len > 0 {
        glossary.lists.insert(glossary.len as usize, Rc::new(RefCell::new(List::new())));
        l = &mut glossary.lists[glossary.len as usize];
      }
      l.borrow_mut().name = scanner.source.into_iter().collect::<String>().to_lowercase();
      glossary.len += 1;
    } else if depth == 2 { // in case of list item( 2 spaces at beginning of line).
			if l.borrow().len >= LIST_ITEMS as i32 {
        return Err(SkiffError::ParseError("Glossary Parsing: Reached LIST_ITEMS limit".to_string()));
      }
      split = helpers::cpos(&scanner.source, ':'); // find index of `:` return -1 if not found. 
      let l_len = l.borrow().len as usize;
			if split < 0 { // handle only list which not include `:` in sentence.
        // return normal string.
        l.borrow_mut().vals.insert(l_len, helpers::sstr(&scanner.source, 2, len + 2));
			} else {
        l.borrow_mut().keys.insert(l_len, helpers::sstr(&scanner.source,2, ( split - 3 ) as usize));// title of list line.
        l.borrow_mut().vals.insert(l_len, helpers::sstr(&scanner.source, ( split + 2 ) as usize, len - split as usize)); // details of list line.
			}
      l.borrow_mut().len += 1;
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
        lexicon.terms.insert(lexicon.len as usize, Rc::new(RefCell::new(Term::new())));
        t = &mut lexicon.terms[lexicon.len as usize];
      }
      if !helpers::sans(&scanner.source) == 0 { 
        println!("Lexicon warning: {}", SkiffError::ParseError("Lexicon key is not alphanum".to_string()));
      } 
      t.borrow_mut().name = helpers::sstr(&scanner.source, 0, len).to_lowercase();
      t.borrow_mut().filename = helpers::sstr(&scanner.source, 0, len).replace(" ", "_").to_lowercase();
      lexicon.len += 1;
    } else if depth == 2 {
      t = &mut lexicon.terms[(lexicon.len - 1) as usize];
      if helpers::spos(&scanner.source, "HOST : ") >= 0{
        t.borrow_mut().host = helpers::sstr(&scanner.source, 9, len - 9);
      }
      if helpers::spos(&scanner.source, "BREF : ") >= 0{
        t.borrow_mut().bref = helpers::sstr(&scanner.source, 9, len - 9);
      }
      if helpers::spos(&scanner.source, "TYPE : ") >= 0{
        t.borrow_mut().r#type = helpers::sstr(&scanner.source, 9, len - 9); 
      }
      catch_body = helpers::spos(&scanner.source, "BODY") >= 0;
      catch_link = helpers::spos(&scanner.source, "LINK") >= 0;
      catch_list = helpers::spos(&scanner.source, "LIST") >= 0;
    } else if depth == 4 { 
      t = &mut lexicon.terms[(lexicon.len - 1) as usize];
      /* Body */
      if catch_body {
        let _len = t.borrow().body_len;
        t.borrow_mut().body.insert(_len, helpers::sstr(&scanner.source,  4, len - 4));
        t.borrow_mut().body_len += 1;
      }
      /* Link */
      if catch_link {
        key_len = (helpers::cpos(&scanner.source, ':') - 5 ) as usize;
        let link_len = t.borrow().link.len as usize;
        t.borrow_mut().link.keys.insert(link_len as usize, helpers::sstr(&scanner.source, 4, key_len));
        val_len = len - key_len - 5;
        t.borrow_mut().link.vals.insert(link_len as usize, helpers::sstr(&scanner.source,  key_len + 7, val_len));
        t.borrow_mut().link.len += 1;
      }
      /* List */
      if catch_list {
        let list_len = t.borrow().list_len as usize;
        t.borrow_mut().list.insert(list_len,helpers::sstr(&scanner.source, 4, len - 4));
        t.borrow_mut().list_len += 1;
      }
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
    
		if !helpers::sans(_host) == 0 {
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

fn link(glo: &mut Glossary, lex: &mut Lexicon, jou: &mut Journal) -> Result<(), SkiffError>{
	println!("Linking  | ");
	for i in 0..jou.len { 
    let jou_logs = &mut jou.logs[i as usize];

    match findterm(lex, &jou_logs.host){
     Some(t) => jou_logs.term = Some(t),
     None =>  jou_logs.term = None 
    } 

    match &jou_logs.term {
      Some(_t) => { 
        let _t_len = _t.borrow().date_last.borrow().len();
        if _t_len == 0 {
          _t.borrow().date_last.borrow_mut().push_str(&jou_logs.date);
        }
        _t.borrow().date_from.borrow_mut().push_str(&jou_logs.date);
      },
      None => {}
    }
	};
  println!("lexicon({} entries) ", lex.len);
  
	for i in 0..lex.len {
    let lex_terms = &lex.terms[i as usize];
    let host_name = lex_terms.borrow().host.to_string();

		for j in 0..lex_terms.borrow().body_len {
      let l_term = &lex_terms.borrow();
      let body = &l_term.body[j];
      ftemplate(None, lex, l_term, body).unwrap();
    }

    match findterm(lex, &host_name) {
      Some(t) => { 
        let children_len = lex_terms.borrow().children_len as usize;
        lex_terms.borrow_mut().parent = Some(Box::new(t.clone())); 
        lex_terms.borrow_mut().children.insert(children_len, Some(Box::new(t.clone())));
        lex_terms.borrow_mut().children_len += 1;
      },
      None => { 
        lex_terms.borrow_mut().parent = None;
        // return Err(SkiffError::ParseError("Unknown term host = {}, , t->host".to_string()))
      }
    }
	};
  println!("glossary({} entries) ", glo.len); 
	for i in 0..lex.len { 
    let mut lext = lex.terms[i as usize].borrow_mut();
    let lex_terms_list = lext.clone().list;
    let lext_terms_len = lext.list_len;
		for j in 0..lext_terms_len {
      match findlist(glo, &lex_terms_list[j as usize]) {
        Some(l) => { 
          let docs_len = lext.docs_len as usize;
          lext.docs.insert(docs_len as usize, Some(l.clone())); 
          lext.docs_len += 1;
          l.borrow_mut().routes += 1;
        },
        None => { 
          lext.parent = None;
          // return Err(SkiffError::ParseError("Unknown list = {}, t.list[j]".to_string()));
        }
      }
		}
  }
  
  // println!("lex = {:#?}", lex);
  // println!("jou = {:#?}", jou);
  Ok(())
}

fn ftemplate(f: Option<String>, lex: &Lexicon, t: &Term, s: &str) -> Result<(), SkiffError>{
  let mut capture = false;
  let mut buf = vec![];
  let fp = f.clone();
  let _s = s.chars().collect::<Vec<char>>();
  buf.insert(0, '\0');
  if s.len() > 0 {
    for i in 0.._s.len() {
      let c = _s[i];
      if c == '}' {
        capture = false;
        // check if it's module link ( eg, {^bandcamp 163410848} )
        if buf[0] == '^' && fp.clone().is_some() {
          fpmodule(fp.clone().unwrap(), &buf);
        }
        else if buf[0] != '^' { // or normal link (eg, {methascope}) 
          // fplink(f, lex, t, buf);
        }
      }
      if capture {
        if helpers::slen(&buf) < STR_BUF_LEN - 1 {
        	buf = helpers::ccat(&buf, c);
        } else {
        	return Err(SkiffError::ParseError("template too long, s".to_string()));
        }
      } else if c != '{' && c != '}' && f.is_some() {
        // fputc(c, f); native C function.
      }
  
      if c == '{' {
        capture = true;
        buf[0] = '\0';
      }
    }
  }
  
  Ok(())
}

// build modules (eg. codeblock, iframe link)
fn fpmodule(f: String, s: &[char]) {

	// s = link (eg, ^bandcamp 163410848);
	let split = helpers::cpos(s, ' ');
  let mut cmd: String;
  let target: String;
	cmd = helpers::sstr(s, 1, (split - 1) as usize);
	println!("s = {:?}", s);
	target = helpers::sstr(s, (split + 1) as usize, helpers::slen(s) - split as usize);

	if cmd == "itchio" {
		// printf(f, "<iframe frameborder='0' src='https://itch.io/embed/%s?link_color=000000' width='600' height='167'></iframe>", target);
  } else if cmd == "bandcamp" {
		// fprintf(f, "<iframe style='border: 0; width: 600px; height: 274px;' src='https://bandcamp.com/EmbeddedPlayer/album=%s/size=large/bgcol=ffffff/linkcol=333333/artwork=small' seamless></iframe>", target);
  } else if cmd == "youtube" {
		// fprintf(f, "<iframe width='600' height='380' src='https://www.youtube.com/embed/%s?rel=0' style='max-width:700px' frameborder='0' allow='autoplay; encrypted-media' allowfullscreen></iframe>", target);
	} else if cmd == "redirect" {
		// fprintf(f, "<meta http-equiv='refresh' content='2; url=%s.html'/><p>In a hurry? Travel to <a href='%s.html'>%s</a>.</p>", target, target, target);
	} else if cmd == "img" {
    let target_chars = &target.chars().collect::<Vec<char>>();
		let split2 = helpers::cpos(target_chars, ' ');
		if split2 > 0 {
      let mut param: String;
      let mut value: String;
      let _split2 = split2 as usize;
			param = helpers::sstr(target_chars, 0, _split2);
			value = helpers::sstr(target_chars, _split2 + 1, target.len() - _split2);
      // fprintf(f, "<img src='../media/%s' width='%s'/>&nbsp;", param, value);
      println!("<img src='../media/{}' width='{}'/>", param, value);
		} else {
			// fprintf(f, "<img src='../media/%s'/>&nbsp;", target);
    }
	} else if cmd == "src" {
		let lines = 0;
		// let c: &[char];
    let mut scanner: Scanner;

		// to build special section (eg. codeblock see `ansi_c.html`) 
    // by pulling texts from ../archive/src
    let f = File::open(format!("../archive/src/{}.txt", target)).expect("fpmodule: Missing src include");
    let mut f_reader = BufReader::new(f);
    let mut line = String::new();
		// fputs("<figure>", f);
		// fputs("<pre>", f);

    while f_reader.read_line(&mut line).unwrap() > 0 {
      scanner = Scanner::new(&line);
      for c in scanner.source.iter() {
        if c == &'<' {
        	// fputs("&lt;", f);
        } else if c == &'>' {
        	// fputs("&gt;", f);
        } else if c == &'&' {
        	// fputs("&amp;", f);
        } else {
        	// fputc(c, f);
        }
        if c == &'\n' {
        	// lines += 1;
        }
      }
		}
		// fputs("</pre>", f);
		// fprintf(f, "<figcaption><a href='../archive/src/%s.txt'>%s</a> %d lines</figcaption>\n", target, target, lines);
		// fputs("</figure>", f);
	} else {
		println!("Warning: Missing template mod: {:?}", s);
  }
}

// fn fplink(f: String, lex: &Lexicon, t: &Term, s: &str) {
// 	int split = cpos(s, ' ');
// 	char target[256], name[256];
// 	/* find target and name */
// 	if(split == -1) {
// 		sstr(s, target, 0, slen(s));
// 		scpy(target, name);
// 	} else {
// 		sstr(s, target, 0, split);
// 		sstr(s, name, split + 1, slen(s) - split);
// 	}
// 	/* output */
// 	if(surl(target)) {
// 		if(f != NULL)
// 			fprintf(f, "<a href='%s' target='_blank'>%s</a>", target, name);
// 	} else {
// 		Term* tt = findterm(lex, target);
// 		if(!tt)
// 			error("Unknown link", target);
// 		if(f != NULL)
// 			fprintf(f, "<a href='%s.html'>%s</a>", tt->filename, name);
// 		else {
// 			tt->incoming[tt->incoming_len] = t;
// 			tt->incoming_len++;
// 			t->outgoing_len++;
// 		}
// 	}
// }

fn findterm(lex: &Lexicon, name: &str) -> Option<Rc<RefCell<Term>>> {
  let mut _name = String::with_capacity(name.len());
  _name  = name.to_lowercase().replace("_", " ");
	for i in 0..lex.len {
    if &_name == &lex.terms[i as usize].borrow().name  {
      return Some(lex.terms[i as usize].clone());
    }
  }
	return None;
}

fn findlist(glo: &Glossary, name: &str) -> Option<Rc<RefCell<List>>>{
  let mut _name = String::with_capacity(name.len());
  _name  = name.to_lowercase().replace("_", " ");
	for i in 0..glo.len{
    if &_name == &glo.lists[i as usize].borrow().name {
      return Some(glo.lists[i as usize].clone());
    }
  }
	return None;
}


pub fn scan(content: &str)  {
  println!("Scanner  | ");
  scan_glossary(&content);
}


fn main() {
  let all_terms = &mut Lexicon::new();
  let all_lists = &mut Glossary::new();
  let all_logs = &mut Journal::new();
  
  parse(all_lists, all_terms, all_logs);
  link(all_lists, all_terms, all_logs).unwrap();
  
}
