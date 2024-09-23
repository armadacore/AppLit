use crate::feedback::error::ErrorFeedback;
use crate::tokenizer::ast::{AstMode, AstOperation};

pub mod verification;

#[derive(Debug, Clone)]
pub enum Token {
    Import(verification::import::ImportDeclaration),
    Function(verification::function::Declaration),
}

pub fn initialize(ast_operation: AstOperation) -> Result<Vec<Token>, ErrorFeedback> {
    match ast_operation.mode {
        AstMode::App => verification::new(&ast_operation.file_path),
        AstMode::AppLit => todo!("read binary file and return [Ast]"),
    }
}