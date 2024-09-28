use crate::token::declaration::DeclarationResult;
use crate::token::reader;
use std::fmt::Debug;
use std::path::Path;

mod function;
pub mod import;

#[derive(Debug)]
pub enum ModuleDeclaration {
    Import(import::ImportDeclaration),
    Function(function::Declaration),
}

pub fn token(file_path: &Path) -> DeclarationResult<ModuleDeclaration> {
    reader::run(file_path, |stack| {
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
