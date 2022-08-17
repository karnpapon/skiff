use super::error::SkiffError;
use super::helpers;
use super::journal::{Journal, Log};
use super::lexicon::{Lexicon, Term};
use super::template::ftemplate;
use super::utils::{finddiary, get_sorted_years, get_sorted_years_root_parent};
use super::vars::{DOMAIN, THUMBNAIL_IMG};

use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::LineWriter;
use std::io::Write;
use std::path::Path;
use comrak::{markdown_to_html, ComrakOptions, ComrakExtensionOptions, ComrakRenderOptions};

pub fn build(lex: &Lexicon, jou: &Journal) -> Result<(), SkiffError> {
  let mut file;
  let mut file_writer;

  println!("Building | ");
  for i in 0..lex.len {
    let lex_term = lex.terms[i as usize].as_ref().borrow_mut().clone();
    if &lex_term.r#type == "category" {
      continue;
    }
    let mut filepath: String = format!("{}/{}.{}", "../site/", lex_term.filename, "html");
    if &lex_term.name == "home" {
      filepath = format!("{}/{}.{}", "../", "index", "html");
    }

    let path = Path::new(&filepath);
    let display = path.display();
    file_writer = match File::create(path) {
      Err(why) => panic!("couldn't create {}: {}", display, why),
      Ok(f) => f,
    };
    file = LineWriter::new(file_writer);
    build_page(&mut file, lex, &lex_term, jou).unwrap();
  }
  Ok(())
}

fn build_page(
  file: &mut LineWriter<File>,
  lex: &Lexicon,
  term: &Term,
  jou: &Journal,
) -> Result<(), std::io::Error> {
  file.write_all(b"<!DOCTYPE html>")?;
  file.write_all(b"<html lang='en'>")?;
  file.write_all(b"<head>")?;
  file.write_all(b"<meta charset='utf-8'>")?;
  file.write_all(b"<meta name='viewport' content='width=device-width, initial-scale=1'>")?;
  file.write_fmt(format_args!(
    "<meta property=\"og:title\" content='Karnpapon Boonput'/>",
  ))?;
  file.write_fmt(format_args!(
    "<meta property=\"og:type\" content='garden'/>"
  ))?;
  file.write_fmt(format_args!(
    "<meta property=\"og:description\" content='{}'/>",
    term.bref
  ))?;
  file.write_fmt(format_args!(
    "<meta property=\"og:url\" content='{}' />",
    DOMAIN
  ))?;
  file.write_fmt(format_args!(
    "<meta property=\"og:image\" content='{}' />",
    THUMBNAIL_IMG
  ))?;
  file.write_all(b"<link rel='stylesheet' type='text/css' href='../styles/main.css'>")?;
  file
    .write_all(b"<link rel='shortcut icon' type='image/png' href='../media/services/icon.png'>")?;
  file.write_fmt(format_args!("<title>{} — karnpapon</title>", term.name))?;
  file.write_all(b"</head>")?;
  file.write_all(b"<body>")?;
  file.write_all(b"<main class=\"container-ctrl scroll-wrapper\">")?;

  if term.name != "home" {
    build_section_header(file, term).unwrap();
    build_section_details(file, term, jou, lex).unwrap();
  } else {
    build_home(file, term).unwrap();
  }

  /* templated pages */
  if term.r#type == "index" {
    build_index(file, lex, term).unwrap()
  }
  // match term.r#type.as_ref() {
  // 	// "portal" => build_portal(file, jou, term).unwrap(),
  // 	// "album" => build_album(file, jou, term).unwrap(),
  // 	"index" => build_index(file, lex, term).unwrap(),
  // 	_ => {}
  // };

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
  file.write_all(b"</main>")?;
  file.write_all(b"<footer>")?;
  build_footer(file, term, term.name.clone()).unwrap();
  file.write_all(b"</footer>")?;
  file.write_all(b"</body></html>")?;
  Ok(())
}

fn build_nav(file: &mut LineWriter<File>, term: &Term) -> Result<(), Box<dyn Error>> {
  file.write_all(b"<nav>")?;
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
          // let _t_parent_parent_clone = t_parent_parent.clone();
          build_nav_part(file, &t_parent.borrow(), &term)?;
        }

        if t_parent_name != term.name.to_string() {
          build_nav_part(file, &term.clone(), &term.clone())?;
        }
      }
    }
    None => println!("parent None"),
  }
  file.write_all(b"</nav>")?;
  Ok(())
}

