use core::fmt::Display;

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

    pub fn merge(&self, other: &Self) -> Self {
        Self {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
            file: Rc::clone(&self.file),
        }
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
pub enum ErrorType {
    UndefinedWord,
    SyntaxError,
}

impl Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{:?} in {} at {} to {} :: {}",
            self.error, self.position.file, self.position.start, self.position.end, self.details
        )
    }
}
