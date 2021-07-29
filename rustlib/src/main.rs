#![allow(dead_code)]
use std::error::Error;

mod lib;
use lib::builder::build;
use lib::error::SkiffError;
use lib::glossary;
use lib::glossary::Glossary;
use lib::journal;
use lib::journal::Journal;
use lib::lexicon;
use lib::lexicon::Lexicon;
use lib::template::link_next_prev;

fn parse(all_lists: &mut Glossary, all_terms: &mut Lexicon, all_logs: &mut Journal) {
  println!("Parsing  | ");
  glossary::parse(String::from("./database/glossary.ndtl"), all_lists).unwrap();
  lexicon::parse(String::from("./database/lexicon.ndtl"), all_terms).unwrap();
  journal::parse(String::from("./database/journals.ndtl"), all_logs).unwrap();
}

fn link(glo: &mut Glossary, lex: &mut Lexicon, jou: &mut Journal) -> Result<(), SkiffError> {
  println!("Linking  | ");
  lexicon::link(jou, lex);
  glossary::link(lex, glo);
  journal::link(lex, glo);

  lexicon::get_home_depth(lex).unwrap();
  Ok(())
}

fn check(lex: &Lexicon, glo: &Glossary, jou: &Journal) -> Result<(), Box<dyn Error>> {
  println!("Checking | ");
  journal::check(jou); /* Find invalid logs */
  glossary::check(glo); /* Find unlinked lists */
  lexicon::check(lex); /* Find unlinked pages */
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
