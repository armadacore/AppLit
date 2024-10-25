use crate::composer::AppLit;
use crate::core::feedback::ErrorCause;
use crate::core::parser::statements::import::ImportStatement;
use crate::core::parser::{parse_main_statements, parse_module_statements, AstMainNode, AstModuleNode};
use crate::core::tokenizer::{literal_to_cleaned_string, tokenize_file};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

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
        let mut handles: Vec<JoinHandle<Result<(), ErrorCause>>> = vec![];
        let arc_ast = self.app_lit.clone_ast().unwrap();
        let arc_result:Arc<Mutex<Result<(), ErrorCause>>> = Arc::new(Mutex::new(Ok(())));

        while let Some(import_statement) = import_statements.pop() {
            let arc_ast = Arc::clone(&arc_ast);
            let arc_result = Arc::clone(&arc_result);

            let path = literal_to_cleaned_string(&import_statement.reference.token);
            let location = self.app_lit.get_joined_location(&path);
            let module_path = self.app_lit.get_module_path(&location);
            
            if self.app_lit.exist_ast_node_item(&path) {
                continue;
            }
            
            let handle = thread::spawn(move || -> Result<(), ErrorCause> {
                let mut ast = arc_ast.lock().unwrap();
                let mut tokens = tokenize_file(&module_path);
                let statements = parse_module_statements(&mut tokens)?;
                let index = ast.push_ast_node(statements);
                ast.insert_reference(&path, index);

                Ok(())
            });
            handles.push(handle);
        }

        for handle in handles {
            match handle.join() {
                Ok(res) => res?,
                Err(err) => {
                    // Änderungen am gemeinsamen Ergebnis, um den Fehlerzustand widerzuspiegeln
                    let mut result = arc_result.lock().unwrap();
                    *result = Err(ErrorCause::MutexUnwrapError("Thread panic".into()));
                    return result.clone();
                }
            }
        }
        Ok(())
    }
}
