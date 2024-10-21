use crate::bin::constants;
use crate::core::feedback::ErrorCause;
use crate::core::parser::statements::icon::{parse_icon_commitment, IconCommitment};
use crate::core::parser::{parse_id_commitment, parse_import_statement, AstError, IdCommitment, ImportStatement, TreeBuilder};
use crate::core::tokenizer::TokenDeclaration;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AstMainNode {
    Statements(Vec<AstMainNode>),
    Import(ImportStatement),
    Id(IdCommitment),
    Icon(IconCommitment),
    Name(String),
    Version(String),
    Description(String),
    Link(String),
    Packages(String),
    Domain{
        default: String,
        distribution: HashMap<String, String>,
    }
}

pub fn parse_statement(builder: &mut TreeBuilder) -> Result<AstMainNode, ErrorCause> {
    let peek = builder.tokens.peek();

    if peek.is_none() {
        return Err(ErrorCause::SyntaxError(AstError::UnexpectedEOF));
    }

    let peek = peek.unwrap();

    if let TokenDeclaration::Keyword(snapshot) = peek {
        return match snapshot.token.as_str() {
            constants::KEYWORD_IMPORT => Ok(AstMainNode::Import(parse_import_statement(builder)?)),
            unknown_token => Err(ErrorCause::SyntaxError(AstError::UnexpectedToken(
                snapshot.clone(),
            ))),
        };
    }

    if let TokenDeclaration::Commitment(snapshot) = peek {
        return match snapshot.token.as_str() {
            constants::COMMITMENT_ID => Ok(AstMainNode::Id(parse_id_commitment(builder)?)),
            constants::COMMITMENT_ICON => Ok(AstMainNode::Icon(parse_icon_commitment(builder)?)),
            unknown_token => Err(ErrorCause::SyntaxError(AstError::UnexpectedToken(
                snapshot.clone(),
            ))),
        };
    }

    panic!(
        "Try to parse on main top level for unknown TokenDeclaration {:#?}",
        builder.tokens.peek().unwrap()
    );
}