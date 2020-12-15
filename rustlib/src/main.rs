#![allow(
  dead_code,
  unused_imports,
  unused_variables,
  unused_assignments,
  unused_mut
)]

use std::cell::{RefCell, RefMut};
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::io::LineWriter;
use std::io::{self, BufReader, Read, Write};
use std::path::Path;
use std::rc::Rc;
use std::time::{Duration, SystemTime};

mod util;
use std::error::Error;
use util::error::SkiffError;
use util::helpers;
use util::parser::Parser;
use util::scanner::Scanner;
use util::token::Token;

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
  routes: i32,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct Term {
  name: String, // title
  host: String, // HOST, nothing fancy.
  bref: String,
  r#type: String,
  body: Vec<String>, // BODY list item (no linking in parsing process ).
  body_len: usize,   // BODY list item counter.
  link: List,        // LINK item ( empty `.name` field eg. link.name = "" )
  list: Vec<String>, // LIST field.
  list_len: i32,
  /* generated */
  filename: String,           // generate from name field.
  date_from: RefCell<String>, // use RefCell, since this field shared mutability.
  date_last: RefCell<String>,
  parent: Option<Box<Rc<RefCell<Term>>>>,
  children: Vec<Option<Box<Rc<RefCell<Term>>>>>,
  children_len: i32,
  docs: Vec<Option<Rc<RefCell<List>>>>,
  docs_len: i32,
  incoming: Vec<Box<Rc<RefCell<Term>>>>,
  incoming_len: i32,
  outgoing_len: i32,
}

#[derive(Clone, Debug)]
pub struct Log {
  date: String,
  rune: String,
  code: i32,
  host: String,
  pict: i32,
  name: String,
  term: Option<Rc<RefCell<Term>>>,
}

#[derive(Debug)]
pub struct Glossary {
  len: i32,
  lists: Vec<Rc<RefCell<List>>>,
}

#[derive(Clone, Debug)]
pub struct Lexicon {
  len: i32,
  terms: Vec<Rc<RefCell<Term>>>,
}

#[derive(Debug)]
pub struct Journal {
  len: i32,
  logs: Vec<Rc<RefCell<Log>>>,
}

impl List {
  fn new() -> List {
    List {
      name: String::with_capacity(KEY_BUF_LEN),
      keys: vec![],
      vals: vec![],
      len: 0,
      routes: 0,
    }
  }
}

impl Term {
  fn new() -> Term {
    Term {
      name: String::with_capacity(KEY_BUF_LEN),
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
      incoming: vec![],
      incoming_len: 0,
      outgoing_len: 0,
    }
  }

  fn get_name(&self) -> String {
    self.name.to_string()
  }
}

impl Log {
  pub fn new() -> Log {
    Log {
      date: String::with_capacity(6),
      rune: String::new(),
      code: 0,
      host: String::with_capacity(KEY_BUF_LEN),
      pict: 0,
      name: String::with_capacity(LOG_BUF_LEN),
      term: None,
    }
  }
}

impl Glossary {
  fn new() -> Glossary {
    Glossary {
      len: 0,
      lists: vec![Rc::new(RefCell::new(List::new()))],
    }
  }

  fn update_len(&mut self) {
    self.len += 1;
  }
}

impl Lexicon {
  fn new() -> Lexicon {
    Lexicon {
      len: 0,
      terms: vec![Rc::new(RefCell::new(Term::new()))],
    }
  }
}

impl Journal {
  fn new() -> Journal {
    Journal {
      len: 0,
      logs: vec![Rc::new(RefCell::new(Log::new()))],
    }
  }
}

fn scan_glossary(content: &str) {
  let mut tokens: Vec<Token> = Vec::new();
  let mut scanner = Scanner::new(&content);
  tokens = scanner.scan_tokens();
}

// ------------------MAIN-----------------------

fn parse(all_lists: &mut Glossary, all_terms: &mut Lexicon, all_logs: &mut Journal) {
  println!("Parsing  | ");
  parse_glossary(String::from("./database/glossary.ndtl"), all_lists).unwrap();
  parse_lexicon(String::from("./database/lexicon.ndtl"), all_terms).unwrap();
  parse_horaire(String::from("./database/horaire.ndtl"), all_logs).unwrap();
}

fn link(glo: &mut Glossary, lex: &mut Lexicon, jou: &mut Journal) -> Result<(), SkiffError> {
  println!("Linking  | ");
  for i in 0..jou.len {
    let jou_logs = &mut jou.logs[i as usize];
    let host_name = jou_logs.borrow().host.to_string();
    let jou_log_date = jou_logs.borrow().date.to_string();

    match findterm(lex, &host_name) {
      Some(t) => jou_logs.borrow_mut().term = Some(t),
      None => jou_logs.borrow_mut().term = None,
    }

    match &jou_logs.borrow_mut().term {
      Some(_t) => {
        let _t_len = _t.borrow().date_last.borrow().len();
        if _t_len == 0 {
          _t.borrow().date_last.borrow_mut().push_str(&jou_log_date);
        }
        _t.borrow().date_from.borrow_mut().push_str(&jou_log_date);
      }
      None => {}
    }
  }
  println!("Linking: lexicon({} entries) ", lex.len);

  for i in 0..lex.len {
    let lex_term = &lex.terms[i as usize];
    let lext_t_clone = lex_term.borrow().body.clone();
    for (idx, j) in lext_t_clone.iter().enumerate() {
      ftemplate(None, lex, &lex_term.clone().borrow(), j).unwrap();
    }
    let host_name = lex_term.borrow().host.to_string();
    let mut ch_len = 0;
    if let Some(len) = *&lex_term.borrow_mut().parent.as_ref() {
      ch_len = len.borrow().children_len as usize;
    }

    match findterm(lex, &host_name) {
      Some(t) => {
        lex_term.borrow_mut().parent = Some(Box::new(t));
        let mut parent_term = lex_term.borrow().parent.as_ref().unwrap().clone();
        parent_term
          .borrow_mut()
          .children
          .insert(ch_len, Some(Box::new(lex_term.clone())));
        parent_term.borrow_mut().children_len += 1;
      }
      None => {
        lex_term.borrow_mut().parent = None;
        println!("Linking: Unknown term host = {}", &host_name.to_string());
      }
    }
  }

  println!("Linking: glossary({} entries) ", glo.len);

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
        }
        None => {
          lext.parent = None;
          println!(
            "Linking: Unknown list = {}",
            &lex_terms_list[j as usize].to_string()
          );
        }
      }
    }
  }
  Ok(())
}

