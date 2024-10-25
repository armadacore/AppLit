use crate::composer::AppLit;
use crate::core::feedback::ErrorCause;
use crate::core::parser::statements::import::ImportStatement;
use crate::core::parser::{parse_main_statements, AstMainNode, AstModuleNode};
use crate::core::tokenizer::{literally_to_clean_string, tokenize_file};
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
        TreeBuilder { app_lit }
    }

    pub fn parse_app(&mut self) -> Result<(), ErrorCause> {
        let path = "main";
        if self.app_lit.exist_ast_node_item(path) {
            panic!("Main source already exists");
        }

        let mut tokens = tokenize_file(&self.app_lit.get_entry());
        let statements = parse_main_statements(&mut tokens)?;
        let index = self.app_lit.add_ast_node_item(path, statements);
        let import_statements: Vec<ImportStatement> = {
            if let Some(AstNode::Main(AstMainNode::Statements(statements))) = self.app_lit.get_ast()?.nodes.get(index) {
                statements
                    .iter()
                    .filter_map(|stmt| {
                        if let AstMainNode::Import(import_statement) = stmt {
                            return Some(import_statement.clone());
                        }
                        None
                    })
                    .collect()
            } else {
                vec![]
            }
        };

        self.parse_modules(import_statements)
    }

    fn parse_modules(&mut self, mut import_statements: Vec<ImportStatement>) -> Result<(), ErrorCause> {
        while let Some(import_statement) = import_statements.pop() {
            let path = literally_to_clean_string(&import_statement.reference.token);
            if self.app_lit.exist_ast_node_item(&path) {
                continue;
            }
            let location = self.app_lit.get_joined_location(&path);
            let module_path = self.app_lit.get_module_path(&location);
            let tokens = tokenize_file(&module_path);

            println!("{:#?}", tokens);
        }
        Ok(())
    }
}
