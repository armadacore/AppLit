use std::{error::Error, fmt};

#[derive(Debug)]
pub struct SyntaxErrorLocation {
    pub start: usize,
    pub end: usize,
    pub line_start: usize,
    pub line_end: usize,
}

#[derive(Debug)]
pub struct SyntaxErrorCause {
    pub location: SyntaxErrorLocation,
    pub cause: String,
}

impl fmt::Display for SyntaxErrorCause {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Syntax error: {} at {}:{}", self.cause, self.location.line_start, 999)
    }
}

#[derive(Debug)]
pub enum ErrorCause {
    Unhandled(Box<dyn Error>),
    PathNotFound(String),
    FileNotFound(String),
    UnknownToken(String),
    TokenNotImplemented(String),
    SyntaxError(SyntaxErrorCause)
}

impl fmt::Display for ErrorCause {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorCause::Unhandled(error) => write!(f, "Unhandled error: {}", error),
            ErrorCause::SyntaxError(error) => {write!(f, "Syntax error: {}", error) },
            ErrorCause::PathNotFound(path) => write!(f, "Path not found: {}", path),
            ErrorCause::FileNotFound(file) => write!(f, "File not found: {}", file),
            ErrorCause::UnknownToken(token) => write!(f, "Unknown token: {}", token),
            ErrorCause::TokenNotImplemented(token) => write!(f, "Token not implemented: {}", token),
        }
    }
}
