use crate::bin::constants;
use crate::core::feedback::error::ErrorCause;
use crate::core::parser::keywords::import;
use crate::core::parser::{AstError, AstNode, AstNodeProgram, ProgramStatement};
use crate::core::tokenizer::TokenDeclaration;
use std::iter::Peekable;
use std::vec::IntoIter;

pub struct Parser {
    pub tokens: Peekable<IntoIter<TokenDeclaration>>,
}

impl<'a> Parser {
    pub fn new(tokens: Vec<TokenDeclaration>) -> Self {
        Parser {
            tokens: tokens.into_iter().peekable(),
        }
    }

    pub fn parse_program(&mut self) -> Result<AstNode, ErrorCause<'a>> {
        let mut statements = Vec::<ProgramStatement>::new();

        while self.tokens.peek().is_some() {
            let statement = self.parse_program_statement()?;
            statements.push(statement);
        }

        Ok(AstNode::Program(AstNodeProgram::Statements(statements)))
    }

    pub fn parse_program_statement(&mut self) -> Result<ProgramStatement, ErrorCause<'a>> {
        let peek = self.tokens.peek();

        if peek.is_none() {
            return Err(ErrorCause::SyntaxError(AstError::UnexpectedEOF));
        }

        let peek = peek.unwrap();

        if let TokenDeclaration::Keyword(snapshot) = peek {
            return match snapshot.token.as_str() {
                constants::KEYWORD_IMPORT => Ok(ProgramStatement::Import(import::parse(self)?)),
                unknown_token => Err(ErrorCause::SyntaxError(AstError::UnexpectedToken(snapshot.clone()))),
            };
        }

        panic!(
            "Try to parse on top level for unknown TokenDeclaration {:#?}",
            self.tokens.peek().unwrap()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::tokenizer::{TokenLocation, TokenSnapshot};

    #[test]
    fn parse_statement_peek_is_none(){
        let mut parser = Parser::new(vec![]);

        match parser.parse_program_statement() {
            Err(ErrorCause::SyntaxError(AstError::UnexpectedEOF)) => (),
            _ => panic!("Did not fail"),
        }
    }

    #[test]
    fn parse_statement_unknown_keyword() {
        let location = TokenLocation::new(0,0,0);
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
        let location = TokenLocation::new(0,0,0);
        let snapshot = TokenSnapshot::new(location, "not_exist_toplevel_token".into());
        let mut parser = Parser::new(vec![TokenDeclaration::Literal(snapshot)]);
        
        parser.parse_program_statement().unwrap();
    }
}