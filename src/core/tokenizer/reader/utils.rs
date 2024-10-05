use super::{constants, models, TokenDeclaration};
use crate::bin;
use regex::Regex;

pub fn split_line(line: &str) -> Vec<String> {
    let regex_pattern = format!("'[^']*'|\\w+|[{}]|\\s", constants::REGEX_TOKENS.join(""));
    let regexp = Regex::new(&regex_pattern).unwrap();
    let result: Vec<String> = regexp
        .find_iter(line)
        .map(|res| res.as_str().to_string())
        .collect();

    result
}

pub fn match_token(token: &str, line: usize, start: usize, end: usize) -> TokenDeclaration{
    let identifier_regex = Regex::new(constants::IDENTIFIER_REGEX).unwrap();
    let literal_regex = Regex::new(constants::LITERAL_REGEX).unwrap();
    let token_snapshot = models::TokenSnapshot {
        location: models::TokenLocation {
            line,
            start,
            end,
        },
        token: token.to_string(),
    };
    
    match token {
        bin::constants::IMPORT_TOKEN | bin::constants::FROM_TOKEN => TokenDeclaration::Keyword(token_snapshot),

        bin::constants::LEFT_ROUND_BRACKETS_TOKEN => TokenDeclaration::ArgumentOpen(token_snapshot),
        bin::constants::RIGHT_ROUND_BRACKETS_TOKEN => TokenDeclaration::ArgumentClose(token_snapshot),

        bin::constants::LEFT_CURLY_BRACES_TOKEN => TokenDeclaration::BlockOpen(token_snapshot),
        bin::constants::RIGHT_CURLY_BRACES_TOKEN => TokenDeclaration::BlockClose(token_snapshot),

        bin::constants::COLON_TOKEN => TokenDeclaration::AssignmentStatement(token_snapshot),
        bin::constants::COMMA_TOKEN => TokenDeclaration::StatementDivider(token_snapshot),
        bin::constants::SEMICOLON_TOKEN => TokenDeclaration::StatementEnd(token_snapshot),

        literal_token if literal_regex.is_match(literal_token) => TokenDeclaration::Literal(token_snapshot),
        identifier_token if identifier_regex.is_match(identifier_token) => TokenDeclaration::Identifier(token_snapshot),

        _ => TokenDeclaration::Error(token_snapshot),
    }
}