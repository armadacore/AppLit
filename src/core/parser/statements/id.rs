use crate::core::feedback::ErrorCause;
use crate::core::parser::TreeBuilder;
use crate::core::tokenizer::{snapshot_error, TokenDeclaration, TokenSnapshot};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IdCommitment {
    pub snapshot: TokenSnapshot,
    pub developer_id: TokenSnapshot,
    pub application_id: TokenSnapshot,
}

pub fn parse_id_commitment(builder: &mut TreeBuilder) -> Result<IdCommitment, ErrorCause> {
    let snapshot = builder.tokens.next().unwrap().extract_snapshot();

    if let Some(TokenDeclaration::ArgumentOpen(_)) = builder.tokens.next() {
        if let Some(TokenDeclaration::Literal(developer_id)) = builder.tokens.next() {
            if let Some(TokenDeclaration::StatementDivider(_)) = builder.tokens.next() {
                if let Some(TokenDeclaration::Literal(application_id)) = builder.tokens.next() {
                    if let Some(TokenDeclaration::ArgumentClose(_)) = builder.tokens.next() {
                        if let Some(TokenDeclaration::StatementEnd(_)) = builder.tokens.next() {
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

    Err(snapshot_error(builder.tokens.peek()))
}