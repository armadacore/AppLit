use crate::core::tokenizer::entities::snapshot::TokenSnapshot;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionStatement {
    pub snapshot: TokenSnapshot,
    pub identifier: TokenSnapshot,
    pub arguments: Vec<String>,
    pub body: Vec<String>,
    pub result: Option<String>,
}
