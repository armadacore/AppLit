use crate::bin::constants;
use crate::core::feedback::ErrorCause;
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
use crate::core::parser::{AstError, TreeBuilder};
use crate::core::tokenizer::TokenDeclaration;
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
            constants::COMMITMENT_NAME => Ok(AstMainNode::Name(parse_name_commitment(builder)?)),
            constants::COMMITMENT_VERSION => {
                Ok(AstMainNode::Version(parse_version_commitment(builder)?))
            }
            constants::COMMITMENT_DESCRIPTION => Ok(AstMainNode::Description(
                parse_description_commitment(builder)?,
            )),
            constants::COMMITMENT_LINK => Ok(AstMainNode::Link(parse_link_commitment(builder)?)),
            constants::COMMITMENT_DOMAIN => {
                Ok(AstMainNode::Domain(parse_domain_commitment(builder)?))
            }
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
