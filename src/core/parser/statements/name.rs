use serde::{Deserialize, Serialize};
use crate::core::feedback::ErrorCause;
use crate::core::parser::TreeBuilder;
use crate::core::tokenizer::{snapshot_error, TokenDeclaration, TokenSnapshot};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NameCommitment {
    pub snapshot: TokenSnapshot,
    pub name: TokenSnapshot,
}

pub fn parse_name_commitment(builder: &mut TreeBuilder) -> Result<NameCommitment, ErrorCause> {
    let snapshot = builder.tokens.next().unwrap().extract_snapshot();

    if let Some(TokenDeclaration::ArgumentOpen(_)) = builder.tokens.next() {
        if let Some(TokenDeclaration::Literal(name)) = builder.tokens.next() {
            if let Some(TokenDeclaration::ArgumentClose(_)) = builder.tokens.next() {
                if let Some(TokenDeclaration::StatementEnd(_)) = builder.tokens.next() {
                    return Ok(NameCommitment { snapshot, name });
                }
            }
        }
    }

    Err(snapshot_error(builder.tokens.peek()))
}