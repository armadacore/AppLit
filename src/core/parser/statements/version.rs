use crate::core::feedback::ErrorCause;
use crate::core::tokenizer::{snapshot_error, TokenDeclaration, TokenSnapshot, Tokens};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VersionCommitment {
    pub snapshot: TokenSnapshot,
    pub version: TokenSnapshot,
}

pub fn parse_version_commitment(tokens: &mut Tokens) -> Result<VersionCommitment, ErrorCause> {
    let snapshot = tokens.next().unwrap().extract_snapshot();

    if let Some(TokenDeclaration::ArgumentOpen(_)) = tokens.next() {
        if let Some(TokenDeclaration::Literal(version)) = tokens.next() {
            if let Some(TokenDeclaration::ArgumentClose(_)) = tokens.next() {
                if let Some(TokenDeclaration::StatementEnd(_)) = tokens.next() {
                    return Ok(VersionCommitment { snapshot, version });
                }
            }
        }
    }

    Err(snapshot_error(tokens.peek()))
}