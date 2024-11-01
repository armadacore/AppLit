use crate::core::parser::error::AstError;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Cause {
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

impl fmt::Display for Cause {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cause::EntryNotFound(entry) => write!(f, "Entry not found: {}", entry),
            Cause::PathNotFound(path) => write!(f, "Path not found: {}", path),
            Cause::DirectoryNotFound(path) => write!(f, "Directory not found: {}", path),
            Cause::FileNotFound(file) => write!(f, "File not found: {}", file),
            Cause::CouldNotSerializeData(file) => write!(f, "Could not serialize data: {}", file),
            Cause::CouldNotDeserializeData(file) => write!(f, "Could not deserialize data: {}", file),
            Cause::CouldNotCreateFile(file) => write!(f, "Could not create file: {}", file),
            Cause::CouldNotOpenFile(file) => write!(f, "Could not open file: {}", file),
            Cause::CouldNotWriteFile(file) => write!(f, "Could not write to file: {}", file),
            Cause::CouldNotReadFile(file) => write!(f, "Could not read file: {}", file),
            Cause::UnexpectedError(err) => write!(f, "Unexpected error: {}", err),
            Cause::SyntaxError(error) => write!(f, "Syntax error: {}", error),
            Cause::MutexUnwrapError(error) => write!(f, "Mutex unwrap error: {}", error),
            Cause::UnexpectedChannelError(error) => write!(f, "Unexpected channel error: {}", error),
        }
    }
}
