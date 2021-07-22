use super::error::SkiffError;
use super::helpers;
use super::journal::{Journal, Log};
use super::lexicon::{Lexicon, Term};
use super::template::ftemplate;
use super::utils::{finddiary, get_sorted_years, get_sorted_years_root_parent};
use super::vars::DOMAIN;

use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::LineWriter;
use std::io::Write;
use std::path::Path;

pub fn build(lex: &Lexicon, jou: &Journal) -> Result<(), SkiffError> {
	let mut file;
	let mut file_writer;

	println!("Building | ");
	for i in 0..lex.len {
		let lex_term = lex.terms[i as usize].as_ref().borrow_mut().clone();
		if &lex_term.r#type != "category" {
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
	file.write(b"<link rel='stylesheet' type='text/css' href='../styles/main.css'>")?;
	file.write(b"<link rel='shortcut icon' type='image/png' href='../media/services/icon.png'>")?;
	file.write_fmt(format_args!("<title>{} — karnpapon</title>", term.name))?;
	file.write(b"</head>")?;
	file.write(b"<body>")?;
	file.write(b"<main class=\"container-ctrl scroll-wrapper\">")?;

	if term.name != "home" {
		build_section_header(file, term).unwrap();
		build_section_details(file, term, jou, lex).unwrap();
	} else {
		build_home(file, term).unwrap();
	}

	/* templated pages */
	match term.r#type.as_ref() {
		// "portal" => build_portal(file, jou, term).unwrap(),
		// "album" => build_album(file, jou, term).unwrap(),
		"index" => build_index(file, lex, term).unwrap(),
		_ => {}
	};
	/* special pages */
	// match term.name.as_ref() {
	// "now" => build_special_now(file, lex, jou).unwrap(),
	// "home" => build_special_home(file, jou).unwrap(),
	// "calendar" => build_special_calendar(file, jou).unwrap(),
	// "tracker" => build_special_tracker(file, jou).unwrap(),
	// "journal" => build_special_journal(file, jou).unwrap(),
	// "index" => build_special_index(file, lex).unwrap(),
	// _ => {}
	// };
	build_list(file, term).unwrap();
	build_incoming(file, term).unwrap();
	file.write(b"</main>")?;
	file.write(b"<footer>")?;
	build_footer(file, term, term.name.clone()).unwrap();
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

fn build_home(file: &mut LineWriter<File>, terms: &Term) -> Result<(), Box<dyn Error>> {
	let mut sorted_years: Vec<_> = get_sorted_years(&terms);
	file.write(b"<pxy><div><ul>")?;
	file.write(b"<li class=\"root\"><h2>unmonetizable stuffs, but joy. &nbsp;&nbsp;</h2></li>")?;
	for year in sorted_years.iter() {
		build_home_children_item(file, terms, year, false)?;
	}
	file.write(b"</ul></div></pxy>")?;
	Ok(())
}

fn build_home_children_item(
	file: &mut LineWriter<File>,
	terms: &Term,
	year: &(String, String),
	recursive: bool,
) -> Result<(), Box<dyn Error>> {
	let mut prev_year = String::new();
	file.write(b"<li>")?;
	if recursive == false {
		file.write_fmt(format_args!("<strong>{}</strong>", year.0))?;
	}
	file.write(b"<ul>")?;
	for term in terms.children.iter() {
		let _term = term.as_ref().unwrap().borrow();

		if year.0.parse::<i32>() == _term.year.parse::<i32>() {
			let name = term.as_ref().unwrap().borrow().name.clone();
			let bref = term.as_ref().unwrap().borrow().bref.clone();

			// TODO: more intuitive code, dont lazy.
			if _term.r#type == "category" {
				file.write_fmt(format_args!(
					"<li><span>{}</span><fdt>{}</fdt></li>",
					name, bref
				))?;
			} else {
				file.write_fmt(format_args!(
					"<li><a href='/site/{}.html'>{}</a> — <fdt>{}</fdt></li>",
					term.as_ref().unwrap().borrow().filename.clone(),
					name,
					bref
				))?;
			}
			if _term.children_len > 0 && _term.name.clone() != "home" {
				build_home_children_item(file, &_term, year, true)?
			}
		}
	}
	file.write(b"</ul>")?;
	file.write(b"</li>")?;

	Ok(())
}