fn build_nav_part(
  file: &mut LineWriter<File>,
  term: &Term,
  target: &Term,
) -> Result<(), Box<dyn Error>> {
  file.write_all(b"<ul>")?;
  for i in 0..term.children_len {
    let term_children = term.children[i as usize].as_ref().unwrap().borrow().clone();
    if term_children.name == term.name {
      continue; /* Paradox */
    }

    let filename = term_children.filename.to_string();
    let name = term_children.name.to_string();

    if term_children.name == target.name {
      // add symbol "/" at the end as a current actived (eg. `about/` ).
      file.write_fmt(format_args!(
        "<li><a href='/site/{}.html'>{}/</a></li>",
        filename, name
      ))?;
    } else {
      file.write_fmt(format_args!(
        "<li><a href='/site/{}.html'>{}</a></li>",
        filename, name
      ))?;
    }
  }
  file.write_all(b"</ul>")?;
  Ok(())
}

fn build_home(file: &mut LineWriter<File>, terms: &Term) -> Result<(), Box<dyn Error>> {
  let sorted_years: Vec<_> = get_sorted_years(&terms);
  file.write_all(b"<pxy><div class=\"home_menu\"><ul>")?;
  file.write_all(b"<li class=\"root\"><h2>~ &nbsp;&nbsp;</h2></li>")?;
  for year in sorted_years.iter() {
    file.write_all(b"<li>")?;
    build_home_children_item(file, terms, year, false)?;
    file.write_all(b"</li>")?;
  }
  file.write_all(b"</ul></div></pxy>")?;
  Ok(())
}

fn build_home_children_item(
  file: &mut LineWriter<File>,
  terms: &Term,
  year: &(String, String),
  recursive: bool,
) -> Result<(), Box<dyn Error>> {
  if !recursive {
    file.write_fmt(format_args!("<strong>{}</strong>", year.0))?;
  }
  file.write_all(b"<ul>")?;
  for term in terms.children.iter() {
    let _term = term.as_ref().unwrap().borrow();
    if year.0.parse::<i32>() == _term.year.parse::<i32>() {
      let name = term.as_ref().unwrap().borrow().name.clone();
      let bref = term.as_ref().unwrap().borrow().bref.clone();
      if _term.children_len > 0 && _term.name.clone() != "home" {
        file.write_fmt(format_args!(
          "<li class='has-child'><a href='/site/{}.html'>{}</a> — <fdt>{}</fdt></li>",
          term.as_ref().unwrap().borrow().filename.clone(),
          name,
          bref
        ))?;
        build_home_children_item(file, &_term, year, true)?
      } else {
        file.write_fmt(format_args!(
          "<li><a href='/site/{}.html'>{}</a> — <fdt>{}</fdt></li>",
          term.as_ref().unwrap().borrow().filename.clone(),
          name,
          bref
        ))?;
      }
    }
  }
  file.write_all(b"</ul>")?;

  Ok(())
}

fn build_section_header(file: &mut LineWriter<File>, term: &Term) -> Result<(), Box<dyn Error>> {
  file.write_all(b"<section class=\"s0\">")?;
  file.write_all(b"<div>")?;
  file.write_all(b"<h1>")?;
  let home_path = format!(
    "<a class=\"link-default\" href=\"/index.html\">~</a>/{}/",
    term.year
  );
  let mut parent_paths = String::from("");
  // println!(
  // 	"term.name = {} & term.home_depth = {}",
  // 	term.name, term.home_dept
  // );

  if term.home_dept > 0 {
    for _ in 0..term.home_dept {
      let parent_string = format!(
        "<a class=\"link-default\" href=\"/site/{}.html\"><span>..</span></a>/",
        term.parent.as_ref().unwrap().borrow().filename
      );
      parent_paths.push_str(parent_string.as_str());
    }
    let paths = format!("{}{}", home_path, parent_paths);
    file.write_fmt(format_args!(
      "{}<a href='/site/{}.html'>{}</a>",
      paths, term.filename, term.name
    ))?;
  } else {
    file.write_fmt(format_args!(
      "{}<a href='/site/{}.html'>{}</a>",
      home_path, term.filename, term.name
    ))?;
  }
  file.write_all(b"</h1>")?;

  file.write_all(b"<px>")?;
  if !term.bref.is_empty() {
    file.write_fmt(format_args!("<p>{}</p>", term.bref))?;
  }
  file.write_all(b"</px>")?;
  // if term.year.is_empty() == false {
  // 	file.write_fmt(format_args!("<p>({})</p>", term.year))?;
  // }
  file.write_all(b"</div>")?;
  file.write_all(b"</section>")?;
  Ok(())
}

