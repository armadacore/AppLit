use crate::core::feedback::ErrorCause;
use crate::core::tokenizer::{snapshot_error, TokenDeclaration, TokenSnapshot, Tokens};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NameCommitment {
    pub snapshot: TokenSnapshot,
    pub name: TokenSnapshot,
}

pub fn parse_name_commitment(tokens: &mut Tokens) -> Result<NameCommitment, ErrorCause> {
    let snapshot = tokens.next().unwrap().extract_snapshot();

    if let Some(TokenDeclaration::ArgumentOpen(_)) = tokens.next() {
        if let Some(TokenDeclaration::Literal(name)) = tokens.next() {
            if let Some(TokenDeclaration::ArgumentClose(_)) = tokens.next() {
                if let Some(TokenDeclaration::StatementEnd(_)) = tokens.next() {
                    return Ok(NameCommitment { snapshot, name });
                }
            }
        }
    }

    Err(snapshot_error(tokens.peek()))
}