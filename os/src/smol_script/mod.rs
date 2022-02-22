use alloc::string::String;

use crate::println;

mod error;
mod lexer;
mod splitter;
mod tokens;

pub fn run(filename: String, contents: &str) {
    println!("{:?}", lexer::lex(filename, contents));
}