fn build_section_details(
  file: &mut LineWriter<File>,
  term: &Term,
  jou: &Journal,
  lex: &Lexicon,
) -> Result<(), Box<dyn Error>> {
  file.write_all(b"<section class=\"s1\">")?;
  file.write_all(b"<div>")?;
  // <!-- <Left-Col/> -->
  file.write_all(b"<div>")?;
  build_banner(file, jou, term, 1).unwrap();
  file.write_all(b"<px>")?;
  build_body(file, lex, term).unwrap();
  file.write_all(b"</px>")?;
  build_include(file, term).unwrap();
  file.write_all(b"</div>")?;

  // <!-- <Right-Col/> -->
  file.write_all(b"<div>")?;
  file.write_all(b"<div class=\"position-sticky\">")?;
  build_links(file, term).unwrap();

  file.write_fmt(format_args!("<p class=\"info-year\">{}</p>", term.year))?;
  file.write_all(b"<px>")?;
  if !term.stack.is_empty() {
    for _stack in term.stack.iter() {
      file.write_fmt(format_args!("<p>{}</p>", _stack.parent))?;
      file.write_all(b"<ul>")?;
      if !_stack.children.is_empty() {
        for stack_child in _stack.children.iter() {
          file.write_fmt(format_args!("<li>{}</li>", stack_child))?;
        }
      }
      file.write_all(b"</ul>")?;
    }
  }
  file.write_all(b"</px>")?;
  file.write_all(b"<px>")?;
  if !term.tag.is_empty() {
    for _term in term.tag.iter() {
      file.write_fmt(format_args!("<tag>{}</tag>", _term))?;
    }
  }
  file.write_all(b"</px>")?;
  file.write_all(b"<a id='go-home' href='/index.html'> ~ </a>")?;
  file.write_all(b"</div>")?;
  file.write_all(b"</div>")?;

  file.write_all(b"</div>")?;
  file.write_all(b"<div class=\"scroll-spacing\"></div>")?;
  if term.name != "home" {
    build_section_suggest(file, term).unwrap();
  }
  file.write_all(b"</section>")?;
  Ok(())
}

fn build_section_suggest(file: &mut LineWriter<File>, term: &Term) -> Result<(), Box<dyn Error>> {
  let term_next = term.next.as_ref().unwrap().borrow();
  let term_prev = term.prev.as_ref().unwrap().borrow();
  file.write_all(b"<div class=\"s2\">")?;
  file.write_all(b"<lc>")?;
  file.write_fmt(format_args!("<fb class=\"pad2\">{}</fb>", "Projects"))?;
  file.write_all(b"</lc>")?;
  file.write_all(b"<rc class=\"flex-col\">")?;

  file.write_fmt(format_args!(
    "<div class=\"box\"><a href={}.html><fm>{}</fm><p>{}</p></a></div>",
    term_next.filename, term_next.name, term_next.bref
  ))?;
  file.write_fmt(format_args!(
    "<div class=\"box\"><a href={}.html><fm>{}</fm><p>{}</p></a></div>",
    term_prev.filename, term_prev.name, term_prev.bref
  ))?;

  file.write_all(b"</rc>")?;
  file.write_all(b"</div>")?;
  Ok(())
}

fn build_current_actived_footer_item(
  file: &mut LineWriter<File>,
  term: String,
  term_name: String,
) -> Result<(), Box<dyn Error>> {
  file.write_fmt(format_args!(
    "<a href='/site/{}.html'><p class=\"work-actived\">{}</p></a>",
    term, term_name
  ))?;
  Ok(())
}

fn build_footer_item(
  file: &mut LineWriter<File>,
  term: String,
  term_name: String,
) -> Result<(), Box<dyn Error>> {
  file.write_fmt(format_args!(
    "<a href='/site/{}.html'><p>{}</p></a>",
    term, term_name
  ))?;
  Ok(())
}

fn is_actived_term(a: &str, b: &str) -> bool {
  a == b
}

