#![allow(unused_variables)]
#![allow(dead_code)]

use std::path::PathBuf;
use crate::core::feedback::error::ErrorCause;
use crate::core::parser::ast::{translate_tokens, AstError, AstNode};
use crate::core::tokenizer;
use crate::core::tokenizer::reader::TokenDeclaration;

pub mod bin;

mod core;

pub fn tokenize_source(path: &str) -> Result<Vec<TokenDeclaration>, ErrorCause> {
    let file_path = PathBuf::from(path);

    if file_path.exists() && file_path.is_file() {
        let tokens = tokenizer::reader::translate_file(file_path)?;
        Ok(tokens)
    } else {
        Err(ErrorCause::FileNotFound(path))
    }
}

pub fn parse_source(tokens: Vec<TokenDeclaration>) -> Result<AstNode, AstError> {
    translate_tokens(tokens)
}