use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::LineWriter;
use std::io::Write;
use std::rc::Rc;

use super::helpers;
use super::journal::{Journal, Log};
use super::lexicon::Term;

pub fn print_term_details(
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

pub fn get_sorted_years_root_parent(terms: &Term) -> Vec<(String, String)> {
	let mut years: HashMap<String, String> = HashMap::new();
	for _term in terms.children.iter() {
		let y = _term.as_ref().unwrap().borrow().year.to_string();
		if years.contains_key(&y) == false && &_term.as_ref().unwrap().borrow().name != "home" {
			years.insert(y.clone(), y.clone());
		}
	}

	let mut sorted_years: Vec<_> = years.into_iter().collect();
	sorted_years.sort_by(|x, y| y.0.cmp(&x.0));
	return sorted_years;
}

// TODO: avoid using same key and value .
pub fn get_sorted_years(terms: &Term) -> Vec<(String, String)> {
	let mut years: HashMap<String, String> = HashMap::new();
	for _term in terms.parent.as_ref().unwrap().borrow().children.iter() {
		let y = _term.as_ref().unwrap().borrow().year.to_string();
		if years.contains_key(&y) == false && &_term.as_ref().unwrap().borrow().r#type != "unindex" {
			years.insert(y.clone(), y.clone());
		}
	}

	let mut sorted_years: Vec<_> = years.into_iter().collect();
	sorted_years.sort_by(|x, y| y.0.cmp(&x.0));
	return sorted_years;
}

pub fn finddiary(jou: &Journal, term: &Term) -> Option<Rc<RefCell<Log>>> {
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
