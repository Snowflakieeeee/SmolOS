use alloc::string::String;

use crate::println;

mod error;
mod lexer;
mod node;
mod parser;
mod splitter;
mod tokens;

pub fn run(filename: String, contents: &str) {
    println!("{:?}", parser::parse(lexer::lex(filename, contents)));
}
