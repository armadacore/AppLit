use crate::core::tokenizer::reader::{
    TokenReaderLocation, TokenReaderNodes, TokenReaderSnapshot, TokenReaderStack,
};
use crate::core::tokenizer::utils;
use crate::bin::constants;
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

const IMPORT_TOKEN: &str = "import";

const FROM_TOKEN: &str = "from";

const UNKNOWN_IMPORT_STATEMENT: &str = "Unknown import statement";

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
    let mut declaration = ImportDeclaration {
        location: utils::location::get_location(stack),
        nodes: vec![],
        reference: None,
    };
    let import_identifiers: Rc<RefCell<Vec<TokenReaderSnapshot>>> = Rc::new(RefCell::new(vec![]));
    let reference_identifiers: Rc<RefCell<Vec<TokenReaderSnapshot>>> =
        Rc::new(RefCell::new(vec![]));
    let on_import = utils::declaration::expected_token(IMPORT_TOKEN.to_string(), |_| {});
    let on_curly_braces = utils::declaration::between_curly_braces(|snapshot| {
        (*import_identifiers.borrow_mut()).push(snapshot);
    });
    let on_from = utils::declaration::expected_token(FROM_TOKEN.to_string(), |_| {});
    let on_single_quotes = utils::declaration::between_single_quotes(|snapshot| {
        (*reference_identifiers.borrow_mut()).push(snapshot);
    });
    let on_end = utils::declaration::expected_token(constants::SEMICOLON_TOKEN.to_string(), |_|{});
    let mut structure: Vec<Box<dyn FnMut(TokenReaderSnapshot) -> bool>> = vec![
        Box::new(on_import),
        Box::new(on_curly_braces),
        Box::new(on_from),
        Box::new(on_single_quotes),
        Box::new(on_end),
    ];

    if !utils::declaration::structure_validation(stack, &mut structure) {
        stack.syntax_error(
            utils::location::get_location(stack),
            UNKNOWN_IMPORT_STATEMENT,
        );
        return false;
    }

    let import_identifiers_ref = import_identifiers.borrow();
    let reference_identifier_ref = reference_identifiers.borrow();

    utils::location::update_location_end(stack, &mut declaration.location);

    if import_identifiers_ref.is_empty() {
        stack.syntax_error(utils::location::from_to(import_identifiers_ref), MISSING_IMPORT);
        return false;
    }

    if reference_identifier_ref.is_empty() {
        stack.syntax_error(utils::location::get_location(stack), MISSING_REFERENCE);
        return false;
    }

    import_identifiers_ref.iter().for_each(|next_literal_item| {
        if next_literal_item.token.is_some() {
            declaration.nodes.push(ImportIdentifier {
                location: utils::location::get_location(stack),
                identifier: next_literal_item.token.clone().unwrap(),
            });
        }
    });
    
    if let Some(reference) = reference_identifier_ref.first() {
        let location = reference.location.clone();
        let identifier = reference.token.clone().unwrap();

        declaration.reference = Some(ImportReference {
            location,
            identifier,
        });
    }

    stack.push_declaration(add(declaration));
    true
}
