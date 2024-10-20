use crate::bin::constants;
use crate::core::feedback::ErrorCause;
use crate::core::parser::{import, AstError, AstModuleNode, AstNode, TreeBuilder};
use crate::core::tokenizer::TokenDeclaration;

pub fn parse(parser: &mut TreeBuilder) -> Result<AstNode, ErrorCause> {
    let mut statements = Vec::<AstModuleNode>::new();

    while parser.tokens.peek().is_some() {
        let statement = parse_statement(parser)?;
        statements.push(statement);
    }

    Ok(AstNode::Module(AstModuleNode::Statements(statements)))
}

fn parse_statement(builder: &mut TreeBuilder) -> Result<AstModuleNode, ErrorCause> {
    let peek = builder.tokens.peek();

    if peek.is_none() {
        return Err(ErrorCause::SyntaxError(AstError::UnexpectedEOF));
    }

    let peek = peek.unwrap();

    if let TokenDeclaration::Keyword(snapshot) = peek {
        return match snapshot.token.as_str() {
            constants::KEYWORD_IMPORT => Ok(AstModuleNode::Import(import::parse(builder)?)),
            unknown_token => Err(ErrorCause::SyntaxError(AstError::UnexpectedToken(
                snapshot.clone(),
            ))),
        };
    }

    panic!(
        "Try to parse on top level for unknown TokenDeclaration {:#?}",
        builder.tokens.peek().unwrap()
    );
}

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
