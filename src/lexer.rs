use crate::token;

use token::{Token, TokenKind};

pub struct Lexer {
  source: Vec<char>,
  counter: usize,
}

impl Lexer {
  pub fn new(content: String) -> Self {
    Self {
      source: content.chars().collect(),
      counter: 0
    }
  }

  pub fn lex(&mut self) {
    let mut tokens = Vec::<Token>::new();

    loop {
      let c = self.current_char();

      match c {
        ';' => {
          tokens.push(Token::new(TokenKind::EndOfLine, ";".to_owned()));

          if !self.inc() {
            break;
          }
        },
        '=' => {
          tokens.push(Token::new(TokenKind::OperatorAssignment, "=".to_owned()));

          if !self.inc() {
            break;
          }
        },
        '"' | '\'' | '`' => {
          if !self.inc() {
            break;
          }

          let mut buffer = String::new();

          // TODO add support for templates
          while self.current_char() != c {
            // TODO improve backslash escaping
            if self.current_char() == '\\' {
              if !self.inc() {
                break;
              }
            }

            buffer.push(self.current_char());
  
            if !self.inc() {
              break;
            }
          }

          tokens.push(Token::new(TokenKind::LiteralString, buffer));

          if !self.inc() {
            break;
          }
        },
        _ if c.is_numeric() => {
          let mut buffer = String::new();

          buffer.push(c);

          if !self.inc() {
            break;
          }

          while
            self.current_char().is_numeric() ||
            self.current_char() == '.' ||
            self.current_char() == '_'
          {
            if self.current_char() != '_' {
              buffer.push(self.current_char());
            }

            if !self.inc() {
              break;
            }
          }

          tokens.push(Token::new(TokenKind::Number, buffer));
        },
        _ if self.current_char().is_alphabetic() || self.current_char() == '_' => {
          let mut buffer = String::new();

          buffer.push(c);

          if !self.inc() {
            break;
          }

          while 
            self.current_char().is_alphabetic() ||
            self.current_char().is_numeric() ||
            self.current_char() == '_'
          {
            buffer.push(self.current_char());

            if !self.inc() {
              break;
            }
          }

          let kind: TokenKind = match buffer.as_str() {
            "const" | "let" => TokenKind::Declarator,
            _ => TokenKind::Identifier
          };

          tokens.push(Token::new(kind, buffer));
        },
        _ => {
          if !self.inc() {
            break;
          }
        },
      }
    }

    println!("{:?}", tokens);
  }

  fn current_char(&self) -> char {
    *self.source.get(self.counter).unwrap()
  }

  fn inc(&mut self) -> bool {
    if self.counter < self.source.len() - 1 {
      self.counter += 1;

      return true;
    } else {
      return false;
    }
  }
}
