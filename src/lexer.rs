use crate::token;

use token::{Token, TokenKind};

pub struct Lexer {
  source: Vec<char>,
  current: usize,
  next: usize,
  char: char,
}

impl Lexer {
  pub fn new(input: String) -> Self {
    let mut s = Self {
      source: input.chars().collect(),
      current: 0,
      next: 1,
      char: '\0'
    };

    s.char = s.source[s.current];

    return s;
  }

  fn read(&mut self) {
    if self.next >= self.source.len() {
      self.char = '\0';
    } else {
      self.char = self.source[self.next];
    }

    self.current = self.next;
    self.next = self.current + 1;
  }

  fn skip_whitespace(&mut self) {
    while self.char.is_whitespace() {
      self.read();
    }
  }

  pub fn match_token(&mut self) -> Token {
    self.skip_whitespace();

    return match self.char {
      ';' | '\0' => {
        self.read();

        Token::new(TokenKind::EndOfLine, ";".to_owned())
      },
      '=' => {
        self.read();

        Token::new(TokenKind::OperatorAssignment, "=".to_owned())
      },
      '+' => {
        self.read();

        Token::new(TokenKind::OperatorInfixPlus, "+".to_owned())
      },
      '-' => {
        self.read();

        Token::new(TokenKind::OperatorInfixMinus, "-".to_owned())
      },
      '*' => {
        self.read();

        Token::new(TokenKind::OperatorInfixMultiply, "*".to_owned())
      },
      '/' => {
        self.read();

        Token::new(TokenKind::OperatorInfixDivide, "/".to_owned())
      },
      '"' | '\'' | '`' => {
        let string_symbol = self.char;

        let mut buffer = String::new();

        self.read();

        // TODO add support for templates
        while self.char != string_symbol {
          // TODO improve backslash escaping
          if self.char == '\\' {
            self.read();
          }

          buffer.push(self.char);

          self.read();
        }

        Token::new(TokenKind::LiteralString, buffer)
      },
      _ if self.char.is_numeric() => {
        let mut buffer = String::new();

        buffer.push(self.char);

        self.read();

        // TODO add scientific and hex notation
        while
          self.char.is_numeric() ||
          self.char == '.' ||
          self.char == '_'
        {
          if self.char != '_' {
            buffer.push(self.char);
          }

          self.read();
        }

        Token::new(TokenKind::Number, buffer)
      },
      _ if self.char.is_alphabetic() || self.char == '_' => {
        let mut buffer = String::new();

        buffer.push(self.char);

        self.read();

        while 
          self.char.is_alphabetic() ||
          self.char.is_numeric() ||
          self.char == '_'
        {
          buffer.push(self.char);

          self.read();
        }

        let kind: TokenKind = match buffer.as_str() {
          "const" | "let" => TokenKind::Declarator,
          _ => TokenKind::Identifier
        };

        Token::new(kind, buffer)
      },
      _ => {
        println!("{:?}", self.char);

        unimplemented!()
      }
    };
  }

  pub fn peek(&mut self) -> Option<Token> {
    if self.next >= self.source.len() {
      return None;
    }

    let old_current = self.current;
    let old_next = self.next;
    let old_char = self.char;

    self.char = self.source[self.next];

    let token = self.match_token();

    self.current = old_current;
    self.next = old_next;
    self.char = old_char;

    return Some(token);
  }
}

impl Iterator for Lexer {
  type Item = Token;

  fn next(&mut self) -> Option<Token> {
    if self.next >= self.source.len() {
      return None;
    }

    let token = self.match_token();

    return Some(token);
  
  }
}
