use std::cell::RefCell;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::rc::Rc;

use super::error::SkiffError;
use super::glossary::List;
use super::helpers;
use super::journal::Journal;
use super::scanner::Scanner;
use super::vars::{KEY_BUF_LEN, STR_BUF_LEN};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[allow(dead_code)]
pub struct Term {
	pub name: String,
	pub host: String,
	pub bref: String,
	pub year: String,
	pub r#type: String,
	pub tag: Vec<String>,
	pub stack: Vec<String>,
	pub body: Vec<String>,
	pub body_len: usize,
	pub link: List,
	pub list: Vec<String>,
	pub list_len: i32,
	pub filename: String,
	pub date_from: RefCell<String>,
	pub date_last: RefCell<String>,
	pub parent: Option<Box<Rc<RefCell<Term>>>>,
	// parent: Option<Box<Weak<RefCell<Term>>>>,
	pub next: Option<Box<Rc<RefCell<Term>>>>,
	pub prev: Option<Box<Rc<RefCell<Term>>>>,
	pub children: Vec<Option<Box<Rc<RefCell<Term>>>>>,
	pub children_len: i32,
	pub docs: Vec<Option<Rc<RefCell<List>>>>,
	pub docs_len: i32,
	pub incoming: Vec<Box<Rc<RefCell<Term>>>>,
	pub incoming_len: i32,
	pub outgoing_len: i32,
}

#[derive(Clone, Debug)]
pub struct Lexicon {
	pub len: i32,
	pub terms: Vec<Rc<RefCell<Term>>>,
}

impl Lexicon {
	pub fn new() -> Lexicon {
		Lexicon {
			len: 0,
			terms: vec![Rc::new(RefCell::new(Term::new()))],
		}
	}
}

impl Term {
	fn new() -> Term {
		Term {
			name: String::with_capacity(KEY_BUF_LEN),
			host: String::with_capacity(KEY_BUF_LEN),
			bref: String::with_capacity(STR_BUF_LEN),
			year: String::with_capacity(STR_BUF_LEN),
			tag: vec![],
			stack: vec![],
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
			next: None,
			prev: None,
			children: vec![],
			children_len: 0,
			docs: vec![],
			docs_len: 0,
			incoming: vec![],
			incoming_len: 0,
			outgoing_len: 0,
		}
	}
}

pub fn findterm(lex: &Lexicon, name: &str) -> Option<Rc<RefCell<Term>>> {
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

pub fn link(jou: &mut Journal, lex: &mut Lexicon) {
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
}

// TODO: make it less C-ish style.
pub fn parse(path: String, lexicon: &mut Lexicon) -> Result<(), SkiffError> {
	let f = File::open(path).expect("lexicon parsing: file not found");
	let mut f_reader = BufReader::new(f);
	let mut key_len: usize;
	let mut val_len: usize;
	let mut len: usize;
	// let count = 0;
	let mut catch_body = false;
	let mut catch_link = false;
	let mut catch_list = false;
	let mut catch_tag = false;
	let mut catch_stack = false;
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

		// if len < 3 = skip eg. case of newline '\n';
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

			if helpers::spos(&_scanner, "YEAR : ") >= 0 {
				t.borrow_mut().year = helpers::sstr(&scanner.source, 9, len - 9);
			}
			catch_body = helpers::spos(&scanner.source, "BODY") >= 0;
			catch_link = helpers::spos(&scanner.source, "LINK") >= 0;
			catch_list = helpers::spos(&scanner.source, "LIST") >= 0;
			catch_tag = helpers::spos(&scanner.source, "TAGS") >= 0;
			catch_stack = helpers::spos(&_scanner, "STACK") >= 0;
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

			// Tag
			if catch_tag {
				let tag_len = t.borrow().tag.len() as usize;
				t.borrow_mut()
					.tag
					.insert(tag_len, helpers::sstr(&scanner.source, 4, len - 4));
			}

			// stack
			if catch_stack {
				let stack_len = t.borrow().stack.len() as usize;
				t.borrow_mut()
					.stack
					.insert(stack_len, helpers::sstr(&scanner.source, 4, len - 4));
			}
		}
		line.clear();
	}
	Ok(())
}

pub fn check(lex: &Lexicon) {
	let mut sends = 0;
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
}
