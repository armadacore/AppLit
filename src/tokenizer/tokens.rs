use crate::feedback::error::ErrorFeedback;
use crate::tokenizer::ast::{AstMode, AstOperation};

pub mod lexer;

#[derive(Debug, Clone)]
pub enum Token {
    Import(lexer::import::ImportDeclaration),
    Function(lexer::function::Declaration),
}

pub fn initialize(ast_operation: AstOperation) -> Result<Vec<Token>, ErrorFeedback> {
    match ast_operation.mode {
        AstMode::App => lexer::new(&ast_operation.file_path),
        AstMode::AppLit => todo!("read binary file and return [Ast]"),
    }
}