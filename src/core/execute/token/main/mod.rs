use std::path::Path;
use crate::core::execute::token::DeclarationResult;
use crate::core::execute::token_reader;

mod id;

#[derive(Debug)]
pub enum MainDeclaration {
    Id(String),
    Icon(String),
    Name(String),
    Version(String),
    Description(String),
    Link(String),
    Domain(String)
}

pub fn declaration(file_path: &Path) -> DeclarationResult<MainDeclaration>{
    let mut tokens = vec![
        id::try_declaration
    ];
    
    token_reader::run_tokens(file_path, &mut tokens)
}