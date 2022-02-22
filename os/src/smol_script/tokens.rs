use core::fmt::Display;

use alloc::string::String;

use super::error::Position;

pub const KEYWORDS: [&str; 6] = ["let", "fn", "if", "else", "true", ";"];

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub position: Position,
    pub token: TokenType,
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

impl Display for Token {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.token)
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            TokenType::Identifier(x) => write!(f, "{}", x),
            TokenType::Number(x) => write!(f, "{}", x),
            TokenType::Keyword(x) => write!(f, "{}", x),
            TokenType::Eof => write!(f, "EOF"),
        }
    }
}
