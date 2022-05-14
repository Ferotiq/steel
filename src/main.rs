mod lexer;
mod token;

use std::fs;
use std::env;
use lexer::Lexer;
use token::Token;

fn main() {
  let file_path_option = env::args().nth(1);

  let file_path = if let Some(fp) = file_path_option {
    fp
  } else {
    panic!("No file argument provided.");
  };

  let file_content_result = fs::read_to_string(file_path);
  
  let file_content = if file_content_result.is_ok() {
    file_content_result.unwrap()
  } else {
    panic!("Could not open file for reading.");
  };

  let lexer = Lexer::new(file_content);

  let mut tokens = Vec::<Token>::new();

  for token in lexer {
    tokens.push(token);
  }

  println!("{:?}", tokens);
}
