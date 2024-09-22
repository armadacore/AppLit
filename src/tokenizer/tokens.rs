use crate::feedback::error::ErrorFeedback;
use crate::tokenizer::ast::{AstMode, AstOperation};

pub mod nodes;

#[derive(Debug)]
pub enum Token {
    Import(nodes::import::Declaration),
}

pub fn initialize(ast_operation: AstOperation) -> Result<Vec<Token>, ErrorFeedback> {
    match ast_operation.mode {
        AstMode::App => nodes::new(ast_operation),
        AstMode::AppLit => todo!("read binary file and return [Ast]"),
    }
}