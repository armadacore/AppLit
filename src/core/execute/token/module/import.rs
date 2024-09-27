use std::fmt::Debug;
use crate::bin::constants;
// use crate::bin::constants;
use crate::core::execute::token_reader::{TokenReaderLocation, TokenReaderNextLiteral, TokenReaderNodes, TokenReaderStack};

const IMPORT_TOKEN: &str = "import";

const FROM_TOKEN: &str = "from";

#[derive(Debug)]
pub struct ImportDeclaration {
    pub location: TokenReaderLocation,
    pub nodes: TokenReaderNodes<ImportIdentifier>,
    pub reference: Option<ImportReference>,
}

#[derive(Debug)]
pub struct ImportIdentifier {
    pub location: TokenReaderLocation,
    pub identifier: String
}

#[derive(Debug)]
pub struct ImportReference {
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
            stack.update_location_end(&mut declaration.location);
            stack.add_declaration(add(declaration));

            return true;
        }
    }

    false
}

fn loop_tokens<T: Debug>(declaration: &mut ImportDeclaration, stack: &mut TokenReaderStack<T>){
    let mut start_curly_braces = false;
    let mut end_curly_braces = false;
    let mut nodes: TokenReaderNodes<ImportIdentifier> = vec![];

    while let Some(TokenReaderNextLiteral{prev_token, token}) = stack.next_literal() {
        if let Some(staring_curly_braces) = &prev_token{
            if staring_curly_braces == constants::START_CURLY_BRACES_TOKEN {
                start_curly_braces = true;
            }
        }

        if let Some(ending_curly_braces) = &prev_token{
            if ending_curly_braces == constants::END_CURLY_BRACES_TOKEN {
                end_curly_braces = true;
            }
        }

        if token == FROM_TOKEN {
            if let Some(next_literal) = stack.next_literal() {
                let location = stack.get_location();

                declaration.reference = Some(ImportReference {
                    location,
                    identifier: next_literal.token,
                });
            }
            continue;
        }

        nodes.push(ImportIdentifier {
            location: stack.get_location(),
            identifier: token
        });
    }

    if start_curly_braces && end_curly_braces{
        declaration.nodes = nodes;
    }
}