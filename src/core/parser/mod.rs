use crate::core::tokenizer::TokenDeclaration;


mod models;
pub use models::{ast::*, parser::*};

mod keywords;

pub fn translate_tokens(tokens: Vec<TokenDeclaration>) -> Result<AstNode, AstError> {
    Parser::new(tokens).parse_program()
}