use crate::core::feedback::ErrorCause;
use crate::core::parser::TreeBuilder;
use crate::core::tokenizer::{snapshot_error, TokenDeclaration, TokenSnapshot};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VersionCommitment {
    pub snapshot: TokenSnapshot,
    pub version: TokenSnapshot,
}

pub fn parse_version_commitment(builder: &mut TreeBuilder) -> Result<VersionCommitment, ErrorCause> {
    let snapshot = builder.tokens.next().unwrap().extract_snapshot();

    if let Some(TokenDeclaration::ArgumentOpen(_)) = builder.tokens.next() {
        if let Some(TokenDeclaration::Literal(version)) = builder.tokens.next() {
            if let Some(TokenDeclaration::ArgumentClose(_)) = builder.tokens.next() {
                if let Some(TokenDeclaration::StatementEnd(_)) = builder.tokens.next() {
                    return Ok(VersionCommitment { snapshot, version });
                }
            }
        }
    }

    Err(snapshot_error(builder.tokens.peek()))
}