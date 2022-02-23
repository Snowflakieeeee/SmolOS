use core::fmt::Display;

use alloc::vec::Vec;

use super::{error::Position, tokens::Token};

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Number(Token),
    Nodes(Vec<Node>, Position),
    Function(Token),
}

// impl Node {
//     pub fn position(&self) -> &Position {
//         match self {
//             Node::Number(token) => &token.position,
//             Node::Nodes(_, position) => position,
//             Node::Function(token) => &token.position,
//         }
//     }
// }

impl Display for Node {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Node::Number(x) => write!(f, "{}", x),
            Node::Nodes(x, _) => write!(f, "{:?}", x),
            Node::Function(x, ..) => write!(f, "{}", x),
        }
    }
}
