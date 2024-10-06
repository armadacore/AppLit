use crate::core::tokenizer::reader::{TokenDeclaration, TokenSnapshot, TokenLocation};
use crate::core::tokenizer::reader::utils::match_token;

fn create_snapshot(token: &str) -> TokenSnapshot{
    TokenSnapshot {
        location: TokenLocation{line: 0, start: 0, end: 0},
        token: token.to_string(),
    }
}

#[test]
fn match_keyword_for_import(){
    let input = "import";
    let result = match_token(input, 0, 0, 0);
    let expected = TokenDeclaration::Keyword(create_snapshot(input));
    
    assert_eq!(expected, result);
}

#[test]
fn match_keyword_for_from(){
    let input = "from";
    let result = match_token(input, 0, 0, 0);
    let expected = TokenDeclaration::Keyword(create_snapshot(input));

    assert_eq!(expected, result);
}

#[test]
fn match_argument_open(){
    let input = "(";
    let result = match_token(input, 0, 0, 0);
    let expected = TokenDeclaration::ArgumentOpen(create_snapshot(input));

    assert_eq!(expected, result);
}

#[test]
fn match_argument_close(){
    let input = ")";
    let result = match_token(input, 0, 0, 0);
    let expected = TokenDeclaration::ArgumentClose(create_snapshot(input));

    assert_eq!(expected, result);
}

#[test]
fn match_block_open(){
    let input = "{";
    let result = match_token(input, 0, 0, 0);
    let expected = TokenDeclaration::BlockOpen(create_snapshot(input));

    assert_eq!(expected, result);
}

#[test]
fn match_block_close(){
    let input = "}";
    let result = match_token(input, 0, 0, 0);
    let expected = TokenDeclaration::BlockClose(create_snapshot(input));

    assert_eq!(expected, result);
}

#[test]
fn match_statement_assignment(){
    let input = ":";
    let result = match_token(input, 0, 0, 0);
    let expected = TokenDeclaration::StatementAssignment(create_snapshot(input));

    assert_eq!(expected, result);
}

#[test]
fn match_statement_divider(){
    let input = ",";
    let result = match_token(input, 0, 0, 0);
    let expected = TokenDeclaration::StatementDivider(create_snapshot(input));

    assert_eq!(expected, result);
}

#[test]
fn match_statement_end(){
    let input = ";";
    let result = match_token(input, 0, 0, 0);
    let expected = TokenDeclaration::StatementEnd(create_snapshot(input));

    assert_eq!(expected, result);
}

#[test]
fn match_unknown(){
    let input = "µ";
    let result = match_token(input, 0, 0, 0);
    let expected = TokenDeclaration::Unknown(create_snapshot(input));

    assert_eq!(expected, result);
}