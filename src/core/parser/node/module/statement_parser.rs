use crate::bin::constants;
use crate::core::feedback::error::Cause;
use crate::core::parser::error::AstError;
use crate::core::parser::node::AstNode;
use crate::core::parser::statements::function::FunctionStatement;
use crate::core::parser::statements::import::{parse_import_statement, ImportStatement};
use crate::core::tokenizer::entities::declaration::TokenDeclaration;
use crate::core::tokenizer::Tokens;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AstModuleNode {
    Statements(Vec<AstModuleNode>),
    Import(ImportStatement),
    Function(FunctionStatement),
}

pub fn parse_module_statements(tokens: &mut Tokens) -> Result<AstNode, Cause> {
    let mut statements: Vec<AstModuleNode> = Vec::new();

    while tokens.peek().is_some() {
        if let Some(keywords) = parse_keywords(tokens)? {
            statements.push(keywords);
            continue;
        }

        panic!(
            "Try to parse on module top level for unknown TokenDeclaration {:#?}",
            tokens.peek().unwrap()
        );
    }

    Ok(AstNode::Module(AstModuleNode::Statements(statements)))
}

fn parse_keywords(tokens: &mut Tokens) -> Result<Option<AstModuleNode>, Cause> {
    if let Some(TokenDeclaration::Keyword(snapshot)) = tokens.peek() {
        return Ok(Some(match snapshot.token.as_str() {
            constants::KEYWORD_IMPORT => AstModuleNode::Import(parse_import_statement(tokens)?),
            constants::KEYWORD_FUNCTION => todo!(),
            unknown_token => return Err(Cause::SyntaxError(AstError::UnexpectedToken(snapshot.clone()))),
        }));
    }

    Ok(None)
}
