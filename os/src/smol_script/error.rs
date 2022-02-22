use alloc::{rc::Rc, string::String};

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub start: usize,
    pub end: usize,
    pub file: Rc<String>,
}

impl Position {
    pub fn new(start: usize, end: usize, file: Rc<String>) -> Self {
        Self { start, end, file }
    }
}

#[derive(Debug, Clone)]
pub struct Error {
    error: ErrorType,
    details: String,
    position: Position,
}

impl Error {
    pub fn new(error: ErrorType, details: String, position: Position) -> Self {
        Self {
            error,
            details,
            position,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorType {}