fn build(lex: &Lexicon, jou: &Journal) -> Result<(), SkiffError> {
  let mut file;
  let mut file_writer;

  println!("Building | ");
  println!("{} pages ", lex.len);
  for i in 0..lex.len {
    let lex_term = lex.terms[i as usize].as_ref().borrow_mut().clone();
    let filepath: String = format!("{}/{}.{}", "../site/", lex_term.filename, "html");
    let path = Path::new(&filepath);
    let display = path.display();
    file_writer = match File::create(path) {
      Err(why) => panic!("couldn't create {}: {}", display, why),
      Ok(f) => f,
    };
    file = LineWriter::new(file_writer);
    build_page(&mut file, lex, &lex_term, jou).unwrap();
  }
  println!("2 feeds ");
  // file = File::open("../links/rss.xml").expect("build: Could not open file -> rss.xml" );
  // fprss(f, jou);
  // file = File::open("../links/tw.txt").expect("build: Could not open file -> tw.txt" );
  // fptwtxt(f, jou);
  Ok(())
}

fn check(lex: &Lexicon, glo: &Glossary, jou: &Journal) -> Result<(), Box<dyn Error>> {
  let mut found = 0;
  let mut sends = 0;
  println!("Checking | ");
  /* Find invalid logs */
  for log in jou.logs.iter() {
    if log.borrow().code < 1 {
      println!("Warning: Empty code {}\n", log.borrow().date);
    }
  }
  /* Find unlinked lists */
  for list in glo.lists.iter() {
    if list.borrow().routes < 1 {
      println!(
        "Warning: Unused (glossary)list \"{}\"\n",
        list.borrow().name
      );
    }
  }
  /* Find next available diary id */
  for i in 1..999 {
    found = 0;
    for j_log in jou.logs.iter() {
      if j_log.borrow().pict == i || found > 0 {
        found = 1;
      }
    }
    if found > 0 {
      println!("Available(#{}) ", i);
      break;
    }
  }
  /* Find unlinked pages */
  for term in lex.terms.iter() {
    sends += term.borrow().incoming_len;
    if term.borrow().incoming_len < 1 && term.borrow().outgoing_len < 1 {
      println!("Warning: \"{}\" unlinked", term.borrow().name);
    } else if term.borrow().incoming_len < 1 {
      println!("Warning: \"{}\" orphaned", term.borrow().name);
    } else if term.borrow().outgoing_len < 1 {
      println!("Warning: \"{}\" dead-end", term.borrow().name);
    }
  }
  println!("sends({} incomings) ", sends);

  Ok(())
}

// ------------------METHODS-----------------------

