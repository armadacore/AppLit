use crate::bin::constants;
use crate::composer::AppLit;
use crate::core::feedback::ErrorCause;
use crate::core::parser::statements::import::ImportStatement;
use crate::core::parser::{parse_main_statements, AstMainNode, AstModuleNode};
use crate::core::tokenizer::{literal_to_cleaned_string, tokenize_file};
use crossbeam_channel::unbounded;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;

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

        let mut tokens = tokenize_file(self.app_lit.get_entry());
        let ast_node = parse_main_statements(&mut tokens)?;
        let index = self.app_lit.add_ast_node_with_reference(path, ast_node);
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
        let pool = ThreadPool::new(constants::MAX_PARSE_THREADS);
        let (sender, receiver) = unbounded::<ErrorCause>();
        let arc_ast = self.app_lit.clone_ast().unwrap();
        let arc_sender = Arc::new(Mutex::new(sender));

        while let Some(import_statement) = import_statements.pop() {
            let arc_ast = Arc::clone(&arc_ast);
            let arc_sender = Arc::clone(&arc_sender);

            if receiver.try_recv().is_ok() {
                break;
            }

            let path = literal_to_cleaned_string(&import_statement.reference.token);
            let location = self.app_lit.get_joined_location(&path);
            let module_path = self.app_lit.get_module_path(&location);

            if self.app_lit.exist_ast_node_item(&path) {
                continue;
            }

            pool.execute(move || {
                let mut ast = arc_ast.lock().unwrap();
                let mut tokens = tokenize_file(&module_path);

                match parse_main_statements(&mut tokens) {
                    Ok(ast_node) => {
                        let index = ast.push_ast_node(ast_node);
                        ast.insert_reference(&path, index);
                    }
                    Err(error_cause) => {
                        let sender = arc_sender.lock().unwrap();
                        let _ = sender.send(error_cause);
                    }
                }
            });

        }
        pool.join();

        if let Ok(error_cause) = receiver.try_recv() {
            return Err(error_cause);
        }

        Ok(())
    }
}
