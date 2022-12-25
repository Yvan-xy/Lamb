use std::fs;
use pretty_env_logger;
use crate::lexer::Lexer;
use log::{debug, error, info, warn, LevelFilter};
use crate::ast::{ASTNode, CompUnit};
use crate::parser::Parser;

mod ast;
mod token;
mod lexer;
mod parser;
mod visitor;

fn main() {
  pretty_env_logger::init();

  let file_path = "./a.l";
  let src = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");
  let mut lexer = Lexer::new();
  match lexer.tokenize(src) {
    Ok(_) => {
      // lexer.dump();
      let mut parser = Parser::new(&mut lexer);
      match parser.parse_top() {
        Ok(_) => {
          parser.dump();
        }
        Err(_) => {}
      }
    }
    Err(msg) => error!("{msg}"),
  }
}
