use crate::core::tokenizer::reader::{TokenReaderLocation, TokenReaderStack};

const ID_TOKEN: &str = "@id";

pub enum ElementDeclarationKind {
    DeveloperId,
    AppLitId,
}

pub struct IdElementDeclaration {
    pub pos: usize,
    pub end: usize,
    pub line_start: usize,
    pub line_end: usize,
    pub kind: ElementDeclarationKind,
    pub specifier: String,
}

pub struct IdDeclaration {
    pub location: TokenReaderLocation,
    pub nodes: Vec<IdElementDeclaration>,
}

pub fn try_declaration(stack: &mut TokenReaderStack<super::MainDeclaration>) -> bool {
    if let Some(token) = &stack.get_token() {
        if token.starts_with(ID_TOKEN) {}
    }

    false
}
