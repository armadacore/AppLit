use crate::core::feedback::ErrorCause;
use crate::core::tokenizer::TokenDeclaration;

mod keywords;
pub use keywords::*;

mod entities;
pub use entities::{
    ast::{
        error::*,
        node::{AstNode, main::*, module::*},
        statements::{
            function::*,
            import::*
        }
    },
    tree::*
};

pub fn main_tree_builder(tokens: Vec<TokenDeclaration>) -> Result<AstNode, ErrorCause> {
    TreeBuilder::new(tokens).parse_main()
}

pub fn module_tree_builder(tokens: Vec<TokenDeclaration>) -> Result<AstNode, ErrorCause> {
    TreeBuilder::new(tokens).parse_module()
}

#[cfg(test)]
mod tests {
    use crate::core::parser::TreeBuilder;
    use crate::core::tokenizer::tests::create_token_declarations;
    use std::io::Cursor;

    pub fn create_builder(statement: &str) -> TreeBuilder{
        let cursor = Cursor::new(statement);
        let token_declaration = create_token_declarations(cursor);

        TreeBuilder::new(token_declaration)
    }
}