use std::fmt::Debug;
use crate::bin::constants;
use crate::core::execute::token_reader::{TokenReaderLocation, TokenReaderNodes, TokenReaderStack};

const IMPORT_TOKEN: &str = "import";

const FROM_TOKEN: &str = "from";

#[derive(Debug)]
pub struct ImportDeclaration {
    pub location: TokenReaderLocation,
    pub nodes: TokenReaderNodes<ImportSpecifier>,
    pub reference: Option<String>,
}

#[derive(Debug)]
pub struct ImportSpecifier {
    pub location: TokenReaderLocation,
    pub identifier: String
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
where F: Fn(ImportDeclaration) -> T {
    if let Some(token) = &stack.get_token() {
        if token == IMPORT_TOKEN {
            let mut declaration = ImportDeclaration{
                location: stack.get_location(),
                nodes: vec![],
                reference: None
            };

            loop_tokens(&mut declaration, stack);
            stack.update_location(&mut declaration.location);
            stack.ast_add(add(declaration));

            return true;
        }
    }

    false
}

fn loop_tokens<T: Debug>(declaration: &mut ImportDeclaration, stack: &mut TokenReaderStack<T>){
    while let Some(token) = stack.next() {
        let cleaned = stack.get_clean_token(token);

        match cleaned.token.as_str() {
            constants::EMPTY | constants::START_CURLY_BRACES_TOKEN | constants::END_CURLY_BRACES_TOKEN => continue,
            FROM_TOKEN => {
                if let Some(from) = stack.next(){
                    let cleaned = from.replace(constants::SINGLE_QUOTES_TOKEN, "").replace(constants::SEMICOLON_TOKEN, "");
                    declaration.reference = Some(cleaned);
                }
                break;
            },
            _ => {
                declaration.nodes.push(ImportSpecifier{
                    location: cleaned.location,
                    identifier: cleaned.token
                });
            }
        };
    }
}