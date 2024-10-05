use std::fmt;

#[derive(Debug, Clone)]
pub struct SyntaxErrorLocation {
    pub start: usize,
    pub end: usize,
    pub line_start: usize,
    pub line_end: usize,
}

#[derive(Debug, Clone)]
pub struct SyntaxErrorCause {
    pub location: SyntaxErrorLocation,
    pub cause: String,
}

impl fmt::Display for SyntaxErrorCause {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Syntax error: {} at {}:{}", self.cause, self.location.line_start, 999)
    }
}

#[derive(Debug, Clone)]
pub enum ErrorCause<'a> {
    PathNotFound(&'a str),
    FileNotFound(&'a str),
    UnknownToken(&'a str),
    TokenNotImplemented(&'a str),
    SyntaxError(SyntaxErrorCause)
}

impl<'a> fmt::Display for ErrorCause<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // ErrorCause::Unhandled(error) => write!(f, "Unhandled error: {}", error),
            ErrorCause::SyntaxError(error) => {write!(f, "Syntax error: {}", error) },
            ErrorCause::PathNotFound(path) => write!(f, "Path not found: {}", path),
            ErrorCause::FileNotFound(file) => write!(f, "File not found: {}", file),
            ErrorCause::UnknownToken(token) => write!(f, "Unknown token: {}", token),
            ErrorCause::TokenNotImplemented(token) => write!(f, "Token not implemented: {}", token),
        }
    }
}