fn build_section_header(file: &mut LineWriter<File>, term: &Term) -> Result<(), Box<dyn Error>> {
	file.write(b"<section class=\"s0\">")?;
	file.write(b"<div>")?;
	file.write(b"<h1>")?;
	file.write_fmt(format_args!(
		"<a class=\"link-default\" href=\"/index.html\"><span>..</span></a>/{}",
		term.name
	))?;
	file.write(b"</h1>")?;

	file.write(b"<px>")?;
	file.write_fmt(format_args!("<p>{}</p>", term.bref))?;
	file.write(b"</px>")?;
	file.write_fmt(format_args!("<p>({})</p>", term.year))?;
	file.write(b"</div>")?;
	file.write(b"<div><a href=\"/index.html\"><i class=\"icon-arr-back\">~</i></a></div>")?;
	file.write(b"</section>")?;
	Ok(())
}

fn build_section_details(
	file: &mut LineWriter<File>,
	term: &Term,
	jou: &Journal,
	lex: &Lexicon,
) -> Result<(), Box<dyn Error>> {
	file.write(b"<section class=\"s1\">")?;
	file.write(b"<div>")?;
	// <!-- <Left-Col/> -->
	file.write(b"<div>")?;
	build_banner(file, jou, term, 1).unwrap();
	file.write(b"<px>")?;
	build_body(file, lex, term).unwrap();
	file.write(b"</px>")?;
	build_include(file, term).unwrap();
	file.write(b"</div>")?;

	// <!-- <Right-Col/> -->
	file.write(b"<div>")?;
	file.write(b"<div class=\"position-sticky\">")?;
	build_links(file, term).unwrap();
	file.write(b"<px>")?;
	if term.stack.len() > 0 {
		for _stack in term.stack.iter() {
			file.write_fmt(format_args!("<p>{}</p>", _stack))?;
		}
	}
	file.write(b"</px>")?;
	file.write(b"<px>")?;
	if term.tag.len() > 0 {
		for _term in term.tag.iter() {
			file.write_fmt(format_args!("<tag>{}</tag>", _term))?;
		}
	}
	file.write(b"</px>")?;
	file.write(b"</div>")?;
	file.write(b"</div>")?;

	file.write(b"</div>")?;
	file.write(b"<div class=\"scroll-spacing\"></div>")?;
	if term.name != "home" {
		build_section_suggest(file, term).unwrap();
	}
	file.write(b"</section>")?;
	Ok(())
}

fn build_section_suggest(file: &mut LineWriter<File>, term: &Term) -> Result<(), Box<dyn Error>> {
	let term_next = term.next.as_ref().unwrap().borrow();
	let term_prev = term.prev.as_ref().unwrap().borrow();
	file.write(b"<div class=\"s2\">")?;
	file.write(b"<lc>")?;
	file.write_fmt(format_args!("<fb class=\"pad2\">{}</fb>", "Projects"))?;
	file.write(b"</lc>")?;
	file.write(b"<rc class=\"flex-col\">")?;

	file.write_fmt(format_args!(
		"<div class=\"box\"><a href={}.html><fm>{}</fm><p>{}</p></a></div>",
		term_next.filename, term_next.name, term_next.bref
	))?;
	file.write_fmt(format_args!(
		"<div class=\"box\"><a href={}.html><fm>{}</fm><p>{}</p></a></div>",
		term_prev.filename, term_prev.name, term_prev.bref
	))?;

	file.write(b"</rc>")?;
	file.write(b"</div>")?;
	Ok(())
}

