use crate::bin;
use crate::core::parser::ast::models::Parser;
use crate::core::parser::ast::{AstError, AstNode};
use crate::core::tokenizer::reader::{TokenDeclaration, TokenSnapshot};

pub fn parse(parser: &mut Parser) -> Result<AstNode, AstError> {
    // consume import
    parser.tokens.next();

    // consume namespace
    let namespace = if let Some(TokenDeclaration::Identifier(name)) = parser.tokens.peek().cloned()
    {
        parser.tokens.next();

        if let Some(token) = parser.tokens.next() {
            if let TokenDeclaration::StatementAssignment(_) = token {
                Some(name)
            } else {
                return Err(AstError::UnexpectedToken(token.extract_snapshot()));
            }
        } else {
            return Err(AstError::UnexpectedError);
        }
    } else {
        None
    };

    // consume identifiers
    let mut identifiers = Vec::<TokenSnapshot>::new();
    if let Some(TokenDeclaration::BlockOpen(_)) = parser.tokens.next() {
        loop {
            match parser.tokens.next() {
                Some(TokenDeclaration::Identifier(name)) => identifiers.push(name),
                Some(TokenDeclaration::StatementDivider(_)) => continue,
                Some(TokenDeclaration::BlockClose(_)) => break,
                _ => return Err(try_snapshot_token_error(parser.tokens.peek())),
            }
        }
    } else {
        return Err(try_snapshot_token_error(parser.tokens.peek()));
    }

    // consume from
    let reference = if let Some(TokenDeclaration::Keyword(snapshot)) = parser.tokens.next() {
        if snapshot.token == bin::constants::KEYWORD_FROM {
            if let Some(TokenDeclaration::Literal(source)) = parser.tokens.next() {
                Some(source)
            } else {
                return Err(try_snapshot_token_error(parser.tokens.peek()));
            }
        } else {
            return Err(try_snapshot_token_error(parser.tokens.peek()));
        }
    } else {
        return Err(try_snapshot_token_error(parser.tokens.peek()));
    };

    // return Result
    if let Some(TokenDeclaration::StatementEnd(_)) = parser.tokens.next() {
        Ok(AstNode::Import {
            namespace,
            identifiers,
            reference: reference.unwrap(),
        })
    } else {
        Err(try_snapshot_token_error(parser.tokens.peek()))
    }
}

fn try_snapshot_token_error(token_declaration: Option<&TokenDeclaration>) -> AstError {
    if let Some(token_declaration) = token_declaration {
        return AstError::UnexpectedToken(token_declaration.clone().extract_snapshot());
    }

    AstError::UnexpectedError
}
