use crate::core::feedback::ErrorCause;
use crate::core::parser::AstNode;

use crate::core::tokenizer::TokenDeclaration;
use std::iter::Peekable;
use std::vec::IntoIter;

mod module;

pub struct TreeBuilder {
    pub tokens: Peekable<IntoIter<TokenDeclaration>>,
}

impl<'a> TreeBuilder {
    pub fn new(tokens: Vec<TokenDeclaration>) -> Self {
        TreeBuilder {
            tokens: tokens.into_iter().peekable(),
        }
    }
    
    pub fn parse_main(&mut self) -> Result<AstNode, ErrorCause<'a>> {
        todo!()
    }

    pub fn parse_module(&mut self) -> Result<AstNode, ErrorCause<'a>> {
        module::parse(self)
    }
}
