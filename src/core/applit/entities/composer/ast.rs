use crate::composer::{AppLit, AppLitAst};
use crate::core::feedback::ErrorCause;
use crate::core::parser::AstNode;
use std::sync::MutexGuard;

impl AppLit {
    pub fn get_ast(&self) -> Result<MutexGuard<AppLitAst>, ErrorCause> {
        if let Some(ast_mutex) = &self.ast {
            return Ok(ast_mutex.lock().unwrap());
        }

        Err(ErrorCause::UnexpectedError("Ast Mutex is None".into()))
    }

    pub fn add_ast_node_item(&mut self, item_path: &str, item_value: AstNode) -> usize {
        if let Some(ast) = &mut self.ast {
            let mut ast = ast.lock().unwrap();

            ast.nodes.push(item_value);

            let index = ast.nodes.len() - 1;
            let path = self.location.join(item_path);

            ast.references.insert(path, index);

            return index;
        }

        panic!("Attempted to add a node item to a composer without valid AST.");
    }

    pub fn exist_ast_node_item(&self, item_path: &str) -> bool {
        if let Some(ast) = &self.ast {
            let ast = ast.lock().unwrap();
            let path = self.location.join(item_path);

            return ast.references.contains_key(&path);
        }

        panic!("Attempted to find a node without valid AST.");
    }
}