fn parse_glossary(path: String, glossary: &mut Glossary) -> Result<(), SkiffError> {
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
    if scanner.source.len() == 0 as usize {
      line.clear();
      continue;
    }

    match helpers::strm(&scanner.source) {
      Some(string) => {
        len = helpers::slen(string.trim_end().chars().collect::<Vec<char>>().as_ref())
      }
      None => len = 0,
    }
    // skip if it's comments or blank line.
    if len < 4 || &scanner.source[0] == &';' {
      line.clear();
      continue;
    }
    if len > 400 {
      return Err(SkiffError::ParseError(
        "Glossary Parsing: Line is too long".to_string(),
      ));
    }
    if depth == 0 {
      if l.borrow().len > 0 {
        glossary
          .lists
          .insert(glossary.len as usize, Rc::new(RefCell::new(List::new())));
        l = &mut glossary.lists[glossary.len as usize];
      }
      l.borrow_mut().name = scanner
        .source
        .into_iter()
        .collect::<String>()
        .to_lowercase();
      glossary.len += 1;
    } else if depth == 2 {
      // in case of list item( 2 spaces at beginning of line).
      if l.borrow().len >= LIST_ITEMS as i32 {
        return Err(SkiffError::ParseError(
          "Glossary Parsing: Reached LIST_ITEMS limit".to_string(),
        ));
      }
      split = helpers::cpos(&scanner.source, ':'); // find index of `:` return -1 if not found.
      let l_len = l.borrow().len as usize;
      if split < 0 {
        // handle only list which not include `:` in sentence.
        // return normal string.
        l.borrow_mut()
          .vals
          .insert(l_len, helpers::sstr(&scanner.source, 2, len + 2));
      } else {
        l.borrow_mut().keys.insert(
          l_len,
          helpers::sstr(&scanner.source, 2, (split - 3) as usize),
        ); // title of list line.
        l.borrow_mut().vals.insert(
          l_len,
          helpers::sstr(&scanner.source, (split + 2) as usize, len - split as usize),
        ); // details of list line.
      }
      l.borrow_mut().len += 1;
    }

    // clear to reuse the buffer
    // in other words, get freshly new line,
    // without cancating with prev line.
    line.clear();
  }
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

    if scanner.source.len() == 0 as usize {
      line.clear();
      continue;
    }

    match helpers::strm(&scanner.source) {
      Some(string) => len = string.len(),
      None => len = 0,
    }

    // len < 3 = skip 'newline' eg. '\n';
    if len < 3 || &scanner.source[0] == &';' {
      line.clear();
      continue;
    }

    if len > 750 {
      return Err(SkiffError::ParseError(
        "Lexicon Parsing: Line is too long".to_string(),
      ));
    }
    if depth == 0 {
      if lexicon.len > 0 {
        lexicon
          .terms
          .insert(lexicon.len as usize, Rc::new(RefCell::new(Term::new())));
        t = &mut lexicon.terms[lexicon.len as usize];
      }
      if !helpers::sans(&scanner.source) == 0 {
        println!(
          "Lexicon warning: {}",
          SkiffError::ParseError("Lexicon key is not alphanum".to_string())
        );
      }
      t.borrow_mut().name = helpers::sstr(&scanner.source, 0, len).to_lowercase();
      t.borrow_mut().filename = helpers::sstr(&scanner.source, 0, len)
        .replace(" ", "_")
        .to_lowercase();
      lexicon.len += 1;
    } else if depth == 2 {
      t = &mut lexicon.terms[(lexicon.len - 1) as usize];
      let _scanner = &scanner.source;
      if helpers::spos(&_scanner, "HOST : ") >= 0 {
        t.borrow_mut().host = helpers::sstr(&scanner.source, 9, len - 9);
      }
      if helpers::spos(&_scanner, "BREF : ") >= 0 {
        t.borrow_mut().bref = helpers::sstr(&scanner.source, 9, len - 9);
      }
      if helpers::spos(&_scanner, "TYPE : ") >= 0 {
        t.borrow_mut().r#type = helpers::sstr(&scanner.source, 9, len - 9);
      }
      catch_body = helpers::spos(&_scanner, "BODY") >= 0;
      catch_link = helpers::spos(&_scanner, "LINK") >= 0;
      catch_list = helpers::spos(&_scanner, "LIST") >= 0;
    } else if depth == 4 {
      t = &mut lexicon.terms[(lexicon.len - 1) as usize];
      /* Body */
      if catch_body {
        let _len = t.borrow().body_len;
        t.borrow_mut()
          .body
          .insert(_len, helpers::sstr(&scanner.source, 4, len - 4));
        t.borrow_mut().body_len += 1;
      }
      /* Link */
      if catch_link {
        key_len = (helpers::cpos(&scanner.source, ':') - 5) as usize;
        let link_len = t.borrow().link.len as usize;
        t.borrow_mut().link.keys.insert(
          link_len as usize,
          helpers::sstr(&scanner.source, 4, key_len),
        );
        val_len = len - key_len - 5;
        t.borrow_mut().link.vals.insert(
          link_len as usize,
          helpers::sstr(&scanner.source, key_len + 7, val_len),
        );
        t.borrow_mut().link.len += 1;
      }
      /* List */
      if catch_list {
        let list_len = t.borrow().list_len as usize;
        t.borrow_mut()
          .list
          .insert(list_len, helpers::sstr(&scanner.source, 4, len - 4));
        t.borrow_mut().list_len += 1;
      }
    }
    // count += 1;
    line.clear();
  }
  Ok(())
}

fn parse_horaire(path: String, journal: &mut Journal) -> Result<(), SkiffError> {
  let f = File::open(path).expect("journal parsing: file not found");
  let mut f_reader = BufReader::new(f);
  let mut len;
  let mut line = String::new();
  let mut scanner: Scanner;
  let mut log = &mut journal.logs[journal.len as usize];

  while f_reader.read_line(&mut line).unwrap() > 0 {
    scanner = Scanner::new(&line.trim_end());

    match helpers::strm(&scanner.source) {
      Some(string) => len = string.len(),
      None => len = 0,
    }

    if len < 14 || &scanner.source[0] == &';' {
      line.clear();
      continue;
    }

    if len > 72 {
      return Err(SkiffError::ParseError("Log is too long".to_string()));
    }
    if journal.len > 0 {
      journal
        .logs
        .insert(journal.len as usize, Rc::new(RefCell::new(Log::new())));
      log = &mut journal.logs[journal.len as usize];
    }
    /* Date */
    log.borrow_mut().date = helpers::sstr(&scanner.source, 0, 5);
    /* Rune */
    // log.borrow_mut().rune = &scanner.source[6];
    /* Code */
    log.borrow_mut().code = helpers::sint(&scanner.source[7..], 3) as i32;
    /* Term */
    // extract only `host` type.
    let mut split_line = line.split_whitespace().into_iter();
    log.borrow_mut().host = split_line.nth(2).unwrap().to_string();
    let _host = &log.borrow_mut().host.chars().collect::<Vec<char>>();

    // TODO: find better way to split without consume nth.
    /* Pict */
    let picture_id = split_line.nth(0).unwrap();
    if picture_id != "-" {
      log.borrow_mut().pict = picture_id.parse().unwrap();
    }
    /* Name */
    if let Some(code_col) = split_line.nth(0) {
      log.borrow_mut().name = code_col.to_string().replace("_", " ");
    }
    if !helpers::sans(_host) == 0 {
      println!("Warning: {} is not alphanum", log.borrow().host);
    }
    journal.len += 1;
    line.clear();
  }

  Ok(())
}

