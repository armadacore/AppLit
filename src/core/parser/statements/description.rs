use crate::core::feedback::ErrorCause;
use crate::core::tokenizer::{snapshot_error, TokenDeclaration, TokenSnapshot, Tokens};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DescriptionCommitment {
    pub snapshot: TokenSnapshot,
    pub description: TokenSnapshot,
}

pub fn parse_description_commitment(tokens: &mut Tokens) -> Result<DescriptionCommitment, ErrorCause> {
    let snapshot = tokens.next().unwrap().extract_snapshot();

    if let Some(TokenDeclaration::ArgumentOpen(_)) = tokens.next() {
        if let Some(TokenDeclaration::Literal(description)) = tokens.next() {
            if let Some(TokenDeclaration::ArgumentClose(_)) = tokens.next() {
                if let Some(TokenDeclaration::StatementEnd(_)) = tokens.next() {
                    return Ok(DescriptionCommitment { snapshot, description });
                }
            }
        }
    }

    Err(snapshot_error(tokens.peek()))
}