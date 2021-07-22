#![allow(dead_code)]
use std::error::Error;

mod lib;
use lib::builder::build;
use lib::error::SkiffError;
use lib::glossary::{glossary_link, parse_glossary, Glossary};
use lib::journal::{journal_link, parse_journals, Journal};
use lib::lexicon::{lexicon_link, parse_lexicon, Lexicon};
use lib::template::link_next_prev;

fn parse(all_lists: &mut Glossary, all_terms: &mut Lexicon, all_logs: &mut Journal) {
  println!("Parsing  | ");
  parse_glossary(String::from("./database/glossary.ndtl"), all_lists).unwrap();
  parse_lexicon(String::from("./database/lexicon.ndtl"), all_terms).unwrap();
  parse_journals(String::from("./database/journals.ndtl"), all_logs).unwrap();
}

fn link(glo: &mut Glossary, lex: &mut Lexicon, jou: &mut Journal) -> Result<(), SkiffError> {
  println!("Linking  | ");
  lexicon_link(jou, lex);
  glossary_link(lex, glo);
  journal_link(lex, glo);
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

fn main() {
  let all_terms = &mut Lexicon::new();
  let all_lists = &mut Glossary::new();
  let all_logs = &mut Journal::new();
  parse(all_lists, all_terms, all_logs);
  link(all_lists, all_terms, all_logs).unwrap();
  link_next_prev(&mut all_terms.terms).unwrap();
  build(all_terms, all_logs).unwrap();
  check(all_terms, all_lists, all_logs).unwrap();
}
