use crate::core::parser::AstError;
use std::fmt;

#[derive(Debug, Clone)]
pub enum ErrorCause {
    EntryNotFound(String),
    PathNotFound(String),
    DirectoryNotFound(String),
    FileNotFound(String),
    CouldNotSerializeData(String),
    CouldNotDeserializeData(String),
    CouldNotCreateFile(String),
    CouldNotOpenFile(String),
    CouldNotWriteFile(String),
    CouldNotReadFile(String),
    UnexpectedError(String),
    SyntaxError(AstError),
    MutexUnwrapError(String),
    UnexpectedChannelError(String),
}

impl fmt::Display for ErrorCause {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorCause::EntryNotFound(entry) => write!(f, "Entry not found: {}", entry),
            ErrorCause::PathNotFound(path) => write!(f, "Path not found: {}", path),
            ErrorCause::DirectoryNotFound(path) => write!(f, "Directory not found: {}", path),
            ErrorCause::FileNotFound(file) => write!(f, "File not found: {}", file),
            ErrorCause::CouldNotSerializeData(file) => write!(f, "Could not serialize data: {}", file),
            ErrorCause::CouldNotDeserializeData(file) => write!(f, "Could not deserialize data: {}", file),
            ErrorCause::CouldNotCreateFile(file) => write!(f, "Could not create file: {}", file),
            ErrorCause::CouldNotOpenFile(file) => write!(f, "Could not open file: {}", file),
            ErrorCause::CouldNotWriteFile(file) => write!(f, "Could not write to file: {}", file),
            ErrorCause::CouldNotReadFile(file) => write!(f, "Could not read file: {}", file),
            ErrorCause::UnexpectedError(err) => write!(f, "Unexpected error: {}", err),
            ErrorCause::SyntaxError(error) => write!(f, "Syntax error: {}", error),
            ErrorCause::MutexUnwrapError(error) => write!(f, "Mutex unwrap error: {}", error),
            ErrorCause::UnexpectedChannelError(error) => write!(f, "Unexpected channel error: {}", error),
        }
    }
}
