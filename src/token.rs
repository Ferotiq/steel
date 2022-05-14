#[derive(Debug, PartialEq)]
pub enum TokenKind {
  Identifier,
  OperatorAssignment,
  OperatorInfixPlus,
  OperatorInfixMinus,
  OperatorInfixMultiply,
  OperatorInfixDivide,
  Declarator,
  LiteralString,
  Number,
  EndOfLine
}

#[derive(Debug)]
pub struct Token {
  pub kind: TokenKind,
  pub literal: String,
}

impl Token {
  pub fn new(kind: TokenKind, literal: String) -> Self {
    Self { kind, literal }
  }
}
