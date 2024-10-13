use crate::core::tokenizer::TokenSnapshot;

#[derive(Debug, PartialEq)]
pub struct FunctionStatement {
    pub snapshot: TokenSnapshot,
    pub identifier: TokenSnapshot,
    pub arguments: Vec<String>,
    pub body: Vec<String>,
    pub result: Option<String>,
}