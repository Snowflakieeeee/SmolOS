use alloc::string::String;

use super::error::Position;

pub const KEYWORDS: [&str; 6] = ["let", "fn", "if", "else", "true", ";"];

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    position: Position,
    token: TokenType,
}

impl Token {
    pub fn new(position: Position, token: TokenType) -> Self {
        Self { position, token }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Identifier(String),
    Number(f64),
    Keyword(String),
    Eof,
}
