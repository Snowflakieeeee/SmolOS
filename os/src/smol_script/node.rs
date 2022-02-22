use core::fmt::Display;

use alloc::{string::String, vec::Vec};

use super::{error::Position, tokens::Token};

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Number(Token),
    Nodes(Vec<Node>, Position),
    Word(String, Position),
}

impl Display for Node {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Node::Number(x) => write!(f, "{}", x),
            Node::Nodes(x, _) => write!(f, "{:?}", x),
            Node::Word(x, _) => write!(f, "{}", x),
        }
    }
}
