use crate::core::tokenizer::TokenSnapshot;

#[derive(Debug)]
pub enum AstNode {
    Program {
        statements: Vec<AstNode>,
    },
    Import {
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