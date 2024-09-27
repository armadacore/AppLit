use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
use crate::core::execute::token_reader::{
    TokenReaderLocation, 
    TokenReaderNextLiteral, 
    TokenReaderNodes, 
    TokenReaderStack
};
use crate::core::execute::token_utils::declaration_node::push_next_literal_token;
use crate::core::execute::token_utils::on_surrounded;

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
    let import_identifiers: Rc<RefCell<Vec<TokenReaderNextLiteral>>> = Rc::new(RefCell::new(vec![]));
    let mut on_curly_braces = on_surrounded::curly_braces(|next_literals| {
        *import_identifiers.borrow_mut() = next_literals.to_vec();
    });

    while let Some(import_next_literal) = stack.next_literal() {
        if on_curly_braces(&import_next_literal) { continue; }
        if import_next_literal.token == FROM_TOKEN {
            if let Some(next_literal) = stack.next_literal() {
                let location = stack.get_location();
        
                declaration.reference = Some(ImportReference {
                    location,
                    identifier: next_literal.token,
                });
            }
            continue;
        }
    }

    push_next_literal_token(stack, declaration, import_identifiers.borrow());
}