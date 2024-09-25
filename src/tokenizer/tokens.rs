use crate::feedback::error::ErrorFeedback;
use crate::tokenizer::ast::{AstMode, AstOperation};
use crate::tokenizer::lexer;

mod import;
mod function;

#[derive(Debug, Clone)]
pub enum Token {
    Import(import::ImportDeclaration),
    Function(function::Declaration),
}

pub fn initialize(ast_operation: AstOperation) -> Result<Vec<Token>, ErrorFeedback> {
    match ast_operation.mode {
        AstMode::App => {
            lexer::new(&ast_operation.file_path, |t2a| {
                if let Some(token) = t2a.get_token(){
                    if import::check(t2a) { return }
                    if function::check() {  }
                }
            })
        },
        AstMode::AppLit => todo!("read binary file and return [Ast]"),
    }
}