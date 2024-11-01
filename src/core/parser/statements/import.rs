use crate::bin::constants;
use crate::core::feedback::error::Cause;
use crate::core::parser::error::AstError;
use crate::core::tokenizer::entities::declaration::TokenDeclaration;
use crate::core::tokenizer::entities::snapshot::TokenSnapshot;
use crate::core::tokenizer::lib::error_conversion::snapshot_error;
use crate::core::tokenizer::Tokens;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportStatement {
    pub snapshot: TokenSnapshot,
    pub namespace: Option<TokenSnapshot>,
    pub identifiers: Vec<TokenSnapshot>,
    pub reference: TokenSnapshot,
}

pub fn parse_import_statement(tokens: &mut Tokens) -> Result<ImportStatement, Cause> {
    let snapshot = tokens.next().unwrap().extract_snapshot();
    let namespace = parse_namespace(tokens)?;
    let identifiers = parse_identifiers(tokens)?;
    let reference = parse_reference(tokens)?;

    if let Some(TokenDeclaration::StatementEnd(_)) = tokens.next() {
        Ok(ImportStatement {
            snapshot,
            namespace,
            identifiers,
            reference,
        })
    } else {
        Err(snapshot_error(tokens.peek()))
    }
}

fn parse_namespace(tokens: &mut Tokens) -> Result<Option<TokenSnapshot>, Cause> {
    if let Some(TokenDeclaration::Identifier(name)) = tokens.peek().cloned() {
        tokens.next();

        if let Some(token) = tokens.next() {
            if let TokenDeclaration::StatementAssignment(_) = token {
                Ok(Some(name))
            } else {
                Err(Cause::SyntaxError(AstError::UnexpectedToken(token.extract_snapshot())))
            }
        } else {
            Err(Cause::SyntaxError(AstError::UnexpectedError(None)))
        }
    } else {
        Ok(None)
    }
}

fn parse_identifiers(tokens: &mut Tokens) -> Result<Vec<TokenSnapshot>, Cause> {
    let mut identifiers = Vec::<TokenSnapshot>::new();

    if let Some(TokenDeclaration::BlockOpen(_)) = tokens.next() {
        loop {
            match tokens.next() {
                Some(TokenDeclaration::Identifier(name)) => identifiers.push(name),
                Some(TokenDeclaration::StatementDivider(_)) => continue,
                Some(TokenDeclaration::BlockClose(_)) => break,
                _ => return Err(snapshot_error(tokens.peek())),
            }
        }
    } else {
        return Err(snapshot_error(tokens.peek()));
    }

    Ok(identifiers)
}

fn parse_reference(tokens: &mut Tokens) -> Result<TokenSnapshot, Cause> {
    if let Some(TokenDeclaration::Keyword(snapshot)) = tokens.next() {
        if snapshot.token == constants::KEYWORD_FROM {
            if let Some(TokenDeclaration::Literal(source)) = tokens.next() {
                Ok(source)
            } else {
                Err(snapshot_error(tokens.peek()))
            }
        } else {
            Err(snapshot_error(tokens.peek()))
        }
    } else {
        Err(snapshot_error(tokens.peek()))
    }
}
