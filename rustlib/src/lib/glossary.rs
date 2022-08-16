use std::cell::RefCell;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::rc::Rc;

use super::error::SkiffError;
use super::helpers;
use super::lexicon::{findterm, Lexicon};
use super::scanner::Scanner;
use super::template::ftemplate;
use super::vars::{KEY_BUF_LEN, LIST_ITEMS};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[allow(dead_code)]
pub struct List {
  pub name: String,
  pub keys: Vec<String>,
  pub vals: Vec<String>,
  pub len: i32,
  pub routes: i32,
}

#[derive(Debug)]
pub struct Glossary {
  pub len: i32,
  pub lists: Vec<Rc<RefCell<List>>>,
}

impl Glossary {
  pub fn new() -> Glossary {
    Glossary {
      len: 0,
      lists: vec![Rc::new(RefCell::new(List::new()))],
    }
  }
}

impl List {
  pub fn new() -> List {
    List {
      name: String::with_capacity(KEY_BUF_LEN),
      keys: vec![],
      vals: vec![],
      len: 0,
      routes: 0,
    }
  }
}

fn scan_glossary(content: &str) {
  let mut scanner = Scanner::new(&content);
}

pub fn link(lex: &mut Lexicon, glo: &mut Glossary) {
  for i in 0..lex.len {
    let lex_term = &lex.terms[i as usize];
    let lext_t_clone = lex_term.borrow().body.clone();
    for (_idx, j) in lext_t_clone.iter().enumerate() {
      ftemplate(None, lex, &lex_term.clone().borrow(), j).unwrap();
    }
    let host_name = lex_term.borrow().host.to_string();
    let mut ch_len = 0;
    if let Some(len) = &lex_term.borrow_mut().parent.as_ref() {
      ch_len = len.borrow().children_len as usize;
    }

    match findterm(lex, &host_name) {
      Some(t) => {
        lex_term.borrow_mut().parent = Some(Box::new(t));
        let parent_term = lex_term.borrow().parent.as_ref().unwrap().clone();
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
}

pub fn findlist(glo: &Glossary, name: &str) -> Option<Rc<RefCell<List>>> {
  let mut _name = String::with_capacity(name.len());
  _name = name.to_lowercase().replace('_', " ");
  for i in 0..glo.len {
    if _name == glo.lists[i as usize].borrow().name {
      return Some(glo.lists[i as usize].clone());
    }
  }
  return None;
}

pub fn parse(path: String, glossary: &mut Glossary) -> Result<(), SkiffError> {
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
    if scanner.source.is_empty() {
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
    if len < 4 || scanner.source[0] == ';' {
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

pub fn check(glo: &Glossary) {
  for list in glo.lists.iter() {
    if list.borrow().routes < 1 {
      println!(
        "Warning: Unused (glossary)list \"{}\"\n",
        list.borrow().name
      );
    }
  }
}