fn ftemplate(
  file: Option<&mut LineWriter<File>>,
  lex: &Lexicon,
  term: &Term,
  string: &str,
) -> Result<(), SkiffError> {
  let mut capture = false;
  let mut buf = vec![];
  let mut has_file = false;
  let mut _file = None;
  if let Some(f) = file {
    has_file = true;
    _file = Some(f);
  }

  let _s = string.chars().collect::<Vec<char>>();

  for i in 0.._s.len() {
    let c = _s[i];

    if c == '}' {
      capture = false;
      // check if it's module link ( eg, {^bandcamp 163410848} )
      if buf[0] == '^' && has_file {
        fpmodule(&mut _file, &buf);
      } else if buf[0] != '^' {
        // or normal link (eg, {methascope})
        fplink(&mut _file, &lex, &term, &buf).unwrap();
      }

      // handle multiple capture areas in same line.
      &buf.clear();
    }

    if capture {
      if buf.len() < STR_BUF_LEN - 1 {
        buf.push(c);
      } else {
        return Err(SkiffError::ParseError("template too long, s".to_string()));
      }
    } else if c != '{' && c != '}' && has_file {
      // write `BODY : `'s list to file except for 'capture(link)' statement.
      &mut _file
        .as_mut()
        .unwrap()
        .write_fmt(format_args!("{}", &c))
        .unwrap();
    }
    if c == '{' {
      capture = true;
    }
  }

  Ok(())
}

// build modules (eg. codeblock, iframe link)
fn fpmodule(f: &mut Option<&mut LineWriter<File>>, s: &[char]) {
  let file = f.as_mut().unwrap();
  let split = helpers::cpos(s, ' ');
  let mut cmd: String;
  let target: String;
  cmd = helpers::sstr(s, 1, (split - 1) as usize);
  target = helpers::sstr(s, (split + 1) as usize, helpers::slen(s) - split as usize);

  if cmd == "itchio" {
    file.write_fmt(format_args!("<iframe frameborder='0' src='https://itch.io/embed/{}?link_color=000000' width='600' height='167'></iframe>", target)).unwrap();
  } else if cmd == "bandcamp" {
    file.write_fmt(format_args!("<iframe style='border: 0; width: 600px; height: 274px;' src='https://bandcamp.com/EmbeddedPlayer/album={}/size=large/bgcol=ffffff/linkcol=333333/artwork=small' seamless></iframe>", target)).unwrap();
  } else if cmd == "youtube" {
    file.write_fmt(format_args!("<iframe width='600' height='380' src='https://www.youtube.com/embed/{}?rel=0' style='max-width:700px' frameborder='0' allow='autoplay; encrypted-media' allowfullscreen></iframe>", target)).unwrap();
  } else if cmd == "redirect" {
    file.write_fmt(format_args!("<meta http-equiv='refresh' content='2; url={}.html'/><p>In a hurry? Travel to <a href='{}.html'>{}</a>.</p>", target, target, target)).unwrap();
  } else if cmd == "img" {
    let target_chars = &target.chars().collect::<Vec<char>>();
    let split2 = helpers::cpos(target_chars, ' ');
    if split2 > 0 {
      let mut param: String;
      let mut value: String;
      let _split2 = split2 as usize;
      param = helpers::sstr(target_chars, 0, _split2);
      value = helpers::sstr(target_chars, _split2 + 1, target.len() - _split2);
      file
        .write_fmt(format_args!(
          "<img src='../media/{}' width='{}'/>&nbsp;",
          param, value
        ))
        .unwrap();
      file
        .write_fmt(format_args!(
          "<img src='../media/{}' width='{}'/>",
          param, value
        ))
        .unwrap();
    } else {
      file
        .write_fmt(format_args!("<img src='../media/{}'/>&nbsp;", target))
        .unwrap();
    }
  } else if cmd == "src" {
    let mut lines = 0;
    let mut scanner: Scanner;

    // to build special section (eg. codeblock see `ansi_c.html`)
    // by pulling texts from ../archive/src
    println!("target = {}", target);
    let f =
      File::open(format!("../archive/src/{}.txt", target)).expect("fpmodule: Missing src include");
    let mut f_reader = BufReader::new(f);
    let mut line = String::new();
    file.write(b"<figure>").unwrap();
    file.write(b"<pre>").unwrap();

    while f_reader.read_line(&mut line).unwrap() > 0 {
      scanner = Scanner::new(&line);
      for c in scanner.source.iter() {
        if c == &'<' {
          file.write(b"&lt;").unwrap();
        } else if c == &'>' {
          file.write(b"&gt;").unwrap();
        } else if c == &'&' {
          file.write(b"&amp;").unwrap();
        } else {
          file.write_fmt(format_args!("{}", c)).unwrap();
        }
        if c == &'\n' {
          lines += 1;
        }
      }
    }
    file.write(b"</pre>").unwrap();
    file
      .write_fmt(format_args!(
        "<figcaption><a href='../archive/src/{}.txt'>{}</a> {} lines</figcaption>\n",
        target, target, lines
      ))
      .unwrap();
    file.write(b"</figure>").unwrap();
  } else {
    println!("Warning: Missing template mod: {:?}", s);
  }
}

fn fplink(
  file: &mut Option<&mut LineWriter<File>>,
  lex: &Lexicon,
  term: &Term,
  s: &[char],
) -> Result<(), SkiffError> {
  let split = helpers::cpos(s, ' ');
  let mut target: String;
  let mut name: Vec<char> = vec![];
  /* find target and name */
  if split == -1 {
    target = helpers::sstr(s, 0, helpers::slen(s) + 1);
    name = target.chars().collect::<Vec<char>>();
  } else {
    target = helpers::sstr(s, 0, split as usize).trim_end().to_string();
    name = helpers::sstr(s, (split + 1) as usize, helpers::slen(s) - (split as usize))
      .chars()
      .collect::<Vec<char>>();
  }
  /* output */
  if helpers::surl(&target) {
    if file.is_some() {
      file
        .as_mut()
        .unwrap()
        .write_fmt(format_args!(
          "<a href='{}' target='_blank'>{}</a>",
          target,
          name.iter().collect::<String>()
        ))
        .unwrap();
    }
  } else {
    match findterm(lex, &target) {
      Some(_t) => {
        if file.is_some() {
          file
            .as_mut()
            .unwrap()
            .write_fmt(format_args!(
              "<a href='{}.html'>{}</a>",
              _t.borrow().filename,
              name.iter().collect::<String>()
            ))
            .unwrap();
        } else {
          let mut _term = _t.as_ref().clone();
          let len = _term.borrow().incoming_len as usize;
          _term.borrow_mut().outgoing_len += 1;
          _term
            .borrow_mut()
            .incoming
            .insert(len, Box::new(_t.clone()));
          _term.borrow_mut().incoming_len += 1;
        }
      }
      None => {
        // println!("Unknown link = {:?}", &target);
        return Err(SkiffError::ParseError("Unknown link".to_string()));
      }
    }
  }

  Ok(())
}

