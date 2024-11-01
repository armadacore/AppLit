use crate::bin::constants;
use crate::core::feedback::error::Cause;
use crate::core::parser::error::AstError;
use crate::core::tokenizer::{snapshot_error, TokenDeclaration, TokenSnapshot, Tokens};
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
        Ok(ImportStatement{
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
                Err(Cause::SyntaxError(AstError::UnexpectedToken(
                    token.extract_snapshot(),
                )))
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

/*
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
        let result_tree = parse_import_statement(&mut builder).unwrap();

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
        let result_tree = parse_import_statement(&mut builder).unwrap();

        assert_eq!(expected_tree, result_tree);
    }

    #[test]
    fn statement_missing_end_should_fail() {
        let statement = "import {foo} from 'somewhere'";
        let mut builder = create_builder(statement);
        let parsed = parse_import_statement(&mut builder);

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
*/