use crate::bin::constants;
use crate::core::feedback::error::ErrorCause;
use crate::core::parser::{import, AstError, AstNode, AstNodeModule, ModuleStatement, Parser};
use crate::core::tokenizer::TokenDeclaration;

pub fn parse<'a>(parser: &mut Parser) -> Result<AstNode, ErrorCause<'a>> {
    let mut statements = Vec::<ModuleStatement>::new();

    while parser.tokens.peek().is_some() {
        let statement = parse_statement(parser)?;
        statements.push(statement);
    }

    Ok(AstNode::Program(AstNodeModule::Statements(statements)))
}

fn parse_statement<'a>(parser: &mut Parser) -> Result<ModuleStatement, ErrorCause<'a>> {
    let peek = parser.tokens.peek();

    if peek.is_none() {
        return Err(ErrorCause::SyntaxError(AstError::UnexpectedEOF));
    }

    let peek = peek.unwrap();

    if let TokenDeclaration::Keyword(snapshot) = peek {
        return match snapshot.token.as_str() {
            constants::KEYWORD_IMPORT => Ok(ModuleStatement::Import(import::parse(parser)?)),
            unknown_token => Err(ErrorCause::SyntaxError(AstError::UnexpectedToken(
                snapshot.clone(),
            ))),
        };
    }

    panic!(
        "Try to parse on top level for unknown TokenDeclaration {:#?}",
        parser.tokens.peek().unwrap()
    );
}
