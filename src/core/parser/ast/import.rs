use crate::bin;
use crate::core::parser::ast::models::Parser;
use crate::core::parser::ast::{AstError, AstNode};
use crate::core::tokenizer::reader::{try_snapshot_error, TokenDeclaration, TokenSnapshot};

pub fn parse(parser: &mut Parser) -> Result<AstNode, AstError> {
    parser.tokens.next();

    let namespace = parse_namespace(parser)?;
    let identifiers = parse_identifiers(parser)?;
    let reference = parse_reference(parser)?;

    if let Some(TokenDeclaration::StatementEnd(_)) = parser.tokens.next() {
        Ok(AstNode::Import {
            namespace,
            identifiers,
            reference,
        })
    } else {
        Err(try_snapshot_error(parser.tokens.peek()))
    }
}

fn parse_namespace(parser: &mut Parser) -> Result<Option<TokenSnapshot>, AstError> {
    if let Some(TokenDeclaration::Identifier(name)) = parser.tokens.peek().cloned() {
        parser.tokens.next();

        if let Some(token) = parser.tokens.next() {
            if let TokenDeclaration::StatementAssignment(_) = token {
                Ok(Some(name))
            } else {
                Err(AstError::UnexpectedToken(token.extract_snapshot()))
            }
        } else {
            Err(AstError::UnexpectedError)
        }
    } else {
        Ok(None)
    }
}

fn parse_identifiers(parser: &mut Parser) -> Result<Vec<TokenSnapshot>, AstError> {
    let mut identifiers = Vec::<TokenSnapshot>::new();
    
    if let Some(TokenDeclaration::BlockOpen(_)) = parser.tokens.next() {
        loop {
            match parser.tokens.next() {
                Some(TokenDeclaration::Identifier(name)) => identifiers.push(name),
                Some(TokenDeclaration::StatementDivider(_)) => continue,
                Some(TokenDeclaration::BlockClose(_)) => break,
                _ => return Err(try_snapshot_error(parser.tokens.peek())),
            }
        }
    } else {
        return Err(try_snapshot_error(parser.tokens.peek()));
    }
    
    Ok(identifiers)
}

fn parse_reference(parser: &mut Parser) -> Result<TokenSnapshot, AstError> {
    if let Some(TokenDeclaration::Keyword(snapshot)) = parser.tokens.next() {
        if snapshot.token == bin::constants::KEYWORD_FROM {
            if let Some(TokenDeclaration::Literal(source)) = parser.tokens.next() {
                Ok(source)
            } else {
                Err(try_snapshot_error(parser.tokens.peek()))
            }
        } else {
            Err(try_snapshot_error(parser.tokens.peek()))
        }
    } else {
        Err(try_snapshot_error(parser.tokens.peek()))
    }
}
