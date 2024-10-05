use crate::core::feedback::error::ErrorCause;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

mod constants;

mod models;

mod utils;

use crate::core::tokenizer::reader::utils::match_token;
pub use models::*;

pub fn new<'a>(file_path: PathBuf) -> Result<Vec<TokenDeclaration>, ErrorCause<'a>> {
    let file = File::open(&file_path).unwrap();
    let reader = BufReader::new(file);
    
    create_token_declaration(reader)
}

fn create_token_declaration<'a>(reader: impl BufRead) -> Result<Vec<TokenDeclaration>, ErrorCause<'a>>{
    let lines = reader.lines();
    let mut result: Vec<TokenDeclaration> = Vec::new();
    let mut line_count: usize = 1;
    let mut start_count: usize = 0;

    for line_result in lines {
        let line_data = line_result.expect("Error reading line");
        let mut tokens = utils::split_line(line_data.as_str());

        for current_token in &mut tokens {
            if current_token.trim().is_empty() {
                start_count += 1;
                continue;
            }

            let end_count = start_count + current_token.len();
            let match_result = match_token(current_token, line_count, start_count, end_count);

            result.push(match_result);
            start_count = end_count;
        }

        line_count += 1;
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    mod import_statement;
}