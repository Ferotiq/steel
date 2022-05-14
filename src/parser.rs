use crate::lexer;
use crate::ast;
use crate::token;

use lexer::Lexer;
use ast::{Statement, Expression};
use token::{Token, TokenKind};

pub struct Parser {
  lexer: Lexer,
}

impl Parser {
  pub fn new(lexer: Lexer) -> Self {
    Self { lexer }
  }

  pub fn parse(&mut self) -> Program {
    let mut statements = Program::new();

    while let Some(token) = self.lexer.next() {
      match token.kind {
        TokenKind::Declarator => {
          let identifier = if let Some(i) = self.lexer.next() {
            i
          } else {
            panic!("Expected identifier.");
          };
          
          if identifier.kind != TokenKind::Identifier {
            panic!("Expected identifier.");
          }

          if !matches!(self.lexer.peek(), Some(Token { kind: TokenKind::OperatorAssignment, .. })) {
            panic!("Expected assignment operator.");
          }
          
          self.lexer.next();

          let expression = self.parse_expression();

          // check for end of line
          if !matches!(self.lexer.peek(), Some(Token { kind: TokenKind::EndOfLine, .. })) {
            panic!("Expected end of line.");
          }

          self.lexer.next();

          if token.literal == "const" {
            statements.push(Statement::Const {
              name: identifier.literal,
              initial_value: expression
            });
          } else {
            statements.push(Statement::Let {
              name: identifier.literal,
              initial_value: expression
            });
          }

        },
        _ => {
          println!("{:?}", token);
        }
      }
    }

    return statements;
  }
  
  fn parse_expression(&mut self) -> Expression {
    match self.lexer.next() {
      Some(Token { kind: TokenKind::Number, literal }) => {
        let parsed_number = if let Ok(n) = literal.parse::<f64>() {
          n
        } else {
          panic!("Number provided is not a valid number.");
        };

        Expression::Number(parsed_number)
      },
      _ => unimplemented!()
    }
  }
}

pub type Program = Vec<Statement>;