fn build_page(
  file: &mut LineWriter<File>,
  lex: &Lexicon,
  term: &Term,
  jou: &Journal,
) -> Result<(), std::io::Error> {
  file.write(b"<!DOCTYPE html>")?;
  file.write(b"<html lang='en'>")?;
  file.write(b"<head>")?;
  file.write(b"<meta charset='utf-8'>")?;
  file.write_fmt(format_args!(
    "<meta name='description' content='{}'/>",
    term.bref
  ))?;
  file.write_fmt(format_args!(
    "<meta name='thumbnail' content='{}media/services/thumbnail.jpg' />",
    DOMAIN
  ))?;
  file.write(
    b"<link rel='alternate' type='application/rss+xml' title='RSS Feed' href='../links/rss.xml' />",
  )?;
  file.write(b"<link rel='stylesheet' type='text/css' href='../links/main.css'>")?;
  file.write(b"<link rel='shortcut icon' type='image/png' href='../media/services/icon.png'>")?;
  file.write_fmt(format_args!(
    "<title>{} — {}</title>",
    term.name.to_uppercase(),
    term.name
  ))?;
  file.write(b"</head>")?;
  file.write(b"<body>")?;
  file.write_fmt(format_args!("<header><a href='home.html'><img src='../media/identity/xiv28.gif' alt='{}' height='29'></a></header>", term.name.to_uppercase()))?;
  build_nav(file, &term).unwrap();
  file.write(b"<main>")?;
  build_banner(file, jou, term, 1).unwrap();
  build_body(file, lex, term).unwrap();
  build_include(file, term).unwrap();
  /* templated pages */
  match term.r#type.as_ref() {
    "portal" => build_portal(file, jou, term).unwrap(),
    "album" => build_album(file, jou, term).unwrap(),
    "index" => build_index(file, lex, term).unwrap(),
    _ => {}
  };
  /* special pages */
  match term.name.as_ref() {
    // "now" => build_special_now(file, lex, jou).unwrap(),
    "home" => build_special_home(file, jou).unwrap(),
    "calendar" => build_special_calendar(file, jou).unwrap(),
    "tracker" => build_special_tracker(file, jou).unwrap(),
    "journal" => build_special_journal(file, jou).unwrap(),
    "index" => build_special_index(file, lex).unwrap(),
    _ => {}
  };
  build_list(file, term).unwrap();
  build_links(file, term).unwrap();
  build_incoming(file, term).unwrap();
  // build_horaire(file, jou, term).unwrap();
  file.write(b"</main>")?;
  file.write(b"<footer>")?;
  file.write(b"<a href='https://creativecommons.org/licenses/by-nc-sa/4.0'><img src='../media/icon/cc.svg' width='30'/></a>")?;
  file.write(
    b"<a href='http://webring.xxiivv.com/'><img src='../media/icon/rotonde.svg' width='30'/></a>",
  )?;
  file.write(b"<a href='https://merveilles.town/@neauoire'><img src='../media/icon/merveilles.svg' width='30'/></a>")?;
  file.write(b"<a href='https://github.com/neauoire'><img src='../media/icon/github.png' alt='github' width='30'/></a>")?;
  file.write(b"<span><a href='devine_lu_linvega.html'>Devine Lu Linvega</a> \xA9 2020 \x97 <a href='about.html'>BY-NC-SA 4.0</a></span>")?;
  file.write(b"</footer>")?;
  file.write(b"</body></html>")?;
  Ok(())
}

fn build_nav(file: &mut LineWriter<File>, term: &Term) -> Result<(), Box<dyn Error>> {
  file.write(b"<nav>")?;
  match &term.parent {
    Some(t_parent) => {
      if let Some(t_parent_parent) = &t_parent.clone().borrow().parent {
        let t_parent_name = t_parent.clone().borrow().name.to_string();
        let t_parent_parent_name = t_parent_parent.clone().borrow().name.to_string();
        if t_parent_parent_name == t_parent_name {
          let t_parent_parent_clone = t_parent_parent.clone();
          build_nav_part(file, &t_parent_parent_clone.borrow(), &term)?;
        } else {
          build_nav_part(file, &t_parent_parent.borrow(), &t_parent.borrow())?;
        }

        if t_parent_parent_name != t_parent_name {
          let t_parent_parent_clone = t_parent_parent.clone();
          build_nav_part(file, &t_parent.borrow(), &term)?;
        }

        if t_parent_name != term.name.to_string() {
          build_nav_part(file, &term.clone(), &term.clone())?;
        }
      }
    }
    None => println!("parent None"),
  }
  file.write(b"</nav>")?;
  Ok(())
}

fn build_nav_part(
  file: &mut LineWriter<File>,
  term: &Term,
  target: &Term,
) -> Result<(), Box<dyn Error>> {
  file.write(b"<ul>")?;
  for i in 0..term.children_len {
    let term_children = term.children[i as usize].as_ref().unwrap().borrow().clone();
    if &term_children.name == &term.name {
      continue; /* Paradox */
    }

    let filename = term_children.filename.to_string();
    let name = term_children.name.to_string();

    if &term_children.name == &target.name.to_string() {
      // add symbol "/" at the end as a current actived (eg. `about/` ).
      file.write_fmt(format_args!(
        "<li><a href='{}.html'>{}/</a></li>",
        filename, name
      ))?;
    } else {
      file.write_fmt(format_args!(
        "<li><a href='{}.html'>{}</a></li>",
        filename, name
      ))?;
    }
  }
  file.write(b"</ul>")?;
  Ok(())
}

