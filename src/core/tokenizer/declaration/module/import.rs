use crate::bin::constants;
use crate::core::tokenizer::{
    reader::{TokenReaderLocation, TokenReaderNodes, TokenReaderStack},
    utils,
};
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

const IMPORT_TOKEN: &str = "import";

const FROM_TOKEN: &str = "from";

const UNKNOWN_IMPORT_STATEMENT: &str = "Unknown import statement";

const MISSING_IDENTIFIER: &str = "Missing identifier";

const MISSING_IMPORT: &str = "Missing import identifiers";

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
    let declaration = Rc::new(RefCell::new(ImportDeclaration {
        location: utils::location::get_location(stack),
        nodes: vec![],
        reference: None,
    }));
    let is_valid = utils::declaration::validate(|validator| {
        validator.expected_token(IMPORT_TOKEN, |_| {});

        {
            let declaration = Rc::clone(&declaration);
            validator.between_curly_braces(move |snapshot| {
                let location = snapshot.location;
                let identifier = snapshot.token.expect(MISSING_IDENTIFIER);

                declaration.borrow_mut().nodes.push(ImportIdentifier {
                    location,
                    identifier,
                });
            });
        }

        validator.expected_token(FROM_TOKEN, |_| {});

        {
            let declaration = Rc::clone(&declaration);
            validator.between_single_quotes(move |snapshot| {
                let location = snapshot.location;
                let identifier = snapshot.token.expect(MISSING_IDENTIFIER);

                declaration.borrow_mut().reference = Some(ImportReference {
                    location,
                    identifier,
                });
            });
        }

        validator.expected_token(constants::SEMICOLON_TOKEN, |_| {});

        validator.check(stack)
    });

    let mut declaration = Rc::try_unwrap(declaration)
        .expect("Unable to unwrap declaration")
        .into_inner();
    utils::location::update_location_end(stack, &mut declaration.location);

    if !is_valid {
        stack.syntax_error(
            utils::location::get_location(stack),
            UNKNOWN_IMPORT_STATEMENT,
        );
        return false;
    }

    if declaration.nodes.is_empty() {
        stack.syntax_error(utils::location::get_location(stack), MISSING_IMPORT);
        return false;
    }

    if declaration.reference.is_none() {
        stack.syntax_error(utils::location::get_location(stack), MISSING_REFERENCE);
        return false;
    }

    stack.push_declaration(add(declaration));

    true
}
