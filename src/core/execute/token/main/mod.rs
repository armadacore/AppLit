use crate::core::execute::token::module::import;
use crate::core::execute::token::DeclarationResult;
use crate::core::execute::token_reader;
use crate::core::execute::token_reader::TokenReaderStack;
use std::fmt::Debug;
use std::path::Path;

mod id;

#[derive(Debug)]
pub enum MainDeclaration {
    Import(import::ImportDeclaration),
    Id(String),
    Icon(String),
    Name(String),
    Version(String),
    Description(String),
    Link(String),
    Domain(String),
}

pub fn declaration(file_path: &Path) -> DeclarationResult<MainDeclaration> {
    let mut tokens = vec![import_try_declaration, id::try_declaration];

    token_reader::run_tokens(file_path, &mut tokens)
}

fn import_try_declaration(stack: &mut TokenReaderStack<MainDeclaration>) -> bool {
    import::try_declaration_with(stack, |declaration| MainDeclaration::Import(declaration))
}