fn build_banner(
  file: &mut LineWriter<File>,
  jou: &Journal,
  term: &Term,
  caption: i32,
) -> Result<(), Box<dyn Error>> {
  let log = finddiary(jou, term);
  if let Some(_log) = log {
    build_log_pict(file, &_log.borrow(), caption).unwrap();
  }

  Ok(())
}

fn build_body(
  file: &mut LineWriter<File>,
  lex: &Lexicon,
  term: &Term,
) -> Result<(), Box<dyn Error>> {
  file.write_fmt(format_args!("<h2>{}</h2>", &term.bref))?;
  build_body_part(file, lex, &term);
  Ok(())
}

fn build_body_part(file: &mut LineWriter<File>, lex: &Lexicon, term: &Term) {
  for term_body in term.body.iter() {
    ftemplate(Some(file), lex, term, term_body).unwrap();
  }
}

fn build_include(file: &mut LineWriter<File>, term: &Term) -> Result<(), Box<dyn Error>> {
  let filepath: String = format!("{}/{}.{}", "../inc", term.filename, "htm");
  let path = Path::new(&filepath);
  let mut buff;
  match fs::read_to_string(path) {
    Ok(c) => buff = c,
    _ => return Ok(()),
  }

  file
    .write_all(buff.as_bytes())
    .expect("error: cannot write /inc file");
  file.write_fmt(format_args!("<p>Found a mistake? Submit an <a href='\" REPOPATH \"{}.htm' target='_blank'>edit</a> to {}.</p>", term.filename, term.name))?;
  Ok(())
}

fn build_list(file: &mut LineWriter<File>, term: &Term) -> Result<(), Box<dyn Error>> {
  for (i, doc) in term.docs.iter().enumerate() {
    if let Some(d) = doc {
      let _doc = d.as_ref().borrow().clone();
      file.write_fmt(format_args!("<h3>{}</h3>", _doc.name))?;
      file.write(b"<ul>")?;
      for j in 0.._doc.len {
        let _j = j as usize;
        if _doc.keys.len() == 0 {
          file.write_fmt(format_args!("<li>{}</li>", _doc.vals[_j]))?;
        } else if helpers::surl(&_doc.vals[_j]) {
          file.write_fmt(format_args!(
            "<li><a href='{}'>{}</a></li>",
            _doc.vals[_j], _doc.keys[_j]
          ))?;
        } else {
          file.write_fmt(format_args!(
            "<li><b>{}</b>: {}</li>",
            _doc.keys[_j], _doc.vals[_j]
          ))?;
        }
      }
      file.write(b"</ul>")?;
    }
  }
  Ok(())
}

fn build_links(file: &mut LineWriter<File>, term: &Term) -> Result<(), Box<dyn Error>> {
  if term.link.len < 1 {
    return Ok(());
  }
  file.write(b"<ul>")?;
  for i in 0..term.link.len {
    file.write_fmt(format_args!(
      "<li><a href='{}' target='_blank'>{}</a></li>",
      term.link.vals[i as usize], term.link.keys[i as usize]
    ))?
  }
  file.write(b"</ul>")?;
  Ok(())
}

fn build_incoming(file: &mut LineWriter<File>, term: &Term) -> Result<(), Box<dyn Error>> {
  if term.incoming_len < 1 {
    return Ok(());
  }
  file.write(b"<p>")?;
  file.write_fmt(format_args!("<i>incoming({})</i>: ", term.incoming_len))?;
  for i in 0..term.incoming_len {
    let incoming = &term.incoming[i as usize].borrow();
    file.write_fmt(format_args!(
      "<a href='{}.html'>{}</a> ",
      incoming.filename, incoming.name
    ))?;
  }
  file.write(b"</p>")?;
  Ok(())
}

// fn build_horaire(
//   file: &mut LineWriter<File>,
//   jou: &Journal,
//   term: &Term,
// ) -> Result<(), Box<dyn Error>> {
//   let mut len = 0;
//   let mut events_len = 0;
//   let mut ch = 0;
//   let mut fh = 0;

//   for l in jou.logs.iter() {
//     let log = l.as_ref().borrow().clone();
//     let log_term = log.term.as_ref().unwrap().borrow().clone();
//     if log_term.name != term.name && log.term.parent != term {
//       continue;
//     }
//     if log.rune == '+'.to_string() {
//       events_len += 1;
//     }
//     ch += (log.code / 10) % 10;
//     fh += log.code % 10;
//     len += 1;
//   }
//   /* Updated */
//   // if len < 2 || helpers::slen(&term.date_last.borrow()) == 0 {
//   if len < 2 || term.date_last.borrow().len() == 0 {
//     return Ok(());
//   }
//   file.write(b"<p>")?;
//   file.write_fmt(format_args!(
//     "<i>Last update on <a href='tracker.html'>{}</a>, edited {} times. +{}/{}fh</i>",
//     term.date_last.borrow().to_string(),
//     len,
//     ch,
//     fh
//   ))?;
//   // build_lifeline(f, t);
//   file.write(b"</p>")?;
//   /* Events */
//   if events_len < 1 {
//     return Ok(());
//   }
//   file.write(b"<ul>")?;
//   for l in jou.logs.iter() {
//     let log = l.as_ref().borrow().clone();
//     let log_term = log.term.as_ref().unwrap().borrow().clone();
//     if log.rune != '+'.to_string() {
//       continue;
//     }
//     if log_term.name != term.name && log_term.parent.unwrap().borrow().name != term.name {
//       continue;
//     }
//     file.write_fmt(format_args!("<li>{} — {}</li>", log.date, log.name))?;
//   }
//   file.write(b"</ul>")?;
//   Ok(())
// }

