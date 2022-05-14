#[derive(Debug)]
pub enum TokenKind {
  Identifier,
  OperatorAssignment,
  Declarator,
  LiteralString,
  Number,
  EndOfLine
}

#[derive(Debug)]
pub struct Token {
  kind: TokenKind,
  literal: String,
}

impl Token {
  pub fn new(kind: TokenKind, literal: String) -> Self {
    Self { kind, literal }
  }
}
