use crate::core::tokenizer::TokenDeclaration;

mod models;
use crate::core::feedback::error::ErrorCause;
pub use models::{
    ast::{
        error::AstError,
        node::AstNode,
        program::*,
        statements::{
            function::*,
            import::*
        }
    },
    parser::*,
};

mod keywords;

pub fn parse_tokens<'a>(tokens: Vec<TokenDeclaration>) -> Result<AstNode, ErrorCause<'a>> {
    Parser::new(tokens).parse_program()
}
