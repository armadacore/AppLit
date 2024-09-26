use std::fmt::Debug;
use crate::bin::constants;
use crate::core::execute::token_reader::TokenReaderStack;

const IMPORT_TOKEN: &str = "import";

const FROM_TOKEN: &str = "from";

#[derive(Debug, Clone)]
pub struct ImportDeclaration {
    pub pos: usize,
    pub end: usize,
    pub line_start: usize,
    pub line_end: usize,
    pub specifier: Vec<String>,
    pub from: Option<String>,
}

pub fn try_declaration(stack: &mut TokenReaderStack<super::ModuleDeclaration>) -> bool {
    try_to_declare(stack, |declaration|{
        super::ModuleDeclaration::Import(declaration)
    })
}

pub fn try_declaration_with<T: Debug, F>(stack: &mut TokenReaderStack<T>, add: F) -> bool
where F: Fn(ImportDeclaration) -> T {
    try_to_declare(stack, add)
}

pub fn try_to_declare<T: Debug, F>(stack: &mut TokenReaderStack<T>, add: F) -> bool
where F: Fn(ImportDeclaration) -> T
{
    if let Some(token) = &stack.get_token() {
        if token == IMPORT_TOKEN {
            let mut declaration = ImportDeclaration{
                pos: stack.get_pos(),
                end: 0,
                line_start: stack.get_line_number(),
                line_end: 0,
                specifier: vec![],
                from: None
            };

            loop_tokens(&mut declaration, stack);
            declaration.end = stack.get_end();
            declaration.line_end = stack.get_line_number();
            stack.ast_add(add(declaration));

            return true;
        }
    }

    false
}

fn loop_tokens<T: Debug>(declaration: &mut ImportDeclaration, stack: &mut TokenReaderStack<T>){
    while let Some(token) = stack.next() {
        let mut current_token = token;

        match current_token.as_str() {
            constants::EMPTY | constants::START_CURLY_BRACES_TOKEN | constants::END_CURLY_BRACES_TOKEN => continue,
            FROM_TOKEN => {
                if let Some(from) = stack.next(){
                    let cleaned = from.replace(constants::SINGLE_QUOTES_TOKEN, "").replace(constants::SEMICOLON_TOKEN, "");
                    declaration.from = Some(cleaned);
                }
                break;
            },
            _ => {
                if current_token.starts_with(constants::START_CURLY_BRACES_TOKEN){
                    current_token.remove(0).to_string();
                }

                if current_token.ends_with(constants::COMMA_TOKEN){
                    current_token.pop();
                }

                if current_token.ends_with(constants::END_CURLY_BRACES_TOKEN){
                    current_token.pop();
                }
            }
        };

        declaration.specifier.push(current_token);
    }
}