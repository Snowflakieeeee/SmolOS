use alloc::{borrow::ToOwned, format, vec::Vec};

use super::{
    error::{Error, ErrorType},
    function::Function,
    node::Node,
    tokens::{Token, TokenType},
    DEFINED_FUNCTIONS,
};

type ParseResult = Result<Node, Error>;

struct Parser {
    tokens: Vec<Token>,
    current: Token,
    defined: Vec<Function>,
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
            statements.push(self.expression()?);
        }
        self.advance();
        Ok(Node::Nodes(statements, pos.merge(&self.current.position)))
    }

    fn expression(&mut self) -> ParseResult {
        match self.current.token {
            TokenType::Identifier(ref ident) => {
                if let Some(f) = self.defined.iter().find(|f| f.name() == ident).cloned() {
                    self.advance();
                    let mut args = Vec::new();
                    for _ in 0..f.args().len() {
                        args.push(self.expression()?);
                    }
                    Ok(Node::Function(f.name().to_owned(), f.ret(), args))
                } else if let Some(f) = DEFINED_FUNCTIONS.iter().find(|f| f.name() == ident) {
                    self.advance();
                    let mut args = Vec::new();
                    for _ in 0..f.args().len() {
                        args.push(self.expression()?);
                    }
                    Ok(Node::Function(f.name().to_owned(), f.ret(), args))
                } else {
                    Err(Error::new(
                        ErrorType::UndefinedWord,
                        format!("'{}' is not defined", ident),
                        self.current.position.clone(),
                    ))
                }
            }
            TokenType::Number(_) => {
                let node = Ok(Node::Number(self.current.clone()));
                self.advance();
                node
            }
            TokenType::Keyword(_) => todo!(),
            _ => Err(Error::new(
                ErrorType::SyntaxError,
                format!("Unexpected token: '{}'", self.current),
                self.current.position.clone(),
            )),
        }
    }
}

pub fn parse(tokens: Vec<Token>) -> ParseResult {
    Parser::new(tokens).statements()
}
