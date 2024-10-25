use crate::composer::AppLit;
use crate::core::feedback::ErrorCause;
use crate::core::parser::{AstMainNode, AstModuleNode};
use crate::core::tokenizer::Tokens;
use serde::{Deserialize, Serialize};

pub mod main;

pub mod module;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AstNode {
    Main(AstMainNode),
    Module(AstModuleNode),
}

pub struct TreeBuilder<'a> {
    app_lit: &'a mut AppLit,
}

impl<'a> TreeBuilder<'a> {
    pub fn new(app_lit: &'a mut AppLit) -> Self {
        TreeBuilder {
            app_lit,
        }
    }

    pub fn parse_app(&mut self, tokens: &mut Tokens) -> Result<(), ErrorCause> {
        let path = "main";
        if self.app_lit.exist_ast_node_item(path) {
            panic!("Main source already exists");
        }

        let mut statements = Vec::<AstMainNode>::new();

        while tokens.peek().is_some() {
            let statement = main::parse_statement(tokens)?;
            statements.push(statement);
        }

        self.app_lit.add_ast_node_item(path, AstNode::Main(AstMainNode::Statements(statements)));
        Ok(())
    }

    // fn parse_module(&mut self, app_lit: &mut AppLit, path: &str) -> Result<(), ErrorCause> {
    //     if app_lit.exist_ast_node_item(path) {
    //         panic!("Module source \"{}\" already exists", path);
    //     }
    //
    //     let mut statements = Vec::<AstModuleNode>::new();
    //
    //     while self.tokens.peek().is_some() {
    //         let statement = module::parse_statement(self)?;
    //         statements.push(statement);
    //     }
    //
    //     app_lit.add_ast_node_item(path, AstNode::Module(AstModuleNode::Statements(statements)));
    //     Ok(())
    // }
}
