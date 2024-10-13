use crate::core::feedback::error::ErrorCause;
use crate::core::parser::AstNode;

use crate::core::tokenizer::TokenDeclaration;
use std::iter::Peekable;
use std::vec::IntoIter;

mod module;

pub struct Parser {
    pub tokens: Peekable<IntoIter<TokenDeclaration>>,
}

impl<'a> Parser {
    pub fn new(tokens: Vec<TokenDeclaration>) -> Self {
        Parser {
            tokens: tokens.into_iter().peekable(),
        }
    }

    pub fn parse_module(&mut self) -> Result<AstNode, ErrorCause<'a>> {
        module::parse(self)
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::tokenizer::{TokenLocation, TokenSnapshot};

    #[test]
    fn parse_statement_peek_is_none() {
        let mut parser = Parser::new(vec![]);

        match parser.parse_program_statement() {
            Err(ErrorCause::SyntaxError(AstError::UnexpectedEOF)) => (),
            _ => panic!("Did not fail"),
        }
    }

    #[test]
    fn parse_statement_unknown_keyword() {
        let location = TokenLocation::new(0, 0, 0);
        let snapshot = TokenSnapshot::new(location, "any_unknown_keyword_token".into());
        let mut parser = Parser::new(vec![TokenDeclaration::Keyword(snapshot)]);

        match parser.parse_program_statement() {
            Err(ErrorCause::SyntaxError(AstError::UnexpectedToken(_))) => {}
            _ => panic!("Did not fail"),
        }
    }

    #[test]
    #[should_panic]
    fn parse_statement_missing_keyword() {
        let location = TokenLocation::new(0, 0, 0);
        let snapshot = TokenSnapshot::new(location, "not_exist_toplevel_token".into());
        let mut parser = Parser::new(vec![TokenDeclaration::Literal(snapshot)]);

        parser.parse_program_statement().unwrap();
    }
}
*/
