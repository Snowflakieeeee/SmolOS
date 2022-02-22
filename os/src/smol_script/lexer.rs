use super::{
    error::Position,
    splitter::Splitter,
    tokens::{Token, TokenType, KEYWORDS},
};
use alloc::{rc::Rc, string::String, vec::Vec};

pub fn lex(filename: String, contents: &str) -> Vec<Token> {
    let file = Rc::new(filename);
    let mut tokens = Splitter::new(|&(_, x)| !x.is_whitespace(), contents.chars().enumerate())
        .map(|x| {
            let mut x = x.peekable();
            let start = x.peek().unwrap().0;
            let mut end = start;
            let mut word = String::new();
            for (i, c) in x {
                word.push(c);
                end = i;
            }
            match word.parse::<f64>().ok() {
                Some(x) => Token::new(
                    Position::new(start, end, Rc::clone(&file)),
                    TokenType::Number(x as f64),
                ),
                None if KEYWORDS.contains(&&*word) => Token::new(
                    Position::new(start, end, Rc::clone(&file)),
                    TokenType::Keyword(word),
                ),
                None => Token::new(
                    Position::new(start, end, Rc::clone(&file)),
                    TokenType::Identifier(word),
                ),
            }
        })
        .collect::<Vec<_>>();

    tokens.push(Token::new(
        Position::new(contents.len(), contents.len(), Rc::clone(&file)),
        TokenType::Eof,
    ));

    tokens
}
