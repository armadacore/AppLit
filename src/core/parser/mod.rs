use crate::core::feedback::error::ErrorCause;
use crate::core::tokenizer::TokenDeclaration;

mod keywords;
pub use keywords::*;

mod entities;
pub use entities::{
    ast::{
        error::AstError,
        node::AstNode,
        program::*,
        statements::{
            function::*,
            import::*
        }
    },
    tree::*
};

pub fn parse_tokens<'a>(tokens: Vec<TokenDeclaration>) -> Result<AstNode, ErrorCause<'a>> {
    Builder::new(tokens).parse_module()
}
