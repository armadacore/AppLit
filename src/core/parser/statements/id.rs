use crate::core::feedback::error::Cause;
use crate::core::tokenizer::{snapshot_error, TokenDeclaration, TokenSnapshot, Tokens};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IdCommitment {
    pub snapshot: TokenSnapshot,
    pub developer_id: TokenSnapshot,
    pub application_id: TokenSnapshot,
}

pub fn parse_id_commitment(tokens: &mut Tokens) -> Result<IdCommitment, Cause> {
    let snapshot = tokens.next().unwrap().extract_snapshot();

    if let Some(TokenDeclaration::ArgumentOpen(_)) = tokens.next() {
        if let Some(TokenDeclaration::Literal(developer_id)) = tokens.next() {
            if let Some(TokenDeclaration::StatementDivider(_)) = tokens.next() {
                if let Some(TokenDeclaration::Literal(application_id)) = tokens.next() {
                    if let Some(TokenDeclaration::ArgumentClose(_)) = tokens.next() {
                        if let Some(TokenDeclaration::StatementEnd(_)) = tokens.next() {
                            return Ok(IdCommitment {
                                snapshot,
                                developer_id,
                                application_id
                            })
                        }
                    }
                }
            }
        }
    }

    Err(snapshot_error(tokens.peek()))
}