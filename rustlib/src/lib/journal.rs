use std::cell::RefCell;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::rc::Rc;

use super::error::SkiffError;
use super::glossary::{findlist, Glossary};
use super::helpers;
use super::lexicon::{Lexicon, Term};
use super::scanner::Scanner;
use super::vars::{KEY_BUF_LEN, LOG_BUF_LEN};

#[derive(Clone, Debug)]
pub struct Log {
  pub date: String,
  pub rune: String,
  pub code: i32,
  pub host: String,
  pub pict: i32,
  pub name: String,
  pub term: Option<Rc<RefCell<Term>>>,
}

#[derive(Debug)]
pub struct Journal {
  pub len: i32,
  pub logs: Vec<Rc<RefCell<Log>>>,
}

impl Journal {
  pub fn new() -> Journal {
    Journal {
      len: 0,
      logs: vec![Rc::new(RefCell::new(Log::new()))],
    }
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

pub fn link(lex: &mut Lexicon, glo: &mut Glossary) {
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
}

pub fn parse(path: String, journal: &mut Journal) -> Result<(), SkiffError> {
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

    if len < 14 || scanner.source[0] == ';' {
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
    log.borrow_mut().rune = scanner.source[6].to_string();
    /* Code */
    log.borrow_mut().code = helpers::sint(&scanner.source[7..], 3) as i32;
    /* Term */
    // extract only `host` type.
    let mut split_line = line.split_whitespace();
    log.borrow_mut().host = split_line.nth(2).unwrap().to_string();
    let _host = &log.borrow_mut().host.chars().collect::<Vec<char>>();

    // TODO: find better way to split without consume nth.
    /* Pict */
    let picture_id = split_line.next().unwrap();
    if picture_id != "-" {
      log.borrow_mut().pict = picture_id.parse().unwrap();
    }
    /* Name */
    if let Some(code_col) = split_line.next() {
      log.borrow_mut().name = code_col.to_string().replace('_', " ");
    }
    if !helpers::sans(_host) == 0 {
      println!("Warning: {} is not alphanum", log.borrow().host);
    }
    journal.len += 1;
    line.clear();
  }

  Ok(())
}

pub fn check(jou: &Journal) {
  for log in jou.logs.iter() {
    if log.borrow().code < 1 {
      println!("Warning: Empty code {}\n", log.borrow().date);
    }
  }
}