fn build_footer(
	file: &mut LineWriter<File>,
	terms: &Term,
	term_name: String,
) -> Result<(), Box<dyn Error>> {
	// recurive to root parent ("home"), then render all children.
	if terms.name == "home" {
		let mut sorted_years: Vec<_> = get_sorted_years_root_parent(&terms);

		file.write(b"<div class=\"footer\">")?;
		file.write(b"<div>")?;
		file.write(b"<lc><div>")?;
		file.write_fmt(format_args!(
			"<input type=\"checkbox\"/><label>{}</label>",
			"index"
		))?;
		file.write(b"<div class=\"works-list\">")?;
		for year in sorted_years.iter() {
			file.write(b"<div class=\"works\">")?;
			file.write_fmt(format_args!("<h2>{}</h2>", year.0))?;
			for term in terms.children.iter() {
				let name = term.as_ref().unwrap().borrow().name.clone();
				if year.0.parse::<i32>() == term.as_ref().unwrap().borrow().year.parse::<i32>() {
					if name == term_name {
						file.write_fmt(format_args!(
							"<a href='{}.html'><p class=\"work-actived\">{}</p></a>",
							term.as_ref().unwrap().borrow().filename.clone(),
							term_name
						))?;

						// if has children (eg. the-blackcodes)
						if term.as_ref().unwrap().borrow().children_len > 0 {
							for x in term.as_ref().unwrap().borrow().children.iter() {
								file.write_fmt(format_args!(
									"<a href='{}.html'><p>{}</p></a>",
									x.as_ref().unwrap().borrow().filename,
									x.as_ref().unwrap().borrow().name
								))?;
							}
						}
					} else {
						// TODO: REFACTOR THESE UGLYYY CODES.
						// if type == category render only text (not building page.)
						if term.as_ref().unwrap().borrow().r#type == "category" {
							for x in term.as_ref().unwrap().borrow().children.iter() {
								if term_name == x.as_ref().unwrap().borrow().name {
									file.write_fmt(format_args!(
										"<a href='{}.html'><p class=\"work-actived\">{}</p></a>",
										term_name, term_name
									))?;
								} else {
									file.write_fmt(format_args!(
										"<a href='{}.html'><p>{}</p></a>",
										x.as_ref().unwrap().borrow().filename,
										x.as_ref().unwrap().borrow().name
									))?;
								}
							}
						} else if term.as_ref().unwrap().borrow().children_len > 0 {
							// if that term has children, then keep looping and render those childs (including term).
							file.write_fmt(format_args!(
								"<a href='{}.html'><p>{}</p></a>",
								term.as_ref().unwrap().borrow().filename,
								name
							))?;

							for x in term.as_ref().unwrap().borrow().children.iter() {
								if term_name == x.as_ref().unwrap().borrow().name {
									file.write_fmt(format_args!(
										"<a href='{}.html'><p class=\"work-actived\">{}</p></a>",
										term_name, term_name
									))?;
								} else {
									file.write_fmt(format_args!(
										"<a href='{}.html'><p>{}</p></a>",
										x.as_ref().unwrap().borrow().filename,
										x.as_ref().unwrap().borrow().name
									))?;
								}
							}
						} else {
							file.write_fmt(format_args!(
								"<a href='{}.html'><p>{}</p></a>",
								term.as_ref().unwrap().borrow().filename,
								name
							))?;
						}
					}
				}
			}
			file.write(b"</div>")?;
		}
		file.write(b"</div>")?;
		file.write(b"</div></lc>")?;
		file.write(b"<rc>")?;
		file.write_fmt(format_args!("<div>{}</div>", "karnpapon - BY-NC-SA 4.0"))?;
		file.write(b"<ic>")?;
		file.write(b"<a href='https://creativecommons.org/licenses/by-nc-sa/4.0'><img src='../media/icon/cc.svg' width='30'/></a>")?;
		file.write(b"<a href='https://github.com/karnpapon'><img src='../media/icon/github.png' alt='github' width='30'/></a>")?;
		file.write(b"</ic>")?;
		file.write(b"</rc>")?;
		file.write(b"</div>")?;
		file.write(b"</div>")?;
		return Ok(());
	}
	build_footer(file, &terms.parent.as_ref().unwrap().borrow(), term_name).unwrap();
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
	// file.write_fmt(format_args!("<h2>{}</h2>", &term.bref))?;
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
			"<url><a href='{}' target='_blank'>\u{1F855} {}</a></url>",
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
