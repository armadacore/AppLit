use crate::core::feedback::ErrorCause;
use crate::core::parser::TreeBuilder;
use crate::core::tokenizer::{snapshot_error, TokenDeclaration, TokenSnapshot};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LinkCommitment {
    pub snapshot: TokenSnapshot,
    pub link: TokenSnapshot,
}

pub fn parse_link_commitment(builder: &mut TreeBuilder) -> Result<LinkCommitment, ErrorCause> {
    let snapshot = builder.tokens.next().unwrap().extract_snapshot();

    if let Some(TokenDeclaration::ArgumentOpen(_)) = builder.tokens.next() {
        if let Some(TokenDeclaration::Literal(link)) = builder.tokens.next() {
            if let Some(TokenDeclaration::ArgumentClose(_)) = builder.tokens.next() {
                if let Some(TokenDeclaration::StatementEnd(_)) = builder.tokens.next() {
                    return Ok(LinkCommitment { snapshot, link });
                }
            }
        }
    }

    Err(snapshot_error(builder.tokens.peek()))
}