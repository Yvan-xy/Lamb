use std::borrow::Borrow;
use crate::ast::{ASTNode, CompUnit, FuncDef};
use crate::lexer::Lexer;
use crate::token::Token;

pub struct Parser<'a> {
  lexer: &'a mut Lexer,
  ast: Option<Box<CompUnit>>,
}

impl<'a> Parser<'a> {
  pub fn new(lex: &'a mut Lexer) -> Parser {
    Parser {
      lexer: lex,
      ast: None,
    }
  }

  pub fn parse_func_def(&mut self) -> Result<Option<Box<FuncDef>>, String> {
    Ok(None)
  }

  /// Parse the compile unit
  pub fn parse_top(&mut self) -> Result<String, String> {
    let mut comp_unit = CompUnit::new();
    while self.lexer.cnt() < self.lexer.get_tokens().len() {
      match self.parse_func_def() {
        Ok(Some(node)) => { comp_unit.add_node(node) }
        Ok(None) => { break; }
        Err(msg) => { return Err(msg); }
      }
    }
    self.ast = Some(Box::new(comp_unit));
    Ok(String::from("Parse successfully!"))
  }

  /// dump the whole AST
  pub fn dump(&self) {
    match self.ast.as_ref() {
      Some(ast) => {
        ast.dump()
      }
      None => {}
    }
  }

}