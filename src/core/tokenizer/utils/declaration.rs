use crate::core::tokenizer::reader::{TokenReaderSnapshot, TokenReaderStack};
use std::fmt::Debug;

pub fn structure_validation<T: Debug + Clone, F>(
    stack: &mut TokenReaderStack<T>,
    structure: &mut Vec<F>,
) -> bool
where
    F: FnMut(TokenReaderSnapshot) -> bool,
{
    while let Some(import_next_literal) = &stack.next_literal() {
        for declaration_item in structure.iter_mut() {
            if declaration_item(import_next_literal.clone()) {
                return true;
            }
        }
    }

    false
}