fn build_portal(
  file: &mut LineWriter<File>,
  jou: &Journal,
  term: &Term,
) -> Result<(), Box<dyn Error>> {
  for term_children in term.children.iter() {
    if let Some(_t) = term_children {
      let mut _term = _t.as_ref().borrow().clone();
      if let Some(l) = finddiary(jou, &_term) {
        build_pict(
          file,
          l.borrow().pict,
          &_term.name,
          &_term.bref,
          1,
          Some(&_term.filename),
        )?;
      }
    }
  }
  Ok(())
}

fn build_album(
  file: &mut LineWriter<File>,
  jou: &Journal,
  term: &Term,
) -> Result<(), Box<dyn Error>> {
  for log in jou.logs.iter() {
    let journal_log = log.as_ref().borrow().clone();
    if journal_log.term.unwrap().borrow().name != term.name
      || journal_log.pict < 1
      || journal_log.pict == finddiary(jou, term).unwrap().borrow().pict
    {
      continue;
    }
    build_log_pict(file, &log.borrow(), 1).unwrap();
  }
  Ok(())
}

fn build_index(
  file: &mut LineWriter<File>,
  lex: &Lexicon,
  term: &Term,
) -> Result<(), Box<dyn Error>> {
  println!("build_index");
  for child in term.children.iter() {
    if let Some(c) = child {
      let mut _c = c.as_ref().borrow().clone();
      file.write_fmt(format_args!(
        "<h3><a href='{}.html'>{}</a></h3>",
        _c.filename, _c.name,
      ))?;
      build_body_part(file, lex, &_c);
      build_list(file, &_c).unwrap();
    }
  }

  Ok(())
}

fn build_special_home(
  file: &mut LineWriter<File>,
  journal: &Journal,
) -> Result<(), Box<dyn Error>> {
  let mut events = 0;
  for i in 0..5 {
    if journal.logs[i].borrow().rune == '+'.to_string() {
      events = 1;
      break;
    }
  }
  if events > 0 {
    return Ok(());
  }
  file.write(b"<h2>Events</h2>")?;
  file.write(b"<ul>")?;
  for i in 0..5 {
    let jou_log = journal.logs[i].as_ref().borrow().clone();
    if jou_log.rune != '+'.to_string() {
      continue;
    }
    file.write_fmt(format_args!(
      "<li><a href='{}.html'>{}</a>{}</li>",
      jou_log.term.unwrap().borrow().filename,
      jou_log.date,
      jou_log.name
    ))?;
  }
  file.write(b"</ul>")?;

  Ok(())
}

fn build_special_calendar(
  file: &mut LineWriter<File>,
  journal: &Journal,
) -> Result<(), Box<dyn Error>> {
  let mut last_year = 0;
  file.write(b"<ul>")?;
  for log in journal.logs.iter() {
    let journal_log = log.as_ref().borrow().clone();
    if log.borrow().rune != '+'.to_string() {
      continue;
    }

    let date = journal_log.date.chars().collect::<Vec<char>>();
    if last_year != helpers::sint(&date, 2) {
      file.write(b"</ul><ul>")?;
    }

    file.write_fmt(format_args!(
      "<li><a href='{}.html'>{}</a> {}</li>",
      journal_log.term.unwrap().borrow().filename,
      journal_log.date,
      journal_log.name
    ))?;
    last_year = helpers::sint(&date, 2);
  }
  file.write(b"</ul>")?;
  Ok(())
}

fn build_special_tracker(
  file: &mut LineWriter<File>,
  journal: &Journal,
) -> Result<(), Box<dyn Error>> {
  let mut known_id: usize = 0;
  let mut last_year = 20;
  file.write(b"<ul>")?;
  let mut known = Vec::with_capacity(LEXICON_BUFFER);
  for log in journal.logs.iter() {
    let journal_log = log.as_ref().borrow().clone();
    let term = journal_log.term.as_ref().unwrap().borrow().clone();
    let date = journal_log.date.chars().collect::<Vec<char>>();
    if known.len() > 0 {
      if helpers::afnd(&known, known_id, &term.name) > -1 {
        continue;
      }
    }
    if known_id >= LEXICON_BUFFER {
      println!("Warning: Reached tracker buffer\n");
      break;
    }
    if last_year != helpers::sint(&date, 2) {
      file.write(b"</ul><ul>")?;
    }
    file.write(b"<li>")?;
    file.write_fmt(format_args!(
      "<a href='{}.html'>{}</a> — last update {}",
      &term.filename, &term.name, &journal_log.date
    ))?;

    // build_lifeline(f, journal_log.term);
    file.write(b"</li>")?;
    last_year = helpers::sint(&date, 2);
    &known.push(term.name.to_string());
    known_id += 1;
  }
  file.write(b"</ul>")?;
  Ok(())
}

fn build_special_journal(
  file: &mut LineWriter<File>,
  journal: &Journal,
) -> Result<(), Box<dyn Error>> {
  let mut count = 0;
  for log in journal.logs.iter() {
    let journal_log = log.as_ref().borrow().clone();
    if count > 20 {
      break;
    }
    if journal_log.pict == 0 {
      continue;
    }
    build_log_pict(file, &journal_log, 1)?;
    count += 1;
  }
  Ok(())
}

fn build_special_index(file: &mut LineWriter<File>, lex: &Lexicon) -> Result<(), Box<dyn Error>> {
  file.write(b"<ul>")?;
  print_term_details(file, &lex.terms[0].as_ref().borrow().clone(), &mut 0)?;
  file.write(b"</ul>")?;
  Ok(())
}

