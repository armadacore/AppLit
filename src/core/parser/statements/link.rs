use crate::core::feedback::ErrorCause;
use crate::core::tokenizer::{snapshot_error, TokenDeclaration, TokenSnapshot, Tokens};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LinkCommitment {
    pub snapshot: TokenSnapshot,
    pub link: TokenSnapshot,
}

pub fn parse_link_commitment(tokens: &mut Tokens) -> Result<LinkCommitment, ErrorCause> {
    let snapshot = tokens.next().unwrap().extract_snapshot();

    if let Some(TokenDeclaration::ArgumentOpen(_)) = tokens.next() {
        if let Some(TokenDeclaration::Literal(link)) = tokens.next() {
            if let Some(TokenDeclaration::ArgumentClose(_)) = tokens.next() {
                if let Some(TokenDeclaration::StatementEnd(_)) = tokens.next() {
                    return Ok(LinkCommitment { snapshot, link });
                }
            }
        }
    }

    Err(snapshot_error(tokens.peek()))
}