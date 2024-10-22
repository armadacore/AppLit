use crate::core::feedback::ErrorCause;
use crate::core::parser::statements::chained_property::{parse_chained_property, ChainedProperties};
use crate::core::parser::statements::object_declaration::{parse_object_declaration, ObjectDeclaration};
use crate::core::parser::TreeBuilder;
use crate::core::tokenizer::{snapshot_error, TokenDeclaration, TokenSnapshot};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DomainCommitment {
    pub snapshot: TokenSnapshot,
    pub default: ChainedProperties,
    pub distribution: Option<ObjectDeclaration>,
}

pub fn parse_domain_commitment(builder: &mut TreeBuilder) -> Result<DomainCommitment, ErrorCause> {
    let snapshot = builder.tokens.next().unwrap().extract_snapshot();
    
    if let Some(TokenDeclaration::ArgumentOpen(_)) = builder.tokens.next() {
        let default = parse_chained_property(builder)?;

        if domain_parser_end(builder) {
            return Ok(DomainCommitment {
                snapshot,
                default,
                distribution: None
            });
        }

        if let Some(TokenDeclaration::StatementDivider(_)) = builder.tokens.next() {
            let distribution = Some(parse_object_declaration(builder)?);

            if domain_parser_end(builder) {
                return Ok(DomainCommitment {
                    snapshot,
                    default,
                    distribution
                });
            }
        }
    }
    
    Err(snapshot_error(builder.tokens.peek()))
}

fn domain_parser_end(builder: &mut TreeBuilder) -> bool {
    if let Some(TokenDeclaration::ArgumentClose(_)) = builder.tokens.peek() {
        builder.tokens.next();

        if let Some(TokenDeclaration::StatementEnd(_)) = builder.tokens.peek() {
            builder.tokens.next();

            return true;
        }
    }

    false
}