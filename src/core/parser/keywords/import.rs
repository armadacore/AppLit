use crate::bin::constants;
use crate::core::feedback::ErrorCause;
use crate::core::parser::{AstError, ImportStatement, TreeBuilder};
use crate::core::tokenizer::{snapshot_error, TokenDeclaration, TokenSnapshot};

pub fn parse<'a>(builder: &mut TreeBuilder) -> Result<ImportStatement, ErrorCause<'a>> {
    let snapshot = builder.tokens.peek().unwrap().extract_snapshot();
    builder.tokens.next();

    let namespace = parse_namespace(builder)?;
    let identifiers = parse_identifiers(builder)?;
    let reference = parse_reference(builder)?;

    if let Some(TokenDeclaration::StatementEnd(_)) = builder.tokens.next() {
        Ok(ImportStatement{
            snapshot,
            namespace,
            identifiers,
            reference,
        })
    } else {
        Err(snapshot_error(builder.tokens.peek()))
    }
}

fn parse_namespace<'a>(builder: &mut TreeBuilder) -> Result<Option<TokenSnapshot>, ErrorCause<'a>> {
    if let Some(TokenDeclaration::Identifier(name)) = builder.tokens.peek().cloned() {
        builder.tokens.next();

        if let Some(token) = builder.tokens.next() {
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

fn parse_identifiers<'a>(builder: &mut TreeBuilder) -> Result<Vec<TokenSnapshot>, ErrorCause<'a>> {
    let mut identifiers = Vec::<TokenSnapshot>::new();

    if let Some(TokenDeclaration::BlockOpen(_)) = builder.tokens.next() {
        loop {
            match builder.tokens.next() {
                Some(TokenDeclaration::Identifier(name)) => identifiers.push(name),
                Some(TokenDeclaration::StatementDivider(_)) => continue,
                Some(TokenDeclaration::BlockClose(_)) => break,
                _ => return Err(snapshot_error(builder.tokens.peek())),
            }
        }
    } else {
        return Err(snapshot_error(builder.tokens.peek()));
    }

    Ok(identifiers)
}

fn parse_reference<'a>(builder: &mut TreeBuilder) -> Result<TokenSnapshot, ErrorCause<'a>> {
    if let Some(TokenDeclaration::Keyword(snapshot)) = builder.tokens.next() {
        if snapshot.token == constants::KEYWORD_FROM {
            if let Some(TokenDeclaration::Literal(source)) = builder.tokens.next() {
                Ok(source)
            } else {
                Err(snapshot_error(builder.tokens.peek()))
            }
        } else {
            Err(snapshot_error(builder.tokens.peek()))
        }
    } else {
        Err(snapshot_error(builder.tokens.peek()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::parser::tests::create_builder;
    use crate::core::tokenizer::TokenLocation;

    #[test]
    fn import_without_namespace_is_valid(){
        let statement = "import {pi,co} from 'applit';";
        let expected_tree = ImportStatement {
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
        };
        let mut builder = create_builder(statement);
        let result_tree = parse(&mut builder).unwrap();

        assert_eq!(expected_tree, result_tree);
    }

    #[test]
    fn import_with_namespace_is_valid(){
        let statement = "import foobar:{pi,co} from 'applit';";
        let expected_tree = ImportStatement {
            snapshot: TokenSnapshot {
                location: TokenLocation {
                    start: 0,
                    end: 6,
                    line: 1,
                },
                token: "import".into(),
            },
            namespace: Some(
                TokenSnapshot {
                    location: TokenLocation {
                        start: 7,
                        end: 13,
                        line: 1,
                    },
                    token: "foobar".into(),
                },
            ),
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
        };
        let mut builder = create_builder(statement);
        let result_tree = parse(&mut builder).unwrap();

        assert_eq!(expected_tree, result_tree);
    }

    #[test]
    fn statement_missing_end_should_fail() {
        let statement = "import {foo} from 'somewhere'";
        let mut builder = create_builder(statement);
        let parsed = parse(&mut builder);
    
        match parsed {
            Err(ErrorCause::SyntaxError(AstError::UnexpectedError(None))) => {/* assert true */},
            Ok(_) => panic!("Did not fail"),
            _ => panic!("Error isn't as expected"),
        }
    }
    
    #[test]
    fn parse_namespace_has_unexpected_error_should_fail() {
        let statement = "import foobar";
        let mut builder = create_builder(statement);
        builder.tokens.next();
        let parsed = parse_namespace(&mut builder);
    
        match parsed {
            Err(ErrorCause::SyntaxError(AstError::UnexpectedError(None))) => {/* assert true */},
            Ok(_) => panic!("Did not fail"),
            _ => panic!("Error isn't as expected"),
        }
    }
    
    #[test]
    fn parse_namespace_has_unexpected_token_should_fail() {
        let statement = "import foobar;";
        let mut builder = create_builder(statement);
        builder.tokens.next();
        let parsed = parse_namespace(&mut builder);
    
        match parsed {
            Err(ErrorCause::SyntaxError(AstError::UnexpectedToken(_))) => {/* assert true */},
            Ok(_) => panic!("Did not fail"),
            _ => panic!("Error isn't as expected"),
        }
    }
    
    #[test]
    fn parse_identifiers_has_unexpected_token_should_fail() {
        let statement = "foobar ;";
        let mut builder = create_builder(statement);
        let parsed = parse_identifiers(&mut builder);
    
        match parsed {
            Err(ErrorCause::SyntaxError(AstError::UnexpectedToken(_))) => {/* assert true */}
            Ok(_) => panic!("Did not fail"),
            _ => panic!("Error isn't as expected"),
        }
    }
    
    #[test]
    fn parse_incomplete_identifiers_should_fail() {
        let statement = "{ foo, ;";
        let mut builder = create_builder(statement);
        let parsed = parse_identifiers(&mut builder);
    
        match parsed {
            Err(ErrorCause::SyntaxError(AstError::UnexpectedError(_))) => {/* assert true */}
            Ok(_) => panic!("Did not fail"),
            _ => panic!("Error isn't as expected"),
        }
    }
    
    #[test]
    fn parse_reference_empty_should_fail() {
        let statement = "";
        let mut builder = create_builder(statement);
        let parsed = parse_reference(&mut builder);
    
        match parsed {
            Err(ErrorCause::SyntaxError(AstError::UnexpectedError(_))) => {/* assert true */}
            Ok(_) => panic!("Did not fail"),
            _ => panic!("Error isn't as expected"),
        }
    }
    
    #[test]
    fn parse_reference_invalid_should_fail() {
        let statement = "no_from";
        let mut builder = create_builder(statement);
        let parsed = parse_reference(&mut builder);
    
        match parsed {
            Err(ErrorCause::SyntaxError(AstError::UnexpectedError(_))) => {/* assert true */}
            Ok(_) => panic!("Did not fail"),
            _ => panic!("Error isn't as expected"),
        }
    }
    
    #[test]
    fn parse_reference_missing_reference_should_fail() {
        let statement = "from";
        let mut builder = create_builder(statement);
        let parsed = parse_reference(&mut builder);
    
        match parsed {
            Err(ErrorCause::SyntaxError(AstError::UnexpectedError(_))) => {/* assert true */}
            Ok(_) => panic!("Did not fail"),
            _ => panic!("Error isn't as expected"),
        }
    }
    
    #[test]
    fn parse_reference_invalid_reference_should_fail() {
        let statement = "from foobar";
        let mut builder = create_builder(statement);
        let parsed = parse_reference(&mut builder);
    
        match parsed {
            Err(ErrorCause::SyntaxError(AstError::UnexpectedError(_))) => {/* assert true */}
            Ok(_) => panic!("Did not fail"),
            _ => panic!("Error isn't as expected"),
        }
    }
}