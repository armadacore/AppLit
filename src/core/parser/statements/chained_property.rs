use crate::core::feedback::ErrorCause;
use crate::core::tokenizer::{snapshot_error, TokenDeclaration, TokenSnapshot, Tokens};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChainedProperties {
    pub chain: Vec<TokenSnapshot>
}

impl ChainedProperties {
    pub fn new(snapshots: Vec<TokenSnapshot>) -> Self {
        ChainedProperties { chain: snapshots }
    }
}

pub fn parse_chained_property(tokens: &mut Tokens) -> Result<ChainedProperties, ErrorCause> {
    let mut chain = Vec::<TokenSnapshot>::new();

    while tokens.peek().is_some() {
        if let Some(TokenDeclaration::Separator(_)) = tokens.peek() {
            tokens.next();
            continue
        }
        
        if let Some(TokenDeclaration::IndicesOpen(_)) = tokens.peek() {
            tokens.next();
            
            if let Some(TokenDeclaration::Literal(_)) = tokens.peek() {
                chain.push(tokens.next().unwrap().extract_snapshot());
            }
            
            if let Some(TokenDeclaration::IndicesClose(_)) = tokens.peek() {
                tokens.next();
                continue
            }
            
            return Err(snapshot_error(tokens.peek()));
        }
        
        if let Some(TokenDeclaration::Identifier(_)) = tokens.peek() {
            chain.push(tokens.next().unwrap().extract_snapshot());
            continue
        }

        return Ok(ChainedProperties { chain });
    }

    Err(snapshot_error(tokens.peek()))
}