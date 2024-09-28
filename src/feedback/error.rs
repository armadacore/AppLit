use std::{error::Error, fmt};

#[derive(Debug)]
pub enum ErrorCause {
    Unhandled(Box<dyn Error>),
    PathNotFound(String),
    FileNotFound(String),
    UnknownToken(String),
    TokenNotImplemented(String),
}

impl fmt::Display for ErrorCause {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorCause::Unhandled(error) => write!(f, "Unhandled error: {}", error),
            ErrorCause::PathNotFound(path) => write!(f, "Path not found: {}", path),
            ErrorCause::FileNotFound(file) => write!(f, "File not found: {}", file),
            ErrorCause::UnknownToken(token) => write!(f, "Unknown token: {}", token),
            ErrorCause::TokenNotImplemented(token) => write!(f, "Token not implemented: {}", token),
        }
    }
}
