use crate::bin::constants;
use crate::core::execute::token_reader::{TokenReaderLocation, TokenReaderStack};
use std::fmt::Debug;

#[derive(Debug)]
pub enum SyntaxErrorKind {
    Missing(String),
    Unknown(String),
    Declaration(String),
}

#[derive(Debug)]
pub struct SyntaxErrorDeclaration {
    pub location: TokenReaderLocation,
    pub kind: SyntaxErrorKind,
}

const MISSING: &str = "Missing semicolon. Expected ';' at the end of the statement.";
const UNKNOWN: &str = "Unknown token";

pub fn report<T: Debug>(stack: &mut TokenReaderStack<T>) {
    if let Some(token) = &stack.get_token() {
        let mut location = stack.get_location();
        let kind = SyntaxErrorKind::Unknown(UNKNOWN.into());

        if token != constants::SEMICOLON_TOKEN {
            while let Some(toke) = stack.next() {
                if token == constants::SEMICOLON_TOKEN {
                    stack.update_location_end(&mut location);
                    break;
                }
            }

            stack
                .syntax_error
                .push(SyntaxErrorDeclaration { location, kind });
        }
    }
}

pub fn declaration_report<T: Debug>(
    stack: &mut TokenReaderStack<T>,
    location: TokenReaderLocation,
    kind: &str,
) {
    if let Some(token) = &stack.get_token() {
        stack.syntax_error.push(SyntaxErrorDeclaration {
            location,
            kind: SyntaxErrorKind::Declaration(kind.into()),
        });
    }
}
