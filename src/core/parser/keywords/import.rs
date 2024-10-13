use crate::bin::constants;
use crate::core::feedback::ErrorCause;
use crate::core::parser::{AstError, ImportStatement, TreeBuilder};
use crate::core::tokenizer::{try_snapshot_error, TokenDeclaration, TokenSnapshot};

pub fn parse<'a>(parser: &mut TreeBuilder) -> Result<ImportStatement, ErrorCause<'a>> {
    let snapshot = parser.tokens.peek().unwrap().extract_snapshot();
    parser.tokens.next();

    let namespace = parse_namespace(parser)?;
    let identifiers = parse_identifiers(parser)?;
    let reference = parse_reference(parser)?;

    if let Some(TokenDeclaration::StatementEnd(_)) = parser.tokens.next() {
        Ok(ImportStatement{
            snapshot,
            namespace,
            identifiers,
            reference,
        })
    } else {
        Err(try_snapshot_error(parser.tokens.peek()))
    }
}

fn parse_namespace<'a>(parser: &mut TreeBuilder) -> Result<Option<TokenSnapshot>, ErrorCause<'a>> {
    if let Some(TokenDeclaration::Identifier(name)) = parser.tokens.peek().cloned() {
        parser.tokens.next();

        if let Some(token) = parser.tokens.next() {
            if let TokenDeclaration::StatementAssignment(_) = token {
                Ok(Some(name))
            } else {
                Err(ErrorCause::SyntaxError(AstError::UnexpectedToken(
                    token.extract_snapshot(),
                )))
            }
        } else {
            Err(ErrorCause::SyntaxError(AstError::UnexpectedError(None)))
        }
    } else {
        Ok(None)
    }
}

fn parse_identifiers<'a>(parser: &mut TreeBuilder) -> Result<Vec<TokenSnapshot>, ErrorCause<'a>> {
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

fn parse_reference<'a>(parser: &mut TreeBuilder) -> Result<TokenSnapshot, ErrorCause<'a>> {
    if let Some(TokenDeclaration::Keyword(snapshot)) = parser.tokens.next() {
        if snapshot.token == constants::KEYWORD_FROM {
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

