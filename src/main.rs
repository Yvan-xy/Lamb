use std::fs;
use pretty_env_logger;
use crate::lexer::Lexer;
use log::{debug, error, info, warn, LevelFilter};

mod token;
mod lexer;
mod parser;

fn main() {
  pretty_env_logger::init();

  let file_path = "./a.l";
  let src = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");
  let mut lexer = Lexer::new();
  match lexer.tokenize(src) {
    Ok(_) => {}
    Err(msg) => error!("{msg}"),
  }
  lexer.dump();
}
