use std::path::Path;
use crate::core::execute::token_reader;
use crate::feedback::error::ErrorFeedback;

mod function;
pub mod import;

#[derive(Debug)]
pub enum ModuleDeclaration {
    Import(import::ImportDeclaration),
    Function(function::Declaration),
}

pub fn declaration(file_path: &Path) -> Result<Vec<ModuleDeclaration>, ErrorFeedback> {
    token_reader::new(file_path, |stack| {
        if let Some(token) = stack.get_token(){
            if import::try_declaration(stack) { return }
            if function::try_declaration() {  }
        }
    })
}