fn build_footer_terms(
  file: &mut LineWriter<File>,
  term: String,
  current_term: String,
  name: String,
  file_name: String,
) -> Result<(), Box<dyn Error>> {
  if term == current_term {
    build_current_actived_footer_item(file, term.clone(), term)?;
  } else {
    build_footer_item(file, file_name, name)?;
  }
  Ok(())
}

fn build_actived_footer(
  file: &mut LineWriter<File>,
  term: &Term,
  _term: String,
  term_name: String,
) -> Result<(), Box<dyn Error>> {
  build_current_actived_footer_item(file, _term, term_name)?;
  if term.children_len > 0 {
    build_term_list(file, term)?;
  }
  Ok(())
}

fn build_unactived_footer(
  file: &mut LineWriter<File>,
  term: &Term,
  _term: String,
  term_name: String,
  name: String,
) -> Result<(), Box<dyn Error>> {
  build_footer_item(file, _term, name)?;
  if term.children_len <= 0 {
    return Ok(());
  }

  // if that term has children, then keep looping and render those childs (including term).
  for x in term.children.iter() {
    build_footer_terms(
      file,
      term_name.clone(),
      x.as_ref().unwrap().borrow().name.clone(),
      x.as_ref().unwrap().borrow().name.clone(),
      x.as_ref().unwrap().borrow().filename.clone(),
    )?;
  }
  Ok(())
}

fn build_term_list(file: &mut LineWriter<File>, term: &Term) -> Result<(), Box<dyn Error>> {
  // if has children (eg. the-blackcodes)
  for x in term.children.iter() {
    build_footer_item(
      file,
      x.as_ref().unwrap().borrow().filename.clone(),
      x.as_ref().unwrap().borrow().name.clone(),
    )?;
  }
  Ok(())
}

