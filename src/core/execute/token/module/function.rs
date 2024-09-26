use crate::core::execute::token_reader::TokenReaderStack;

#[derive(Debug, Clone)]
pub struct Declaration {
    pub specifier: String,
}
pub fn try_declaration(stack: &mut TokenReaderStack<super::ModuleDeclaration>) -> bool{
    false
}