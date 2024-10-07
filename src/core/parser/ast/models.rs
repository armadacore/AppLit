use crate::bin;
use crate::core::parser::ast::import;
use crate::core::tokenizer::reader::{TokenDeclaration, TokenSnapshot};
use std::iter::Peekable;
use std::vec::IntoIter;

#[derive(Debug)]
pub enum AstNode {
    Program {
        statements: Vec<AstNode>,
    },
    Import {
        namespace: Option<TokenSnapshot>,
        identifiers: Vec<TokenSnapshot>,
        reference: TokenSnapshot,
    },
}

#[derive(Debug)]
pub enum AstError {
    UnexpectedToken(TokenSnapshot),
    UnexpectedError,
    UnexpectedEOF,
}

pub struct Parser {
    pub tokens: Peekable<IntoIter<TokenDeclaration>>,
}

impl Parser {
    pub fn new(tokens: Vec<TokenDeclaration>) -> Self {
        Parser {
            tokens: tokens.into_iter().peekable(),
        }
    }

    pub fn parse_program(&mut self) -> Result<AstNode, AstError> {
        let mut statements = Vec::<AstNode>::new();

        while self.tokens.peek().is_some() {
            let statement = self.parse_statement()?;
            statements.push(statement);
        }

        Ok(AstNode::Program { statements })
    }

    pub fn parse_statement(&mut self) -> Result<AstNode, AstError> {
        let peek = self.tokens.peek();

        if peek.is_none() {
            return Err(AstError::UnexpectedEOF);
        }

        if let TokenDeclaration::Keyword(snapshot) = peek.unwrap() {
            return match snapshot.token.as_str() {
                bin::constants::KEYWORD_IMPORT => Ok(import::parse(self)?),
                unknown_token => Err(AstError::UnexpectedToken(snapshot.clone())),
            };
        }

        panic!(
            "Try to parse on top level for unknown TokenDeclaration {:#?}",
            self.tokens.peek().unwrap()
        );
    }
}
