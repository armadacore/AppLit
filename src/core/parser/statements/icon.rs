use crate::core::feedback::ErrorCause;
use crate::core::tokenizer::{snapshot_error, TokenDeclaration, TokenSnapshot, Tokens};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IconCommitment {
    pub snapshot: TokenSnapshot,
    pub icon: TokenSnapshot,
}

pub fn parse_icon_commitment(tokens: &mut Tokens) -> Result<IconCommitment, ErrorCause> {
    let snapshot = tokens.next().unwrap().extract_snapshot();

    if let Some(TokenDeclaration::ArgumentOpen(_)) = tokens.next() {
        if let Some(TokenDeclaration::Literal(icon)) = tokens.next() {
            if let Some(TokenDeclaration::ArgumentClose(_)) = tokens.next() {
                if let Some(TokenDeclaration::StatementEnd(_)) = tokens.next() {
                    return Ok(IconCommitment { snapshot, icon });
                }
            }
        }
    }

    Err(snapshot_error(tokens.peek()))
}
