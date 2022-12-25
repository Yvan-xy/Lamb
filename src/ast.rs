use crate::visitor::Visitor;

/// base trait of AST node
pub trait ASTNode {
  /// the api for visitor pattern
  fn visit(&self, visitor: &dyn Visitor);

  /// dump AST node
  fn dump(&self);
}

#[derive(Debug)]
pub struct FuncDef {}

impl ASTNode for FuncDef {
  fn visit(&self, visitor: &dyn Visitor) {
    todo!()
  }

  fn dump(&self) {
    println!("{:#?}", *self);
  }
}

#[derive(Debug)]
pub struct VariableNode {
  name: String,
}

impl ASTNode for VariableNode {
  fn visit(&self, visitor: &dyn Visitor) {
    todo!()
  }

  fn dump(&self) {
    println!("{:#?}", *self);
  }
}

/// Top level AST node
#[derive(Debug)]
pub struct CompUnit {
  func_defs: Vec<Box<FuncDef>>,
}

impl CompUnit {
  pub fn new() -> CompUnit {
    CompUnit {
      func_defs: Vec::new()
    }
  }

  pub fn add_node(&mut self, node: Box<FuncDef>) {
    self.func_defs.push(node);
  }

  fn get_children(&self) -> &Vec<Box<FuncDef>> {
    &self.func_defs
  }
}

impl ASTNode for CompUnit {
  fn visit(&self, visitor: &dyn Visitor) {
    todo!()
  }

  fn dump(&self) {
    println!("{:#?}", *self);
  }
}
