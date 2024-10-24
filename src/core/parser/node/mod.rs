use crate::composer::AppLit;
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

    pub fn parse(&mut self, app_lit: &mut AppLit) -> Result<(), ErrorCause> {
        let mut statements = Vec::<AstMainNode>::new();

        while self.tokens.peek().is_some() {
            let statement = main::parse_statement(self)?;
            statements.push(statement);
        }

        let path = "main";
        if app_lit.exist_ast_node_item(path) {
            panic!("Main source already exists");
        } else {
            app_lit.add_ast_node_item(path, AstNode::Main(AstMainNode::Statements(statements)));
        }
        
        Ok(())
    }

    fn parse_module(&mut self) -> Result<AstNode, ErrorCause> {
        let mut statements = Vec::<AstModuleNode>::new();

        while self.tokens.peek().is_some() {
            let statement = module::parse_statement(self)?;
            statements.push(statement);
        }

        Ok(AstNode::Module(AstModuleNode::Statements(statements)))
    }
}
