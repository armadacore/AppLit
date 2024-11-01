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
    Function(FunctionStatement)
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
            unknown_token => return Err(Cause::SyntaxError(AstError::UnexpectedToken(
                snapshot.clone(),
            ))),
        }));
    }
    
    Ok(None)
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::tokenizer::{TokenLocation, TokenSnapshot};
    use std::panic;

    #[test]
    fn parse_statement_peek_is_none_should_fail() {
        let mut builder = TreeBuilder::new(vec![]);

        match parse_statement(&mut builder) {
            Err(ErrorCause::SyntaxError(AstError::UnexpectedEOF)) => (),
            _ => panic!("Did not fail"),
        }
    }

    #[test]
    fn parse_statement_unknown_keyword_should_fail() {
        let location = TokenLocation::new(0, 0, 0);
        let snapshot = TokenSnapshot::new(location, "any_unknown_keyword_token".into());
        let mut builder = TreeBuilder::new(vec![TokenDeclaration::Keyword(snapshot)]);

        match parse_statement(&mut builder) {
            Err(ErrorCause::SyntaxError(AstError::UnexpectedToken(_))) => {}
            _ => panic!("Did not fail"),
        }
    }

    #[test]
    #[should_panic]
    fn parse_statement_missing_keyword_should_panic() {
        let location = TokenLocation::new(0, 0, 0);
        let snapshot = TokenSnapshot::new(location, "not_exist_toplevel_token".into());
        let mut builder = TreeBuilder::new(vec![TokenDeclaration::Literal(snapshot)]);

        parse_statement(&mut builder).expect("Did not panic from parse_statement self");
    }
}
*/