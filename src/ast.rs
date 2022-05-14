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
  Number(f64)
}
