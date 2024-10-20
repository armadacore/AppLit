use crate::core::tokenizer::TokenSnapshot;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportStatement {
    pub snapshot: TokenSnapshot,
    pub namespace: Option<TokenSnapshot>,
    pub identifiers: Vec<TokenSnapshot>,
    pub reference: TokenSnapshot,
}

