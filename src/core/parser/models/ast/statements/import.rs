use crate::core::tokenizer::TokenSnapshot;

#[derive(Debug, PartialEq)]
pub struct ImportStatement {
    pub snapshot: TokenSnapshot,
    pub namespace: Option<TokenSnapshot>,
    pub identifiers: Vec<TokenSnapshot>,
    pub reference: TokenSnapshot,
}

