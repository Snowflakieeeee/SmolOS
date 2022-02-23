use alloc::string::String;

use crate::println;

use function::Function;
use node::Node;
use types::Type;

mod error;
mod function;
mod lexer;
mod node;
mod parser;
mod splitter;
mod tokens;
mod types;

pub const KEYWORDS: [&str; 7] = ["fn", "if", "else", "while", "{", "}", ";"];
pub const DEFINED_FUNCTIONS: [Function; 1] = [Function::new(
    "print",
    print_to_console,
    &[Type::Number],
    Type::Null,
)];

pub fn run(filename: String, contents: &str) {
    let tokens = lexer::lex(filename, contents);
    let ast = parser::parse(tokens);
    println!("{:?}", ast);
}

pub fn print_to_console(args: &[Node]) -> Node {
    println!("{}", args[0]);
    Node::Null
}
