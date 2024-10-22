use crate::core::feedback::ErrorCause;
use crate::core::parser::TreeBuilder;
use crate::core::tokenizer::{snapshot_error, TokenDeclaration, TokenSnapshot};
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

pub fn parse_chained_property(builder: &mut TreeBuilder) -> Result<ChainedProperties, ErrorCause> {
    let mut chain = Vec::<TokenSnapshot>::new();

    while builder.tokens.peek().is_some() {
        if let Some(TokenDeclaration::Identifier(_)) = builder.tokens.peek() {
            chain.push(builder.tokens.next().unwrap().extract_snapshot());
        }

        if let Some(TokenDeclaration::Separator(_)) = builder.tokens.peek() {
            builder.tokens.next();
            continue
        }

        return Ok(ChainedProperties { chain });
    }

    Err(snapshot_error(builder.tokens.peek()))
}