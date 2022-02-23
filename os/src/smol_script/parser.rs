use alloc::{format, vec::Vec};

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
        let token = self.current.clone();
        match token.token {
            TokenType::Identifier(ref ident) => {
                if self.defined.iter().any(|f| f.name() == ident)
                    || DEFINED_FUNCTIONS.iter().any(|f| f.name() == ident)
                {
                    self.advance();
                    Ok(Node::Function(token))
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
