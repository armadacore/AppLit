use crate::core::execute::token::DeclarationResult;
use crate::core::execute::token_reader;
use std::fmt::Debug;
use std::path::Path;

mod function;
pub mod import;

#[derive(Debug)]
pub enum ModuleDeclaration {
    Import(import::ImportDeclaration),
    Function(function::Declaration),
}

pub fn declaration(file_path: &Path) -> DeclarationResult<ModuleDeclaration> {
    token_reader::run(file_path, |stack| {
        if let Some(token) = stack.get_token() {
            if import::try_declaration(stack) {
                return true;
            }
            if function::try_declaration(stack) {
                return true;
            }
        }

        false
    })
}
