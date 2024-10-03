use crate::core::tokenizer::reader::{
    TokenReaderLocation, TokenReaderNodes, TokenReaderSnapshot, TokenReaderStack,
};
use crate::core::tokenizer::utils::{declaration, location, surrounded_by};
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

const IMPORT_TOKEN: &str = "import";

const FROM_TOKEN: &str = "from";

const MISSING_IMPORT: &str = "Missing import identifier";

const MISSING_REFERENCE: &str = "Missing import reference";

#[derive(Debug, Clone)]
pub struct ImportDeclaration {
    pub location: TokenReaderLocation,
    pub nodes: TokenReaderNodes<ImportIdentifier>,
    pub reference: Option<ImportReference>,
}

#[derive(Debug, Clone)]
pub struct ImportIdentifier {
    pub location: TokenReaderLocation,
    pub identifier: String,
}

#[derive(Debug, Clone)]
pub struct ImportReference {
    pub location: TokenReaderLocation,
    pub identifier: String,
}

pub fn try_declaration(stack: &mut TokenReaderStack<super::ModuleDeclaration>) -> bool {
    try_to_declare(stack, |declaration| {
        super::ModuleDeclaration::Import(declaration)
    })
}

pub fn try_declaration_with<T: Debug + Clone, F>(stack: &mut TokenReaderStack<T>, add: F) -> bool
where
    F: Fn(ImportDeclaration) -> T,
{
    try_to_declare(stack, add)
}

fn try_to_declare<T: Debug + Clone, F>(stack: &mut TokenReaderStack<T>, add: F) -> bool
where
    F: Fn(ImportDeclaration) -> T,
{
    if let Some(token) = &stack.get_token() {
        if token == IMPORT_TOKEN {
            let mut declaration = ImportDeclaration {
                location: location::get_location(stack),
                nodes: vec![],
                reference: None,
            };

            loop_tokens(&mut declaration, stack);
            stack.push_declaration(add(declaration));

            return true;
        }
    }

    false
}

fn loop_tokens<T: Debug + Clone>(
    declaration: &mut ImportDeclaration,
    stack: &mut TokenReaderStack<T>,
) {
    let import_identifiers: Rc<RefCell<Vec<TokenReaderSnapshot>>> = Rc::new(RefCell::new(vec![]));
    let reference_identifiers: Rc<RefCell<Vec<TokenReaderSnapshot>>> =
        Rc::new(RefCell::new(vec![]));

    let on_curly_braces = surrounded_by::curly_braces(|snapshot| {
        (*import_identifiers.borrow_mut()).push(snapshot);
    });

    let on_single_quotes = surrounded_by::single_quotes(|snapshot| {
        (*reference_identifiers.borrow_mut()).push(snapshot);
    });

    let mut structure: Vec<Box<dyn FnMut(TokenReaderSnapshot) -> bool>> =
        vec![Box::new(on_curly_braces), Box::new(on_single_quotes)];

    if declaration::structure_validation(stack, &mut structure) {
        location::update_location_end(stack, &mut declaration.location);

        let import_identifiers_ref = import_identifiers.borrow();
        if !import_identifiers_ref.is_empty() {
            import_identifiers_ref.iter().for_each(|next_literal_item| {
                if next_literal_item.token.is_some() {
                    declaration.nodes.push(ImportIdentifier {
                        location: location::get_location(stack),
                        identifier: next_literal_item.token.clone().unwrap(),
                    });
                }
            });
        } else {
            stack.syntax_error(location::from_to(import_identifiers_ref), MISSING_IMPORT);
        }

        let reference_identifier_ref = reference_identifiers.borrow();
        if !reference_identifier_ref.is_empty() {
            if let Some(reference) = reference_identifier_ref.first() {
                let location = reference.location.clone();
                let identifier = reference.token.clone().unwrap();

                declaration.reference = Some(ImportReference {
                    location,
                    identifier,
                })
            }
        }

        if declaration.reference.is_none() {
            stack.syntax_error(location::get_location(stack), MISSING_REFERENCE)
        }
    }
}