fn build_footer(
  file: &mut LineWriter<File>,
  terms: &Term,
  term_name: String,
) -> Result<(), Box<dyn Error>> {
  // recurive to root parent ("home"), then render all children.
  if terms.name == "home" {
    let sorted_years: Vec<_> = get_sorted_years_root_parent(terms);

    file.write_all(b"<div class=\"footer\">")?;
    file.write_all(b"<div>")?;
    file.write_all(b"<div class='footer-mobile-index'><a href='/index.html'>~</a></div>")?;
    file.write_all(b"<lc><div>")?;
    file.write_fmt(format_args!(
      "<input type=\"checkbox\"/><label>{}</label>",
      "INDEX"
    ))?;
    file.write_all(b"<div class=\"works-list\">")?;
    for year in sorted_years.iter() {
      file.write_all(b"<div class=\"works\">")?;
      file.write_fmt(format_args!("<h2>{}</h2>", year.0))?;
      for term in terms.children.iter() {
        let term_ref = &term.as_ref().unwrap().borrow();
        let term_ref_name = term_ref.name.clone();
        let term_ref_filename = term_ref.filename.clone();
        let term_name_clone = term_name.clone();
        if year.0.parse::<i32>() == term.as_ref().unwrap().borrow().year.parse::<i32>() {
          match is_actived_term(&term_ref_name, &term_name_clone) {
            true => {
              build_actived_footer(file, term_ref, term_ref_filename, term_name_clone)?;
            }
            false => {
              build_unactived_footer(
                file,
                term_ref,
                term_ref_filename,
                term_name_clone,
                term_ref_name,
              )?;
            }
          }
        }
      }
      file.write_all(b"</div>")?;
    }
    file.write_all(b"</div>")?;
    file.write_all(b"</div></lc>")?;
    file.write_all(b"<rc>")?;
    file.write_all(b"<p>this website was generated by <a href='/site/skiff.html'> skiff</a>")?;
    file.write_all(b"<div class='footer-extlink'>")?;
    file.write_fmt(format_args!(
      "<div class='footer-cc'>{}</div>",
      "BY-NC-SA 4.0"
    ))?;
    file.write_all(b"<a target='_blank' href='https://creativecommons.org/licenses/by-nc-sa/4.0'><img src='../media/icon/cc.svg'/></a>")?;
    file.write_all(b"<a target='_blank' href='https://github.com/karnpapon'><img src='../media/icon/github.png' alt='github'/></a>")?;
    file.write_all(b"</div>")?;
    file.write_all(b"</rc>")?;
    file.write_all(b"</div>")?;
    file.write_all(b"</div>")?;
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
  build_body_part(file, lex, term);
  Ok(())
}

fn build_body_part(file: &mut LineWriter<File>, lex: &Lexicon, term: &Term) {
  for term_body in term.body.iter() {
    ftemplate(Some(file), lex, term, term_body).unwrap();
  }
}

fn build_include(file: &mut LineWriter<File>, term: &Term) -> Result<(), Box<dyn Error>> {
  let filepath: String = format!("{}/{}.{}", "../inc", term.filename, "md");
  let path = Path::new(&filepath);
  let buff = match fs::read_to_string(path) {
    Ok(c) => { 
      let opts = ComrakOptions {
        extension: ComrakExtensionOptions {
            strikethrough: true,
            tagfilter: false,
            table: true,
            autolink: true,
            tasklist: true,
            superscript: true,
            footnotes: true,
            description_lists: true,
            ..ComrakExtensionOptions::default()
        },
        render: ComrakRenderOptions {
          unsafe_: true,
          ..ComrakRenderOptions::default()
        },
        ..ComrakOptions::default()
      };
      markdown_to_html(&c, &opts)
    } 
    _ => return Ok(()),
  };

  file.write_all(b"<div class='markdown-body'>")?;
  file
    .write_all(buff.as_bytes())
    .expect("error: cannot write /inc file");
  file.write_all(b"</div>")?;
  Ok(())
}

fn build_list(file: &mut LineWriter<File>, term: &Term) -> Result<(), Box<dyn Error>> {
  for (_i, doc) in term.docs.iter().enumerate() {
    if let Some(d) = doc {
      let _doc = d.as_ref().borrow().clone();
      file.write_fmt(format_args!("<h3>{}</h3>", _doc.name))?;
      file.write_all(b"<ul>")?;
      for j in 0.._doc.len {
        let _j = j as usize;
        if _doc.keys.is_empty() {
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
      file.write_all(b"</ul>")?;
    }
  }
  Ok(())
}

fn build_links(file: &mut LineWriter<File>, term: &Term) -> Result<(), Box<dyn Error>> {
  if term.link.len < 1 {
    return Ok(());
  }
  file.write_all(b"<ul style='margin: 0;'>")?;
  for i in 0..term.link.len {
    let k = term.link.keys[i as usize].clone();
    let icon = match term.link.keys[i as usize].to_lowercase().as_str() {
      "playground" => "🏓",
      "source" => "🔎",
      _ => "🌏",
    };

    file.write_fmt(format_args!(
      "<url><a href='{}' target='_blank'>{}\u{00A0} {}</a></url>",
      term.link.vals[i as usize], icon, k
    ))?
  }
  file.write_all(b"</ul>")?;
  Ok(())
}

fn build_incoming(file: &mut LineWriter<File>, term: &Term) -> Result<(), Box<dyn Error>> {
  if term.incoming_len < 1 {
    return Ok(());
  }
  file.write_all(b"<p>")?;
  file.write_fmt(format_args!("<i>incoming({})</i>: ", term.incoming_len))?;
  for i in 0..term.incoming_len {
    let incoming = &term.incoming[i as usize].borrow();
    file.write_fmt(format_args!(
      "<a href='{}.html'>{}</a> ",
      incoming.filename, incoming.name
    ))?;
  }
  file.write_all(b"</p>")?;
  Ok(())
}

fn build_index(
  file: &mut LineWriter<File>,
  lex: &Lexicon,
  term: &Term,
) -> Result<(), Box<dyn Error>> {
  println!("build_index");
  for child in term.children.iter().flatten() {
    let mut _c = child.as_ref().borrow().clone();
    file.write_fmt(format_args!(
      "<h3><a href='{}.html'>{}</a></h3>",
      _c.filename, _c.name,
    ))?;
    build_body_part(file, lex, &_c);
    build_list(file, &_c).unwrap();
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
  file.write_all(b"<figure>")?;
  file.write_fmt(format_args!(
    "<img src='../media/images/{}.jpg' alt='{} picture' width='900' style='margin-top: 0;' />",
    pict, name
  ))?;
  // if caption > 0 {
  // 	file.write(b"<figcaption>")?;
  // 	if let Some(_link) = link {
  // 		file.write_fmt(format_args!(
  // 			"<a href='{}.html'>{}</a> — {}",
  // 			_link, host, name
  // 		))?;
  // 	} else {
  // 		file.write_fmt(format_args!("{} — {}", host, name))?;
  // 	}
  // 	file.write(b"</figcaption>")?;
  // }
  file.write_all(b"</figure>")?;
  Ok(())
}
