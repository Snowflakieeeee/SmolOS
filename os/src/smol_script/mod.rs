use alloc::string::String;

use crate::println;

use function::Function;

mod error;
mod function;
mod lexer;
mod node;
mod parser;
mod splitter;
mod tokens;

pub const KEYWORDS: [&str; 7] = ["fn", "if", "else", "while", "{", "}", ";"];
pub const DEFINED_FUNCTIONS: [Function; 1] = [Function::new("print")];

pub fn run(filename: String, contents: &str) {
    let tokens = lexer::lex(filename, contents);
    let ast = parser::parse(tokens);
    println!("{:?}", ast);
}
