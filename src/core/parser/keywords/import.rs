use crate::bin;
use crate::core::parser::{AstError, AstNode, Parser};
use crate::core::tokenizer::{try_snapshot_error, TokenDeclaration, TokenSnapshot};

pub fn parse(parser: &mut Parser) -> Result<AstNode, AstError> {
    let snapshot = parser.tokens.peek().unwrap().extract_snapshot();
    parser.tokens.next();

    let namespace = parse_namespace(parser)?;
    let identifiers = parse_identifiers(parser)?;
    let reference = parse_reference(parser)?;

    if let Some(TokenDeclaration::StatementEnd(_)) = parser.tokens.next() {
        Ok(AstNode::Import {
            snapshot,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::parser::tests::create_parsed_tokens;
    use crate::core::tokenizer::TokenLocation;

    #[test]
    fn statement_without_namespace_is_valid() {
        let expected_import_ast = AstNode::Program {
            statements: vec![AstNode::Import {
                snapshot: TokenSnapshot {
                    location: TokenLocation {
                        start: 0,
                        end: 6,
                        line: 1,
                    },
                    token: "import".into(),
                },
                namespace: None,
                identifiers: vec![
                    TokenSnapshot {
                        location: TokenLocation {
                            start: 8,
                            end: 10,
                            line: 1,
                        },
                        token: "pi".into(),
                    },
                    TokenSnapshot {
                        location: TokenLocation {
                            start: 11,
                            end: 13,
                            line: 1,
                        },
                        token: "co".into(),
                    },
                ],
                reference: TokenSnapshot {
                    location: TokenLocation {
                        start: 20,
                        end: 28,
                        line: 1,
                    },
                    token: "'applit'".into(),
                },
            }],
        };
        let data = "import {pi,co} from 'applit';";
        let parsed_tokens = create_parsed_tokens(data);

        assert_eq!(parsed_tokens, expected_import_ast);
    }

    #[test]
    fn statement_with_namespace_is_valid() {
        let expected_import_ast = AstNode::Program {
            statements: vec![AstNode::Import {
                snapshot: TokenSnapshot {
                    location: TokenLocation {
                        start: 0,
                        end: 6,
                        line: 1,
                    },
                    token: "import".into(),
                },
                namespace: Some(TokenSnapshot {
                    location: TokenLocation {
                        start: 7,
                        end: 13,
                        line: 1,
                    },
                    token: "foobar".into(),
                }),
                identifiers: vec![
                    TokenSnapshot {
                        location: TokenLocation {
                            start: 15,
                            end: 17,
                            line: 1,
                        },
                        token: "pi".into(),
                    },
                    TokenSnapshot {
                        location: TokenLocation {
                            start: 18,
                            end: 20,
                            line: 1,
                        },
                        token: "co".into(),
                    },
                ],
                reference: TokenSnapshot {
                    location: TokenLocation {
                        start: 27,
                        end: 35,
                        line: 1,
                    },
                    token: "'applit'".into(),
                },
            }],
        };
        let data = "import foobar:{pi,co} from 'applit';";
        let parsed_tokens = create_parsed_tokens(data);

        assert_eq!(parsed_tokens, expected_import_ast);
    }
}
