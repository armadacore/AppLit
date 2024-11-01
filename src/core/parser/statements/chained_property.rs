use crate::core::feedback::error::Cause;
use crate::core::tokenizer::entities::declaration::TokenDeclaration;
use crate::core::tokenizer::entities::snapshot::TokenSnapshot;
use crate::core::tokenizer::lib::error_conversion::snapshot_error;
use crate::core::tokenizer::Tokens;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChainedProperties {
    pub chain: Vec<TokenSnapshot>,
}

impl ChainedProperties {
    pub fn new(snapshots: Vec<TokenSnapshot>) -> Self {
        ChainedProperties { chain: snapshots }
    }
}

pub fn parse_chained_property(tokens: &mut Tokens) -> Result<ChainedProperties, Cause> {
    let mut chain = Vec::<TokenSnapshot>::new();

    while tokens.peek().is_some() {
        if let Some(TokenDeclaration::Separator(_)) = tokens.peek() {
            tokens.next();
            continue;
        }

        if let Some(TokenDeclaration::IndicesOpen(_)) = tokens.peek() {
            tokens.next();

            if let Some(TokenDeclaration::Literal(_)) = tokens.peek() {
                chain.push(tokens.next().unwrap().extract_snapshot());
            }

            if let Some(TokenDeclaration::IndicesClose(_)) = tokens.peek() {
                tokens.next();
                continue;
            }

            return Err(snapshot_error(tokens.peek()));
        }

        if let Some(TokenDeclaration::Identifier(_)) = tokens.peek() {
            chain.push(tokens.next().unwrap().extract_snapshot());
            continue;
        }

        return Ok(ChainedProperties { chain });
    }

    Err(snapshot_error(tokens.peek()))
}
