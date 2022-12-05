use std::fmt::format;
use std::vec;
use std::str::Chars;
use crate::token::{TokClass, Token};

enum GetCharStatus {
  EOF(String),
  OOB(String),
}

const LEGAL_PUNCTUATION: &'static [char] =
  &['\\', '-', '>', '<', '+', '-', '*', '/', '!', '(', ')'];

#[derive(Debug)]
pub struct Lexer {
  cnt: usize,
  src: String,
  tokens: Vec<Token>,
}

impl Lexer {
  pub fn new() -> Lexer {
    Lexer {
      cnt: 0,
      src: String::new(),
      tokens: vec![],
    }
  }

  pub fn next(&mut self) -> Token {
    let tok = self.tokens[self.cnt].clone();
    self.cnt += 1;
    tok
  }

  pub fn cnt(&self) -> usize {
    self.cnt
  }

  pub fn dump(&self) {
    println!("{:?}", &self.src);
    for tok in &self.tokens {
      tok.dump();
    }
  }

  fn get_ch(&mut self, idx: usize) -> Result<char, GetCharStatus> {
    if idx == self.src.as_bytes().len() {
      return Err(GetCharStatus::EOF(String::from("end of file")));
    }
    if let Some(ch) = self.src.chars().nth(idx) {
      Ok(ch)
    } else {
      Err(GetCharStatus::OOB(format!("i: {idx}, get char failed")))
    }
  }

  fn is_legal_punctuation(ch: char) -> bool {
    LEGAL_PUNCTUATION.contains(&ch)
  }

  fn eat_space(&mut self, idx: &mut usize) {
    match self.get_ch(*idx) {
      Ok(ch) => {
        if ch.is_whitespace() {
          *idx += 1;
        } else {
          return;
        }
      }
      Err(GetCharStatus::EOF(_)) => { return; }
      Err(GetCharStatus::OOB(msg)) => { panic!("{msg}"); }
    }
  }

  fn handle_var(&mut self, idx: &mut usize) -> Result<String, String> {
    let mut tok = String::new();
    loop {
      match self.get_ch(*idx) {
        Ok(ch) => {
          if ch.is_alphanumeric() {
            tok.push(ch);
            *idx += 1;
          } else {
            break;
          }
        }
        Err(GetCharStatus::EOF(_)) => { break; }
        Err(GetCharStatus::OOB(msg)) => { return Err(msg); }
      }
    }
    let res = Ok(tok.clone());
    self.tokens.push(Token::new(tok, TokClass::Var));
    res
  }

  fn handle_keyword(&mut self, idx: &mut usize) -> Result<String, String> {
    let mut tok = String::new();
    let mut last_ch = 0 as char;
    let ch0 = 0 as char;
    loop {
      match self.get_ch(*idx) {
        Ok(ch) => {
          if !ch.is_ascii_punctuation() { break; }
          if !Self::is_legal_punctuation(ch) {
            return Err(format!("i: {:?}, illegal punctuation '{ch}'", *idx));
          }
          match ch {
            '\\' => {
              tok.push(ch);
              *idx += 1;
              break;
            }
            '-' | '<' | '!' => {
              if last_ch != ch0 {
                break;
              }
              tok.push(ch);
              *idx += 1;
            }
            '>' => {
              tok.push(ch);
              *idx += 1;
              if last_ch == '-' {
                break;
              }
            }
            '+' | '*' | '/' => {
              if last_ch != ch0 {
                return Err(format!("i: {:?}, expect non-keyword before '{ch}'", *idx));
              }
              tok.push(ch);
              *idx += 1;
              break;
            }
            '=' => {
              tok.push(ch);
              *idx += 1;
              match last_ch {
                '=' | '>' | '<' | '!' => { break; }
                _ => {}
              }
              if last_ch != ch0 {
                *idx -= 1;
                return Err(format!("i: {:?}, expect non-keyword or ('=' | '>' | '<' | '!') before '{ch}'", *idx));
              }
            }
            '(' | ')' => {
              tok.push(ch);
              *idx += 1;
              break;
            }
            _ => {
              return Err(format!("i: {:?}, illegal character {ch}", *idx));
            }
          }
          last_ch = ch;
        }
        Err(GetCharStatus::EOF(_)) => { break; }
        Err(GetCharStatus::OOB(msg)) => { return Err(msg); }
      }
    }
    let res = Ok(tok.clone());
    self.tokens.push(Token::new(tok, TokClass::Keyword));
    res
  }

  fn handle_num(&mut self, idx: &mut usize) -> Result<String, String> {
    let mut tok = String::new();
    loop {
      match self.get_ch(*idx) {
        Ok(ch) => {
          if ch.is_digit(10) {
            tok.push(ch);
            *idx += 1;
          } else {
            break;
          }
        }
        Err(GetCharStatus::EOF(_)) => { break; }
        Err(GetCharStatus::OOB(msg)) => { return Err(msg); }
      }
    }
    let res = Ok(tok.clone());
    self.tokens.push(Token::new(tok, TokClass::NumLiteral));
    res
  }

  fn handle_bool(&mut self, idx: &mut usize) -> Result<String, String> {
    let mut tok = String::new();
    loop {
      match self.get_ch(*idx) {
        Ok(ch) => {
          if !ch.is_alphanumeric() { break; }
          tok.push(ch);
          *idx += 1;
        }
        Err(GetCharStatus::EOF(_)) => { break; }
        Err(GetCharStatus::OOB(msg)) => { return Err(msg); }
      }
    }
    let res = Ok(tok.clone());
    if (tok == "false") || (tok == "true") {
      self.tokens.push(Token::new(tok, TokClass::BoolLiteral));
    } else {
      self.tokens.push(Token::new(tok, TokClass::Var));
    }
    res
  }

  pub fn tokenize(&mut self, src: String) -> Result<String, String> {
    let mut i = 0;
    self.src = src;
    while i < self.src.len() {
      match self.get_ch(i) {
        Ok(ch) => {
          if ch.is_alphabetic() {
            if let Err(msg) = match ch {
              't' | 'f' => self.handle_bool(&mut i),
              _ => self.handle_var(&mut i)
            } {
              return Err(msg);
            }
          } else if ch.is_ascii_punctuation() {
            if let Err(msg) = self.handle_keyword(&mut i) {
              return Err(msg);
            }
          } else if ch.is_digit(10) {
            if let Err(msg) = self.handle_num(&mut i) {
              return Err(msg);
            }
          } else if ch.is_whitespace() {
            self.eat_space(&mut i);
          }
        }
        Err(GetCharStatus::EOF(_)) => { break; }
        Err(GetCharStatus::OOB(msg)) => return Err(msg),
      }
    }
    Ok(String::from("Tokenize successfully!"))
  }
}
