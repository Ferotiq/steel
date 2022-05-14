use crate::lexer;
use crate::ast;
use crate::token;

use lexer::Lexer;
use ast::{Statement, Expression, BinaryOperator};
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

          let expression = self.parse_expression(0);

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
  
  fn parse_expression(&mut self, bp: u8) -> Expression {
    let mut left = match self.lexer.next() {
      Some(Token { kind: TokenKind::Number, literal }) => {
        let parsed_number = if let Ok(n) = literal.parse::<f64>() {
          n
        } else {
          panic!("Number provided is not a valid number.");
        };

        Expression::Number(parsed_number)
      },
      _ => unimplemented!()
    };

    loop {
      let infix = if let Some(infix) = self.lexer.peek() {
        infix
      } else {
        break;
      };

      if let Some((left_bp, right_bp)) = infix_binding_power(infix.kind) {
        if left_bp < bp {
          break;
        }

        let next_operator = self.lexer.next().unwrap();

        let right = self.parse_expression(right_bp);

        left = make_infix_expression(left, next_operator, right);

        continue;
      } else {
        break;
      }
    }

    return left;
  }
}

fn infix_binding_power(kind: TokenKind) -> Option<(u8, u8)> {
  let bp = match kind {
    TokenKind::OperatorInfixPlus | TokenKind::OperatorInfixMinus => (6, 7),
    TokenKind::OperatorInfixMultiply | TokenKind::OperatorInfixDivide => (8, 9),
    _ => return None,
  };

  return Some(bp);
}

fn make_infix_expression(left: Expression, operator: Token, right: Expression) -> Expression {
  let left = Box::new(left);
  let right = Box::new(right);

  return match operator.kind {
    TokenKind::OperatorInfixPlus => Expression::Binary(left, BinaryOperator::Plus, right),
    TokenKind::OperatorInfixMinus => Expression::Binary(left, BinaryOperator::Minus, right),
    TokenKind::OperatorInfixMultiply => Expression::Binary(left, BinaryOperator::Multiply, right),
    TokenKind::OperatorInfixDivide => Expression::Binary(left, BinaryOperator::Divide, right),
    _ => unimplemented!()
  }
}

pub type Program = Vec<Statement>;
