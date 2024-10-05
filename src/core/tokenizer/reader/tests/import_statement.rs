use std::io::Cursor;
use crate::core::tokenizer::reader::*;

#[test]
fn import_statement_result_is_ok(){
    let data = "import { pi, co } from 'applit';";
    let cursor = Cursor::new(data);
    let result = token_declaration(cursor);

    assert!(result.is_ok());
}

#[test]
fn import_statement_result_is_as_expected(){
    let data = "import { pi, co } from 'applit';";
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
        TokenDeclaration::BlockOpen(
            TokenSnapshot {
                location: TokenLocation {
                    start: 7,
                    end: 8,
                    line: 1,
                },
                token: "{".into(),
            },
        ),
        TokenDeclaration::Identifier(
            TokenSnapshot {
                location: TokenLocation {
                    start: 9,
                    end: 11,
                    line: 1,
                },
                token: "pi".into(),
            },
        ),
        TokenDeclaration::StatementDivider(
            TokenSnapshot {
                location: TokenLocation {
                    start: 11,
                    end: 12,
                    line: 1,
                },
                token: ",".into(),
            },
        ),
        TokenDeclaration::Identifier(
            TokenSnapshot {
                location: TokenLocation {
                    start: 13,
                    end: 15,
                    line: 1,
                },
                token: "co".into(),
            },
        ),
        TokenDeclaration::BlockClose(
            TokenSnapshot {
                location: TokenLocation {
                    start: 16,
                    end: 17,
                    line: 1,
                },
                token: "}".into(),
            },
        ),
        TokenDeclaration::Keyword(
            TokenSnapshot {
                location: TokenLocation {
                    start: 18,
                    end: 22,
                    line: 1,
                },
                token: "from".into(),
            },
        ),
        TokenDeclaration::Literal(
            TokenSnapshot {
                location: TokenLocation {
                    start: 23,
                    end: 31,
                    line: 1,
                },
                token: "'applit'".into(),
            },
        ),
        TokenDeclaration::StatementEnd(
            TokenSnapshot {
                location: TokenLocation {
                    start: 31,
                    end: 32,
                    line: 1,
                },
                token: ";".into(),
            },
        ),
    ];
    let result = token_declaration(cursor);
    
    assert_eq!(expected_import, result.unwrap());
}