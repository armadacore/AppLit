use crate::core::feedback::error::Cause;
use crate::core::parser::statements::chained_property::{parse_chained_property, ChainedProperties};
use crate::core::parser::statements::object_declaration::{parse_object_declaration, ObjectDeclaration};
use crate::core::tokenizer::entities::declaration::TokenDeclaration;
use crate::core::tokenizer::entities::snapshot::TokenSnapshot;
use crate::core::tokenizer::lib::error_conversion::snapshot_error;
use crate::core::tokenizer::Tokens;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DomainCommitment {
    pub snapshot: TokenSnapshot,
    pub default: ChainedProperties,
    pub distribution: Option<ObjectDeclaration>,
}

pub fn parse_domain_commitment(tokens: &mut Tokens) -> Result<DomainCommitment, Cause> {
    let snapshot = tokens.next().unwrap().extract_snapshot();
    
    if let Some(TokenDeclaration::ArgumentOpen(_)) = tokens.next() {
        let default = parse_chained_property(tokens)?;

        if domain_parser_end(tokens) {
            return Ok(DomainCommitment {
                snapshot,
                default,
                distribution: None
            });
        }

        if let Some(TokenDeclaration::StatementDivider(_)) = tokens.next() {
            let distribution = Some(parse_object_declaration(tokens)?);

            if domain_parser_end(tokens) {
                return Ok(DomainCommitment {
                    snapshot,
                    default,
                    distribution
                });
            }
        }
    }
    
    Err(snapshot_error(tokens.peek()))
}

fn domain_parser_end(tokens: &mut Tokens) -> bool {
    if let Some(TokenDeclaration::ArgumentClose(_)) = tokens.peek() {
        tokens.next();

        if let Some(TokenDeclaration::StatementEnd(_)) = tokens.peek() {
            tokens.next();

            return true;
        }
    }

    false
}