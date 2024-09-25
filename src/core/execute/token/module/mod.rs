use std::path::Path;
use crate::core::execute::lexer;
use crate::feedback::error::ErrorFeedback;

mod function;
pub mod import;

#[derive(Debug)]
pub enum ModuleToken {
    Import(import::ImportDeclaration),
    Function(function::Declaration),
}

pub fn declaration(file_path: &Path) -> Result<Vec<ModuleToken>, ErrorFeedback> {
    lexer::new(file_path, |stack| {
        if let Some(token) = stack.get_token(){
            if import::check(stack) { return }
            if function::check() {  }
        }
    })
}