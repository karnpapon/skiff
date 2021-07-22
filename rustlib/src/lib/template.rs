use std::cell::RefCell;
use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::LineWriter;
use std::io::{BufReader, Write};
use std::rc::Rc;

use super::error::SkiffError;
use super::helpers;
use super::lexicon::{findterm, Lexicon, Term};
use super::scanner::Scanner;
use super::vars::STR_BUF_LEN;

pub fn ftemplate(
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
	} else if cmd == "vimeo" {
		file.write_fmt(format_args!("<iframe width='600' height='380' src='https://player.vimeo.com/video/{}'  style='max-width:700px' frameborder='0' allow='autoplay; encrypted-media' allowfullscreen></iframe>", target)).unwrap();
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

		// to build special section
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
				return Err(SkiffError::ParseError(format!(
					"Unknown link {:?}",
					&target
				)));
			}
		}
	}

	Ok(())
}

pub fn link_next_prev(lex_term: &mut Vec<Rc<RefCell<Term>>>) -> Result<(), Box<dyn Error>> {
	let mut idx_next = 0;
	let mut idx_prev = 0;
	lex_term.sort_by(|a, b| {
		b.borrow()
			.year
			.parse::<i32>()
			.unwrap()
			.cmp(&a.borrow().year.parse::<i32>().unwrap())
	});

	// link to `next` and `prev` term
	// but skip only `home`(purposely for building suggested works).
	for (i, term) in lex_term.iter().enumerate() {
		if term.borrow().name != "home" {
			if i + 1 > lex_term.len() - 1 {
				idx_next = 0;
			} else {
				idx_next = i + 1;
			}

			if i - 1 == 0 {
				idx_prev = lex_term.len() - 1;
			} else {
				idx_prev = i - 1;
			}

			// if term.borrow().children_len > 0 {
			//   let mut term_children = Vec::new();
			//   for _term in &term.borrow().children {
			//     term_children.push(*(_term.as_ref().unwrap()).clone());
			//   }
			//   link_next_prev(&mut term_children).unwrap();
			// } else {
			term.borrow_mut().prev = Some(Box::new(lex_term[idx_prev].clone()));
			term.borrow_mut().next = Some(Box::new(lex_term[idx_next].clone()));
			// }
		}
	}

	Ok(())
}
