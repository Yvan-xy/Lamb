#[derive(Debug, Copy, Clone)]
pub enum TokClass {
  Var,
  Keyword,
  NumLiteral,
  BoolLiteral,
}

#[derive(Debug)]
pub struct Token {
  name: String,
  tok_class: TokClass,
}

impl Token {
  pub fn new(n: String, class: TokClass) -> Token {
    Token {
      name: n,
      tok_class: class,
    }
  }

  pub fn clone(&self) -> Token {
    Token {
      name: self.name.clone(),
      tok_class: self.tok_class,
    }
  }

  pub fn get_name(&self) -> String {
    return String::from(&self.name);
  }

  pub fn get_tok_class(&self) -> TokClass {
    return self.tok_class;
  }

  pub fn dump(&self) {
    print!("{:#?}\n", *self);
  }
}