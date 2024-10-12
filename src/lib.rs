#![allow(unused_variables)]
#![allow(dead_code)]

use crate::core::feedback::error::ErrorCause;
use crate::core::parser::{parse_tokens, AstNode};
use crate::core::tokenizer::{tokenize_file, TokenDeclaration};
use std::path::PathBuf;

pub mod bin;

mod core;

pub fn tokenize_source(path: &str) -> Result<Vec<TokenDeclaration>, ErrorCause> {
    let file_path = PathBuf::from(path);

    if file_path.exists() && file_path.is_file() {
        Ok(tokenize_file(file_path))
    } else {
        Err(ErrorCause::FileNotFound(path))
    }
}

pub fn parse_source<'a>(tokens: Vec<TokenDeclaration>) -> Result<AstNode, ErrorCause<'a>> {
    parse_tokens(tokens)
}