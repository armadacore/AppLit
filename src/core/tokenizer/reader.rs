use crate::bin::constants;
use crate::core::feedback::error::ErrorCause;
use regex::Regex;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

const IDENTIFIER_REGEX: &str = r"^[a-zA-Z0-9_]+$";
const LITERAL_REGEX: &str = r"'([^']*)'";


#[derive(Debug, Clone)]
pub struct TokenSnapshot {
    pub location: TokenLocation,
    pub token: String,
}

#[derive(Debug, Clone)]
pub struct TokenLocation {
    pub start: usize,
    pub end: usize,
    pub line: usize,
}

#[derive(Debug, Clone)]
pub enum TokenDeclaration {
    Keyword(TokenSnapshot),
    Identifier(TokenSnapshot),
    Literal(TokenSnapshot),
    
    ArgumentOpen(TokenSnapshot),
    ArgumentClose(TokenSnapshot),
    
    BlockOpen(TokenSnapshot),
    BlockClose(TokenSnapshot),
    
    AssignmentStatement(TokenSnapshot),
    StatementDivider(TokenSnapshot),
    StatementEnd(TokenSnapshot),
    
    Error(TokenSnapshot),
}

pub fn new<'a>(file_path: PathBuf) -> Result<Vec<TokenDeclaration>, ErrorCause<'a>> {
    let file = File::open(&file_path).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut result: Vec<TokenDeclaration> = Vec::new();
    let mut line_count: usize = 1;
    let mut start_count: usize = 0;

    // let identifier_regex = Regex::new("[a-zA-Z_][a-zA-Z0-9_]*").unwrap();
    let identifier_regex = Regex::new(IDENTIFIER_REGEX).unwrap();
    // let literal_regex = Regex::new("'([^']*)'").unwrap();
    let literal_regex = Regex::new(LITERAL_REGEX).unwrap();

    for line_result in lines {
        let line_data = line_result.expect("Error reading line");
        let mut tokens = split_line(line_data.as_str());

        for token_data in &mut tokens {
            if token_data.trim().is_empty() {
                start_count += 1;
                continue;
            }

            let start = start_count;
            let end = start_count + token_data.len();
            let token_snapshot = TokenSnapshot {
                location: TokenLocation {
                    line: line_count,
                    start,
                    end,
                },
                token: token_data.clone(),
            };

            start_count = end;
            result.push(match token_data.as_str() {
                constants::IMPORT_TOKEN | constants::FROM_TOKEN => TokenDeclaration::Keyword(token_snapshot),
                
                constants::LEFT_ROUND_BRACKETS_TOKEN => TokenDeclaration::ArgumentOpen(token_snapshot),
                constants::RIGHT_ROUND_BRACKETS_TOKEN => TokenDeclaration::ArgumentClose(token_snapshot),
                
                constants::LEFT_CURLY_BRACES_TOKEN => TokenDeclaration::BlockOpen(token_snapshot),
                constants::RIGHT_CURLY_BRACES_TOKEN => TokenDeclaration::BlockClose(token_snapshot),
                
                constants::COLON_TOKEN => TokenDeclaration::AssignmentStatement(token_snapshot),
                constants::COMMA_TOKEN => TokenDeclaration::StatementDivider(token_snapshot),
                constants::SEMICOLON_TOKEN => TokenDeclaration::StatementEnd(token_snapshot),
                
                literal_token if literal_regex.is_match(literal_token) => TokenDeclaration::Literal(token_snapshot),
                identifier_token if identifier_regex.is_match(identifier_token) => TokenDeclaration::Identifier(token_snapshot),
                
                _ => TokenDeclaration::Error(token_snapshot),
            });
        }

        line_count += 1;
    }

    Ok(result)
}

fn split_line(line: &str) -> Vec<String> {
    let regex_tokens = [
        constants::LEFT_ROUND_BRACKETS_TOKEN,
        constants::LEFT_CURLY_BRACES_TOKEN,
        constants::COLON_TOKEN,
        constants::COMMA_TOKEN,
        constants::RIGHT_CURLY_BRACES_TOKEN,
        constants::RIGHT_ROUND_BRACKETS_TOKEN,
        constants::SEMICOLON_TOKEN,
    ];
    let regex_pattern = format!("'[^']*'|\\w+|[{}]|\\s", regex_tokens.join(""));
    let regexp = Regex::new(&regex_pattern).unwrap();
    let result: Vec<String> = regexp
        .find_iter(line)
        .map(|res| res.as_str().to_string())
        .collect();

    result
}
