#![allow(unused_variables)]
#![allow(dead_code)]

use crate::core::feedback::ErrorCause;
use crate::core::parser::{main_tree_builder, module_tree_builder, AstNode};
use crate::core::tokenizer::{tokenize_file, TokenDeclaration};
use std::path::PathBuf;

pub mod bin;

mod core;

pub struct AppLit {
    
}

pub fn tokenize_source(path: &str) -> Result<Vec<TokenDeclaration>, ErrorCause> {
    let file_path = PathBuf::from(path);

    if file_path.exists() && file_path.is_file() {
        Ok(tokenize_file(file_path))
    } else {
        Err(ErrorCause::FileNotFound(path))
    }
}

pub fn parse_main<'a>(tokens: Vec<TokenDeclaration>) -> Result<AstNode, ErrorCause<'a>> {
    main_tree_builder(tokens)
}

pub fn parse_module<'a>(tokens: Vec<TokenDeclaration>) -> Result<AstNode, ErrorCause<'a>> {
    module_tree_builder(tokens)
}
