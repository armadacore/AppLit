use std::{fmt, error::Error};

#[derive(Debug)]
pub enum ErrorCause {
    Unhandled(String),
    PathNotFound(String),
    FileNotFound(String),
    UnknownToken(String),
    TokenNotImplemented(String),
}

impl fmt::Display for ErrorCause {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorCause::Unhandled(msg) => write!(f, "Unhandled error: {}", msg),
            ErrorCause::PathNotFound(path) => write!(f, "Path not found: {}", path),
            ErrorCause::FileNotFound(file) => write!(f, "File not found: {}", file),
            ErrorCause::UnknownToken(token) => write!(f, "Unknown token: {}", token),
            ErrorCause::TokenNotImplemented(token) => write!(f, "Token not implemented: {}", token),
        }
    }
}

#[derive(Debug)]
pub struct ErrorFeedback {
    cause: ErrorCause, 
}

impl fmt::Display for ErrorFeedback {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.cause)
    }
}

impl Error for ErrorFeedback {}
impl ErrorFeedback {
    pub fn new(message: &str) -> Self {
        ErrorFeedback {
            cause: ErrorCause::Unhandled(message.into())
        }
    }
    
    pub fn from(error: &dyn Error) -> Self {
        ErrorFeedback {
            cause: ErrorCause::Unhandled(error.to_string())
        }
    }
    
    pub fn cause(cause: ErrorCause) -> Self {
        ErrorFeedback { cause }
    }
}