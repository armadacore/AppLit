use crate::bin::constants;
use crate::core::feedback::error::ErrorCause;
use crate::core::parser::{AstError, AstNode, Parser};
use crate::core::tokenizer::{try_snapshot_error, TokenDeclaration, TokenSnapshot};

pub fn parse<'a>(parser: &mut Parser) -> Result<AstNode, ErrorCause<'a>> {
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

fn parse_namespace<'a>(parser: &mut Parser) -> Result<Option<TokenSnapshot>, ErrorCause<'a>> {
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

fn parse_identifiers<'a>(parser: &mut Parser) -> Result<Vec<TokenSnapshot>, ErrorCause<'a>> {
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

fn parse_reference<'a>(parser: &mut Parser) -> Result<TokenSnapshot, ErrorCause<'a>> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::parser::tests::{create_ast_node, create_parsed, create_parser};
    use crate::core::tokenizer::TokenLocation;

    #[test]
    fn statement_without_namespace_is_valid() {
        let expected_import_ast = create_ast_node(vec![AstNode::Import {
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
        }]);
        let data = "import {pi,co} from 'applit';";
        let parsed_tokens = create_parsed(data);

        assert_eq!(
            parsed_tokens.expect("Import token declarations did not parse"),
            expected_import_ast
        );
    }

    #[test]
    fn statement_with_namespace_is_valid() {
        let expected_import_ast = create_ast_node(vec![AstNode::Import {
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
        }]);
        let data = "import foobar:{pi,co} from 'applit';";
        let parsed_tokens = create_parsed(data);

        assert_eq!(
            parsed_tokens.expect("Import token declarations did not parse"),
            expected_import_ast
        );
    }

    #[test]
    fn statement_missing_end() {
        let data = "import {foo} from 'somewhere'";
        let mut parser = create_parser(data);
        let parsed = parse(&mut parser);

        match parsed {
            Err(ErrorCause::SyntaxError(AstError::UnexpectedError(None))) => {/* assert true */},
            Ok(_) => panic!("Did not fail"),
            _ => panic!("Error isn't as expected"),
        }
    }

    #[test]
    fn parse_namespace_has_unexpected_error() {
        let data = "import foobar";
        let mut parser = create_parser(data);
        parser.tokens.next();
        let parsed = parse_namespace(&mut parser);

        match parsed {
            Err(ErrorCause::SyntaxError(AstError::UnexpectedError(None))) => {/* assert true */},
            Ok(_) => panic!("Did not fail"),
            _ => panic!("Error isn't as expected"),
        }
    }

    #[test]
    fn parse_namespace_has_unexpected_token_error() {
        let data = "import foobar;";
        let mut parser = create_parser(data);
        parser.tokens.next();
        let parsed = parse_namespace(&mut parser);

        match parsed {
            Err(ErrorCause::SyntaxError(AstError::UnexpectedToken(_))) => {/* assert true */},
            Ok(_) => panic!("Did not fail"),
            _ => panic!("Error isn't as expected"),
        }
    }

    #[test]
    fn parse_identifiers_has_unexpected_token_error() {
        let data = "foobar ;";
        let mut parser = create_parser(data);
        let parsed = parse_identifiers(&mut parser);

        match parsed {
            Err(ErrorCause::SyntaxError(AstError::UnexpectedToken(_))) => {/* assert true */}
            Ok(_) => panic!("Did not fail"),
            _ => panic!("Error isn't as expected"),
        }
    }

    #[test]
    fn parse_incomplete_identifiers_has_unexpected_token_error() {
        let data = "{ foo, ;";
        let mut parser = create_parser(data);
        let parsed = parse_identifiers(&mut parser);

        match parsed {
            Err(ErrorCause::SyntaxError(AstError::UnexpectedError(_))) => {/* assert true */}
            Ok(_) => panic!("Did not fail"),
            _ => panic!("Error isn't as expected"),
        }
    }

    #[test]
    fn parse_reference_has_unexpected_token_error() {
        let data = "";
        let mut parser = create_parser(data);
        let parsed = parse_reference(&mut parser);

        match parsed {
            Err(ErrorCause::SyntaxError(AstError::UnexpectedError(_))) => {/* assert true */}
            Ok(_) => panic!("Did not fail"),
            _ => panic!("Error isn't as expected"),
        }
    }

    #[test]
    fn parse_reference_without_from_condition_has_unexpected_token_error() {
        let data = "no_from";
        let mut parser = create_parser(data);
        let parsed = parse_reference(&mut parser);

        match parsed {
            Err(ErrorCause::SyntaxError(AstError::UnexpectedError(_))) => {/* assert true */}
            Ok(_) => panic!("Did not fail"),
            _ => panic!("Error isn't as expected"),
        }
    }

    #[test]
    fn parse_reference_no_literal_after_from_condition_has_unexpected_token_error() {
        let data = "from";
        let mut parser = create_parser(data);
        let parsed = parse_reference(&mut parser);

        match parsed {
            Err(ErrorCause::SyntaxError(AstError::UnexpectedError(_))) => {/* assert true */}
            Ok(_) => panic!("Did not fail"),
            _ => panic!("Error isn't as expected"),
        }
    }

    #[test]
    fn parse_reference_invalid_reference_condition_has_unexpected_token_error() {
        let data = "from foobar";
        let mut parser = create_parser(data);
        let parsed = parse_reference(&mut parser);

        match parsed {
            Err(ErrorCause::SyntaxError(AstError::UnexpectedError(_))) => {/* assert true */}
            Ok(_) => panic!("Did not fail"),
            _ => panic!("Error isn't as expected"),
        }
    }
}
