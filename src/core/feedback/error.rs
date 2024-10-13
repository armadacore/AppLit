use crate::core::parser::AstError;
use std::fmt;

#[derive(Debug, Clone)]
pub enum ErrorCause<'a> {
    EntryNotFound(&'a str),
    PathNotFound(&'a str),
    DirectoryNotFound(&'a str),
    FileNotFound(&'a str),
    SyntaxError(AstError)
}

impl<'a> fmt::Display for ErrorCause<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorCause::EntryNotFound(entry) => write!(f, "Entry not found: {}", entry),
            ErrorCause::PathNotFound(path) => write!(f, "Path not found: {}", path),
            ErrorCause::DirectoryNotFound(path) => write!(f, "Directory not found: {}", path),
            ErrorCause::FileNotFound(file) => write!(f, "File not found: {}", file),
            ErrorCause::SyntaxError(error) => {write!(f, "Syntax error: {}", error) },
        }
    }
}