// fn build_special_now(file: &mut LineWriter<File>, lex: &Lexicon, jou: &Journal) -> Result<(), Box<dyn Error>> {
//   let mut projects_len = 0;
//   let mut pname = vec![];
//   let mut pfname = vec![];
//   let sum_value = 0;
//   let pval = vec![];
//   let pmaxval = 0;
//   let mut epoch = SystemTime::now();

// 	for i in 0..LOGS_RANGE {
// 		let index = 0;
// 		let l = jou.logs[i];
// 		if epoch - arvelie_to_epoch(l.date) > LOGS_RANGE {
// 			break;
//     }
// 		index = afnd(pname, projects_len, l.term.name);
// 		if index < 0 {
// 			index = projects_len;
// 			pname[index] = l.term.name;
// 			pfname[index] = l.term.filename;
// 			pval[index] = 0;
// 			projects_len += 1;
// 		}
// 		pval[index] += l.code % 10;
// 		sum_value += l.code % 10;
// 	}
// 	/* find most active with a photo */
// 	for i in 0..projects_len {
// 		if finddiary(jou, findterm(lex, pname[i])) && pval[i] > pmaxval {
// 			pmaxval = pval[i];
//     }
// 	}
// 	for i in 0..projects_len {
// 		if pval[i] != pmaxval {
// 			continue;
//     }
// 		build_log_pict(f, finddiary(jou, findterm(lex, pname[i])), 1);
// 	}
// 	fprintf(
// 	    f,
// 	    "<p>This data shows the distribution of <b>%.0f hours over %d projects</b>, "
// 	    "recorded during the last %d days, for an average of %.1f work hours per day "
// 	    "and %.1f work hours per project.</p>",
// 	    sum_value, projects_len,
// 	    LOGS_RANGE, sum_value / LOGS_RANGE,
// 	    sum_value / projects_len);
// 	fputs("<ul style='columns:2'>", f);
// 	for i in 0..projects_len {
// 		fputs("<li>", f);
// 		fprintf(f, "<a href='%s.html'>%s</a> %.2f&#37; ",
// 		        pfname[i],
// 		        pname[i],
// 		        pval[i] / sum_value * 100);
// 		fputs("</li>", f);
// 	}
// 	fputs("</ul>", f);
//   fprintf(f, "<p>Last generated on %s(" LOCATION ").</p>", nowstr());
//   Ok(())
// }

fn build_log_pict(
  file: &mut LineWriter<File>,
  log: &Log,
  caption: i32,
) -> Result<(), Box<dyn Error>> {
  build_pict(file, log.pict, &log.date, &log.name, caption, None)
}

fn build_pict(
  file: &mut LineWriter<File>,
  pict: i32,
  host: &str,
  name: &str,
  caption: i32,
  link: Option<&str>,
) -> Result<(), Box<dyn Error>> {
  file.write(b"<figure>")?;
  file.write_fmt(format_args!(
    "<img src='../media/images/{}.jpg' alt='{} picture' width='900'/>",
    pict, name
  ))?;
  if caption > 0 {
    file.write(b"<figcaption>")?;
    if let Some(_link) = link {
      file.write_fmt(format_args!(
        "<a href='{}.html'>{}</a> — {}",
        _link, host, name
      ))?;
    } else {
      file.write_fmt(format_args!("{} — {}", host, name))?;
    }
    file.write(b"</figcaption>")?;
  }
  file.write(b"</figure>")?;
  Ok(())
}

fn print_term_details(
  file: &mut LineWriter<File>,
  term: &Term,
  depth: &mut i32,
) -> Result<(), Box<dyn Error>> {
  *depth += 1;
  file.write_fmt(format_args!(
    "<li><a href='{}.html'>{}</a></li>",
    term.filename, term.name
  ))?;
  if term.children_len < 1 {
    return Ok(());
  }
  file.write(b"<ul>")?;
  for child in term.children.iter() {
    if let Some(_child) = child {
      if !helpers::scmp(&_child.borrow().name, &term.name) {
        print_term_details(file, &_child.as_ref().borrow().clone(), depth)?;
      }
    }
  }
  file.write(b"</ul>")?;
  Ok(())
}

// ------------------HELPERS-----------------------

fn finddiary(jou: &Journal, term: &Term) -> Option<Rc<RefCell<Log>>> {
  for log in jou.logs.iter() {
    let jou_log = &log.borrow();
    let log_term = jou_log.term.as_ref();
    if log_term.unwrap().borrow().name != term.name || log.borrow().pict < 1 {
      continue;
    }
    return Some(log.clone());
  }
  return None;
}

fn findterm(lex: &Lexicon, name: &str) -> Option<Rc<RefCell<Term>>> {
  let mut _name = String::with_capacity(name.len());
  _name = name.to_lowercase().replace("_", " ");
  for term in lex.terms.iter() {
    let mut_s = &term.borrow();
    if _name == mut_s.name {
      return Some(term.clone());
    }
  }
  return None;
}

fn findlist(glo: &Glossary, name: &str) -> Option<Rc<RefCell<List>>> {
  let mut _name = String::with_capacity(name.len());
  _name = name.to_lowercase().replace("_", " ");
  for i in 0..glo.len {
    if &_name == &glo.lists[i as usize].borrow().name {
      return Some(glo.lists[i as usize].clone());
    }
  }
  return None;
}

pub fn scan(content: &str) {
  println!("Scanner  | ");
  scan_glossary(&content);
}

fn main() {
  let all_terms = &mut Lexicon::new();
  let all_lists = &mut Glossary::new();
  let all_logs = &mut Journal::new();
  parse(all_lists, all_terms, all_logs);
  link(all_lists, all_terms, all_logs).unwrap();
  build(all_terms, all_logs).unwrap();
  check(all_terms, all_lists, all_logs).unwrap();
}
