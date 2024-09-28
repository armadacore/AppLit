use crate::token::reader::{
    TokenReaderLocation, TokenReaderNextLiteral, TokenReaderNodes, TokenReaderStack,
};
use crate::token::utils::location;
use crate::token::utils::location::{get_location, update_location_end};
use crate::token::utils::on_surrounded;
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

const IMPORT_TOKEN: &str = "import";

const FROM_TOKEN: &str = "from";

const MISSING_IMPORT: &str = "Missing import identifier";

const MISSING_REFERENCE: &str = "Missing import reference";

#[derive(Debug)]
pub struct ImportDeclaration {
    pub location: TokenReaderLocation,
    pub nodes: TokenReaderNodes<ImportIdentifier>,
    pub reference: Option<ImportReference>,
}

#[derive(Debug)]
pub struct ImportIdentifier {
    pub location: TokenReaderLocation,
    pub identifier: String,
}

#[derive(Debug)]
pub struct ImportReference {
    pub location: TokenReaderLocation,
    pub identifier: String,
}

pub fn try_declaration(stack: &mut TokenReaderStack<super::ModuleDeclaration>) -> bool {
    try_to_declare(stack, |declaration| {
        super::ModuleDeclaration::Import(declaration)
    })
}

pub fn try_declaration_with<T: Debug, F>(stack: &mut TokenReaderStack<T>, add: F) -> bool
where
    F: Fn(ImportDeclaration) -> T,
{
    try_to_declare(stack, add)
}

fn try_to_declare<T: Debug, F>(stack: &mut TokenReaderStack<T>, add: F) -> bool
where
    F: Fn(ImportDeclaration) -> T,
{
    if let Some(token) = &stack.get_token() {
        if token == IMPORT_TOKEN {
            let mut declaration = ImportDeclaration {
                location: get_location(stack),
                nodes: vec![],
                reference: None,
            };

            loop_tokens(&mut declaration, stack);
            stack.add_declaration(add(declaration));

            return true;
        }
    }

    false
}

fn loop_tokens<T: Debug>(declaration: &mut ImportDeclaration, stack: &mut TokenReaderStack<T>) {
    let import_identifiers: Rc<RefCell<Vec<TokenReaderNextLiteral>>> =
        Rc::new(RefCell::new(vec![]));
    let mut on_curly_braces = on_surrounded::curly_braces(|next_literal| {
        (*import_identifiers.borrow_mut()).push(next_literal);
    });
    let reference_identifiers: Rc<RefCell<Vec<TokenReaderNextLiteral>>> =
        Rc::new(RefCell::new(vec![]));
    let mut on_single_quotes = on_surrounded::single_quotes(|next_literal| {
        (*reference_identifiers.borrow_mut()).push(next_literal);
    });

    while let Some(import_next_literal) = stack.next_literal() {
        on_curly_braces(&import_next_literal);
        on_single_quotes(&import_next_literal);
    }

    update_location_end(stack, &mut declaration.location);

    let import_identifiers_ref = import_identifiers.borrow();
    if !import_identifiers_ref.is_empty() {
        import_identifiers_ref.iter().for_each(|next_literal_item| {
            declaration.nodes.push(ImportIdentifier {
                location: get_location(stack),
                identifier: next_literal_item.token.clone(),
            });
        });
    } else {
        stack.syntax_error(location::from_to(import_identifiers_ref), MISSING_IMPORT);
    }

    let reference_identifier_ref = reference_identifiers.borrow();
    if !reference_identifier_ref.is_empty() {
        if let Some(reference) = reference_identifier_ref.first() {
            declaration.reference = Some(ImportReference {
                location: reference.location.clone(),
                identifier: reference.token.clone(),
            })
        }
    }

    if declaration.reference.is_none() {
        stack.syntax_error(get_location(stack), MISSING_REFERENCE)
    }
}
