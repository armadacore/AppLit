use crate::core::parser::AstError;
use std::fmt;

#[derive(Debug, Clone)]
pub enum ErrorCause<'a> {
    PathNotFound(&'a str),
    FileNotFound(&'a str),
    SyntaxError(AstError)
}

impl<'a> fmt::Display for ErrorCause<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorCause::PathNotFound(path) => write!(f, "Path not found: {}", path),
            ErrorCause::FileNotFound(file) => write!(f, "File not found: {}", file),
            ErrorCause::SyntaxError(error) => {write!(f, "Syntax error: {}", error) },
        }
    }
}
