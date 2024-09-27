use std::cell::Ref;
use std::fmt::Debug;
use crate::core::execute::token::module::import::{ImportDeclaration, ImportIdentifier};
use crate::core::execute::token_reader::{TokenReaderNextLiteral, TokenReaderStack};

pub fn push_next_literal_token<T: Debug>(
    stack: &TokenReaderStack<T>, 
    declaration: &mut ImportDeclaration, 
    nodes: Ref<Vec<TokenReaderNextLiteral>>
) -> bool {
    if !nodes.is_empty() {
        nodes.iter().for_each(|next_literal_item| {
            declaration.nodes.push(ImportIdentifier {
                location: stack.get_location(),
                identifier: next_literal_item.token.clone()
            });
        });
        
        return true;
    }
    
    false
}