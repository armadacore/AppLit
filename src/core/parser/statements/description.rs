use crate::core::feedback::error::Cause;
use crate::core::tokenizer::Tokens;
use serde::{Deserialize, Serialize};
use crate::core::tokenizer::entities::declaration::TokenDeclaration;
use crate::core::tokenizer::entities::snapshot::TokenSnapshot;
use crate::core::tokenizer::lib::error_conversion::snapshot_error;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DescriptionCommitment {
    pub snapshot: TokenSnapshot,
    pub description: TokenSnapshot,
}

pub fn parse_description_commitment(tokens: &mut Tokens) -> Result<DescriptionCommitment, Cause> {
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