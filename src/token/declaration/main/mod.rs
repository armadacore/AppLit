use crate::token::declaration::module::import;
use crate::token::declaration::DeclarationResult;
use crate::token::reader;
use crate::token::reader::TokenReaderStack;
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

pub fn token(file_path: &Path) -> DeclarationResult<MainDeclaration> {
    let mut tokens = vec![try_import_declaration, id::try_declaration];

    reader::run_tokens(file_path, &mut tokens)
}

fn try_import_declaration(stack: &mut TokenReaderStack<MainDeclaration>) -> bool {
    import::try_declaration_with(stack, MainDeclaration::Import)
}
