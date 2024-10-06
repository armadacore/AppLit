use crate::core::tokenizer::reader::*;
use std::io::Cursor;
#[test]
fn result_is_ok() {
    let data = "import { pi, co } from 'applit';";
    let cursor = Cursor::new(data);
    let result = create_token_declaration(cursor);

    assert!(result.is_ok());
}

#[test]
fn result_is_as_expected() {
    let data = "import { pi, co } from 'applit';";
    let cursor = Cursor::new(data);
    let expected_import = vec![
        TokenDeclaration::Keyword(TokenSnapshot {
            location: TokenLocation {
                start: 0,
                end: 6,
                line: 1,
            },
            token: "import".into(),
        }),
        TokenDeclaration::BlockOpen(TokenSnapshot {
            location: TokenLocation {
                start: 7,
                end: 8,
                line: 1,
            },
            token: "{".into(),
        }),
        TokenDeclaration::Identifier(TokenSnapshot {
            location: TokenLocation {
                start: 9,
                end: 11,
                line: 1,
            },
            token: "pi".into(),
        }),
        TokenDeclaration::StatementDivider(TokenSnapshot {
            location: TokenLocation {
                start: 11,
                end: 12,
                line: 1,
            },
            token: ",".into(),
        }),
        TokenDeclaration::Identifier(TokenSnapshot {
            location: TokenLocation {
                start: 13,
                end: 15,
                line: 1,
            },
            token: "co".into(),
        }),
        TokenDeclaration::BlockClose(TokenSnapshot {
            location: TokenLocation {
                start: 16,
                end: 17,
                line: 1,
            },
            token: "}".into(),
        }),
        TokenDeclaration::Keyword(TokenSnapshot {
            location: TokenLocation {
                start: 18,
                end: 22,
                line: 1,
            },
            token: "from".into(),
        }),
        TokenDeclaration::Literal(TokenSnapshot {
            location: TokenLocation {
                start: 23,
                end: 31,
                line: 1,
            },
            token: "'applit'".into(),
        }),
        TokenDeclaration::StatementEnd(TokenSnapshot {
            location: TokenLocation {
                start: 31,
                end: 32,
                line: 1,
            },
            token: ";".into(),
        }),
    ];
    let result = create_token_declaration(cursor);

    assert_eq!(expected_import, result.unwrap());
}

#[test]
fn multiline_result_is_as_expected() {
    let data = "import {\npi,\nco\n} from 'applit';";
    let cursor = Cursor::new(data);
    let expected_import = vec![
        TokenDeclaration::Keyword(TokenSnapshot {
            location: TokenLocation {
                start: 0,
                end: 6,
                line: 1,
            },
            token: "import".into(),
        }),
        TokenDeclaration::BlockOpen(TokenSnapshot {
            location: TokenLocation {
                start: 7,
                end: 8,
                line: 1,
            },
            token: "{".into(),
        }),
        TokenDeclaration::Identifier(TokenSnapshot {
            location: TokenLocation {
                start: 8,
                end: 10,
                line: 2,
            },
            token: "pi".into(),
        }),
        TokenDeclaration::StatementDivider(TokenSnapshot {
            location: TokenLocation {
                start: 10,
                end: 11,
                line: 2,
            },
            token: ",".into(),
        }),
        TokenDeclaration::Identifier(TokenSnapshot {
            location: TokenLocation {
                start: 11,
                end: 13,
                line: 3,
            },
            token: "co".into(),
        }),
        TokenDeclaration::BlockClose(TokenSnapshot {
            location: TokenLocation {
                start: 13,
                end: 14,
                line: 4,
            },
            token: "}".into(),
        }),
        TokenDeclaration::Keyword(TokenSnapshot {
            location: TokenLocation {
                start: 15,
                end: 19,
                line: 4,
            },
            token: "from".into(),
        }),
        TokenDeclaration::Literal(TokenSnapshot {
            location: TokenLocation {
                start: 20,
                end: 28,
                line: 4,
            },
            token: "'applit'".into(),
        }),
        TokenDeclaration::StatementEnd(TokenSnapshot {
            location: TokenLocation {
                start: 28,
                end: 29,
                line: 4,
            },
            token: ";".into(),
        }),
    ];
    let result = create_token_declaration(cursor);

    assert_eq!(expected_import, result.unwrap());
}

#[test]
fn with_module_name_is_as_expected() {
    let data = "import foobar:{pi,co} from 'applit';";
    let cursor = Cursor::new(data);
    let expected_import = vec![
        TokenDeclaration::Keyword(
            TokenSnapshot {
                location: TokenLocation {
                    start: 0,
                    end: 6,
                    line: 1,
                },
                token: "import".into(),
            },
        ),
        TokenDeclaration::Identifier(
            TokenSnapshot {
                location: TokenLocation {
                    start: 7,
                    end: 13,
                    line: 1,
                },
                token: "foobar".into(),
            },
        ),
        TokenDeclaration::StatementAssignment(
            TokenSnapshot {
                location: TokenLocation {
                    start: 13,
                    end: 14,
                    line: 1,
                },
                token: ":".into(),
            },
        ),
        TokenDeclaration::BlockOpen(
            TokenSnapshot {
                location: TokenLocation {
                    start: 14,
                    end: 15,
                    line: 1,
                },
                token: "{".into(),
            },
        ),
        TokenDeclaration::Identifier(
            TokenSnapshot {
                location: TokenLocation {
                    start: 15,
                    end: 17,
                    line: 1,
                },
                token: "pi".into(),
            },
        ),
        TokenDeclaration::StatementDivider(
            TokenSnapshot {
                location: TokenLocation {
                    start: 17,
                    end: 18,
                    line: 1,
                },
                token: ",".into(),
            },
        ),
        TokenDeclaration::Identifier(
            TokenSnapshot {
                location: TokenLocation {
                    start: 18,
                    end: 20,
                    line: 1,
                },
                token: "co".into(),
            },
        ),
        TokenDeclaration::BlockClose(
            TokenSnapshot {
                location: TokenLocation {
                    start: 20,
                    end: 21,
                    line: 1,
                },
                token: "}".into(),
            },
        ),
        TokenDeclaration::Keyword(
            TokenSnapshot {
                location: TokenLocation {
                    start: 22,
                    end: 26,
                    line: 1,
                },
                token: "from".into(),
            },
        ),
        TokenDeclaration::Literal(
            TokenSnapshot {
                location: TokenLocation {
                    start: 27,
                    end: 35,
                    line: 1,
                },
                token: "'applit'".into(),
            },
        ),
        TokenDeclaration::StatementEnd(
            TokenSnapshot {
                location: TokenLocation {
                    start: 35,
                    end: 36,
                    line: 1,
                },
                token: ";".into(),
            },
        ),

    ];
    let result = create_token_declaration(cursor);

    assert_eq!(expected_import, result.unwrap());
}