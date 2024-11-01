use crate::bin::constants;
use crate::core::feedback::error::Cause;
use crate::core::parser::error::AstError;
use crate::core::parser::node::AstNode;
use crate::core::parser::statements::description::{
    parse_description_commitment, DescriptionCommitment,
};
use crate::core::parser::statements::domain::{parse_domain_commitment, DomainCommitment};
use crate::core::parser::statements::icon::{parse_icon_commitment, IconCommitment};
use crate::core::parser::statements::id::{parse_id_commitment, IdCommitment};
use crate::core::parser::statements::import::{parse_import_statement, ImportStatement};
use crate::core::parser::statements::link::{parse_link_commitment, LinkCommitment};
use crate::core::parser::statements::name::{parse_name_commitment, NameCommitment};
use crate::core::parser::statements::version::{parse_version_commitment, VersionCommitment};
use crate::core::tokenizer::entities::declaration::TokenDeclaration;
use crate::core::tokenizer::Tokens;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AstMainNode {
    Statements(Vec<AstMainNode>),
    Import(ImportStatement),
    Id(IdCommitment),
    Icon(IconCommitment),
    Name(NameCommitment),
    Version(VersionCommitment),
    Description(DescriptionCommitment),
    Link(LinkCommitment),
    // TODO support third party packages
    Dependencies(()),
    // TODO support enable different options like network etc
    Permissions(()),
    Domain(DomainCommitment),
}

pub fn parse_main_statements(tokens: &mut Tokens) -> Result<AstNode, Cause>{
    let mut statements = Vec::<AstMainNode>::new();

    while tokens.peek().is_some() {
        if let Some(keywords) = parse_keywords(tokens)? {
            statements.push(keywords);
            continue;
        }

        if let Some(commitments) = parse_commitments(tokens)? {
            statements.push(commitments);
            continue;
        }

        panic!(
            "Try to parse on main top level for unknown TokenDeclaration {:#?}",
            tokens.peek().unwrap()
        );
    }

    Ok(AstNode::Main(AstMainNode::Statements(statements)))
}

fn parse_keywords(tokens: &mut Tokens) -> Result<Option<AstMainNode>, Cause> {
    if let Some(TokenDeclaration::Keyword(snapshot)) = tokens.peek() {
        return Ok(Some(match snapshot.token.as_str() {
            constants::KEYWORD_IMPORT => AstMainNode::Import(parse_import_statement(tokens)?),
            unknown_token => return Err(Cause::SyntaxError(AstError::UnexpectedToken(
                snapshot.clone(),
            ))),
        }));
    }

    Ok(None)
}

fn parse_commitments(tokens: &mut Tokens) -> Result<Option<AstMainNode>, Cause> {
    if let Some(TokenDeclaration::Commitment(snapshot)) = tokens.peek() {
        return Ok(Some(match snapshot.token.as_str() {
            constants::COMMITMENT_ID => AstMainNode::Id(parse_id_commitment(tokens)?),
            constants::COMMITMENT_ICON => AstMainNode::Icon(parse_icon_commitment(tokens)?),
            constants::COMMITMENT_NAME => AstMainNode::Name(parse_name_commitment(tokens)?),
            constants::COMMITMENT_VERSION => AstMainNode::Version(parse_version_commitment(tokens)?),
            constants::COMMITMENT_DESCRIPTION => AstMainNode::Description(parse_description_commitment(tokens)?),
            constants::COMMITMENT_LINK => AstMainNode::Link(parse_link_commitment(tokens)?),
            constants::COMMITMENT_DOMAIN => AstMainNode::Domain(parse_domain_commitment(tokens)?),
            unknown_token =>  return Err(Cause::SyntaxError(AstError::UnexpectedToken(
                snapshot.clone(),
            ))),
        }));
    }

    Ok(None)
}
