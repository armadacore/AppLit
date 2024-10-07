use crate::bin;
use crate::core::tokenizer::reader::TokenDeclaration;
use models::Parser;

mod models;
pub use models::{AstError, AstNode};

mod import;

pub fn translate_tokens(tokens: Vec<TokenDeclaration>) -> Result<AstNode, AstError> {
    Parser::new(tokens).parse_program()
}
