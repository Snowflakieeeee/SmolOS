use alloc::{format, string::String, vec::Vec};

use super::{
    error::{Error, ErrorType},
    node::Node,
    tokens::{Token, TokenType},
};

type ParseResult = Result<Node, Error>;

struct Parser {
    tokens: Vec<Token>,
    current: Token,
    defined: Vec<String>,
    index: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            current: tokens.first().cloned().unwrap(),
            tokens,
            defined: Vec::new(),
            index: 0,
        }
    }

    fn advance(&mut self) {
        self.index += 1;
        if let Some(token) = self.tokens.get(self.index) {
            self.current = token.clone();
        }
    }

    fn statements(&mut self) -> ParseResult {
        let pos = self.current.position.clone();
        let mut statements = Vec::new();
        while self.current.token != TokenType::Eof {
            statements.push(self.statement()?);
        }
        self.advance();
        Ok(Node::Nodes(statements, pos.merge(&self.current.position)))
    }

    fn statement(&mut self) -> ParseResult {
        match self.current.token {
            TokenType::Identifier(ref ident) => {
                if !self.is_defined(&ident) {
                    Err(Error::new(
                        ErrorType::UndefinedWord,
                        format!("'{}' is not defined", ident),
                        self.current.position.clone(),
                    ))
                } else {
                    Ok(Node::Word(ident.clone(), self.current.position.clone()))
                }
            }
            TokenType::Number(_) => Ok(Node::Number(self.current.clone())),
            TokenType::Keyword(_) => todo!(),
            _ => Err(Error::new(
                ErrorType::SyntaxError,
                format!("Unexpected token: '{}'", self.current),
                self.current.position.clone(),
            )),
        }
    }

    fn is_defined(&self, token: &String) -> bool {
        self.defined.contains(token)
    }
}

pub fn parse(tokens: Vec<Token>) -> ParseResult {
    Parser::new(tokens).statements()
}
