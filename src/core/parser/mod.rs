use crate::core::feedback::ErrorCause;
use crate::core::tokenizer::TokenDeclaration;

mod keywords;
pub use keywords::*;

mod entities;
pub use entities::{
    ast::{
        error::*,
        main::*,
        node::*,
        program::*,
        statements::{
            function::*,
            import::*
        }
    },
    tree::*
};

pub fn parse_tokens<'a>(tokens: Vec<TokenDeclaration>) -> Result<AstNode, ErrorCause<'a>> {
    TreeBuilder::new(tokens).parse_module()
}
