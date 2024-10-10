use crate::core::tokenizer::TokenSnapshot;

#[derive(Debug, PartialEq)]
pub enum AstNode {
    Program {
        statements: Vec<AstNode>,
    },
    Import {
        snapshot: TokenSnapshot,
        namespace: Option<TokenSnapshot>,
        identifiers: Vec<TokenSnapshot>,
        reference: TokenSnapshot,
    },
}

#[derive(Debug)]
pub enum AstError {
    UnexpectedToken(TokenSnapshot),
    UnexpectedError,
    UnexpectedEOF,
}