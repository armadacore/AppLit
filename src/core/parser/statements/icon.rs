use crate::core::feedback::ErrorCause;
use crate::core::parser::TreeBuilder;
use crate::core::tokenizer::{snapshot_error, TokenDeclaration, TokenSnapshot};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IconCommitment {
    pub snapshot: TokenSnapshot,
    pub icon: TokenSnapshot,
}

pub fn parse_icon_commitment(builder: &mut TreeBuilder) -> Result<IconCommitment, ErrorCause> {
    let snapshot = builder.tokens.peek().unwrap().extract_snapshot();
    builder.tokens.next();

    if let Some(TokenDeclaration::ArgumentOpen(_)) = builder.tokens.next() {
        if let Some(TokenDeclaration::Literal(icon)) = builder.tokens.next() {
            if let Some(TokenDeclaration::ArgumentClose(_)) = builder.tokens.next() {
                if let Some(TokenDeclaration::StatementEnd(_)) = builder.tokens.next() {
                    return Ok(IconCommitment { snapshot, icon });
                }
            }
        }
    }

    Err(snapshot_error(builder.tokens.peek()))
}
