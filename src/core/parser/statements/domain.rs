use crate::core::feedback::ErrorCause;
use crate::core::parser::TreeBuilder;
use crate::core::tokenizer::{snapshot_error, TokenDeclaration, TokenSnapshot};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DomainCommitment {
    pub snapshot: TokenSnapshot,
    pub default: TokenSnapshot,
    pub distribution: Option<HashMap<String, TokenSnapshot>>,
}

pub fn parse_domain_commitment(builder: &mut TreeBuilder) -> Result<DomainCommitment, ErrorCause> {
    let snapshot = builder.tokens.next().unwrap().extract_snapshot();
    
    if let Some(TokenDeclaration::ArgumentOpen(_)) = builder.tokens.next() {
        
    }

    Err(snapshot_error(builder.tokens.peek()))
}
