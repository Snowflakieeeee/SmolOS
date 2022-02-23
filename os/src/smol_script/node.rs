use core::fmt::Display;

use alloc::{string::String, vec::Vec};

use super::{error::Position, tokens::Token, types::Type};

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Number(Token),
    Nodes(Vec<Node>, Position),
    Function(String, Type, Vec<Node>),
    Null,
}

impl Display for Node {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Node::Number(x) => write!(f, "{}", x),
            Node::Nodes(x, _) => write!(f, "{:?}", x),
            Node::Function(x, ..) => write!(f, "{}", x),
            Node::Null => write!(f, "Null"),
        }
    }
}
