use super::{constants, TokenDeclaration, TokenLocation, TokenSnapshot};
use crate::bin;
use regex::Regex;

pub fn split_line(line: &str) -> Vec<String> {
    let regex_tokens = constants::REGEX_TOKENS.join("");
    let regex_pattern = format!(r#"'(?:\\'|[^'])*'|\w+|[{}]|\s"#, regex_tokens);
    let regexp = Regex::new(&regex_pattern).unwrap();
    let result: Vec<String> = regexp
        .find_iter(line)
        .map(|res| res.as_str().to_string())
        .collect();

    result
}

pub fn match_token(token: &str, line: usize, start: usize, end: usize) -> TokenDeclaration {
    let identifier_regex =
        Regex::new(constants::IDENTIFIER_REGEX).expect("identifier regex are broken");
    let literal_regex = Regex::new(constants::LITERAL_REGEX).expect("literal regex are broken");
    let token_location = TokenLocation::new(line, start, end);
    let token_snapshot = TokenSnapshot::new(token_location, token.into());

    match token {
        bin::constants::KEYWORD_IMPORT | bin::constants::KEYWORD_FROM => {
            TokenDeclaration::Keyword(token_snapshot)
        }

        bin::constants::ARGUMENT_OPEN => TokenDeclaration::ArgumentOpen(token_snapshot),
        bin::constants::ARGUMENT_CLOSE => TokenDeclaration::ArgumentClose(token_snapshot),

        bin::constants::BLOCK_OPEN => TokenDeclaration::BlockOpen(token_snapshot),
        bin::constants::BLOCK_CLOSE => TokenDeclaration::BlockClose(token_snapshot),

        bin::constants::STATEMENT_ASSIGNMENT => {
            TokenDeclaration::StatementAssignment(token_snapshot)
        }
        bin::constants::STATEMENT_DIVIDER => TokenDeclaration::StatementDivider(token_snapshot),
        bin::constants::STATEMENT_END => TokenDeclaration::StatementEnd(token_snapshot),

        literal_token if literal_regex.is_match(literal_token) => {
            TokenDeclaration::Literal(token_snapshot)
        }
        identifier_token if identifier_regex.is_match(identifier_token) => {
            TokenDeclaration::Identifier(token_snapshot)
        }

        _ => TokenDeclaration::Unknown(token_snapshot),
    }
}

#[cfg(test)]
mod unit_tests {
    mod match_token_unit;
    mod split_line_unit;
}
