use crate::core::feedback::ErrorCause;
use crate::core::parser::{AstMainNode, AstModuleNode};
use crate::core::tokenizer::TokenDeclaration;
use serde::{Deserialize, Serialize};
use std::iter::Peekable;
use std::vec::IntoIter;

pub mod main;

pub mod module;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AstNode {
    Main(AstMainNode),
    Module(AstModuleNode),
}

pub struct TreeBuilder {
    pub tokens: Peekable<IntoIter<TokenDeclaration>>,
}

impl TreeBuilder {
    pub fn new(tokens: Vec<TokenDeclaration>) -> Self {
        TreeBuilder {
            tokens: tokens.into_iter().peekable(),
        }
    }

    pub fn parse_main(&mut self) -> Result<AstNode, ErrorCause> {
        self.parse_statement(&mut main::parse_statement)
    }

    pub fn parse_module(&mut self) -> Result<AstNode, ErrorCause> {
        self.parse_statement(&mut module::parse_statement)
    }

    fn parse_statement(&mut self, token_parser: &mut dyn  FnMut(&mut Self) -> Result<AstModuleNode, ErrorCause>) -> Result<AstNode, ErrorCause> {
        let mut statements = Vec::<AstModuleNode>::new();

        while self.tokens.peek().is_some() {
            let statement = token_parser(self)?;
            statements.push(statement);
        }

        Ok(AstNode::Module(AstModuleNode::Statements(statements)))
    }
}