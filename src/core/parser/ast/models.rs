use std::iter::Peekable;
use std::vec::IntoIter;
use crate::core::tokenizer::reader::{TokenDeclaration, TokenSnapshot};

pub enum AstNode{
    Program {
        statements: Vec<AstNode>
    },
    Import {
        namespace: Option<TokenSnapshot>,
        specifiers: Vec<TokenSnapshot>,
        reference: TokenSnapshot,
    },
}

pub enum AstError {
    UnexpectedToken(TokenSnapshot),
    UnexpectedEOF,
}

pub struct Parser {
    pub tokens: Peekable<IntoIter<TokenDeclaration>>,
}