#[derive(Debug)]
pub enum Statement {
  Let {
    name: String,
    initial_value: Expression
  },
  Const {
    name: String,
    initial_value: Expression
  }
}

#[derive(Debug)]
pub enum Expression {
  Number(f64),
  Binary(Box<Expression>, BinaryOperator, Box<Expression>)
}

#[derive(Debug)]
pub enum BinaryOperator {
  Plus,
  Minus,
  Multiply,
  Divide
}
