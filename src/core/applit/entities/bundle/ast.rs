use crate::core::applit::entities::bundle::{AppLit, AppLitAst};
use crate::core::feedback::error::Cause;
use crate::core::parser::node::AstNode;
use std::sync::{Arc, Mutex, MutexGuard};

impl AppLitAst {
    pub fn exist_reference(&self, path: &str) -> bool {
        self.references.contains_key(path)
    }
    
    pub fn insert_reference(&mut self, path: &str, index: usize) -> Option<usize> {
        self.references.insert(path.into(), index)
    }
    
    pub fn push_ast_node(&mut self, ast_node: AstNode) -> usize {
        self.nodes.push(ast_node);
        
        self.nodes.len() - 1
    }
}

impl AppLit {
    pub fn clone_ast(&self) -> Option<Arc<Mutex<AppLitAst>>>{
        self.ast.clone()
    }

    pub fn get_ast(&self) -> Result<MutexGuard<AppLitAst>, Cause> {
        if let Some(ast_mutex) = &self.ast {
            return Ok(ast_mutex.lock().unwrap());
        }

        Err(Cause::UnexpectedError("Ast Mutex is None".into()))
    }

    pub fn add_ast_node_with_reference(&mut self, reference: &str, ast_node: AstNode) -> usize {
        if let Some(ast) = &mut self.ast {
            let mut ast = ast.lock().unwrap();

            let index = ast.push_ast_node(ast_node);
            
            ast.insert_reference(reference, index);

            return index;
        }

        panic!("Attempted to add a node item to a composer without valid AST.");
    }

    pub fn exist_ast_node_item(&self, reference: &str) -> bool {
        if let Some(ast) = &self.ast {
            let ast = ast.lock().unwrap();
            
            return ast.exist_reference(reference);
        }

        panic!("Attempted to find a node without valid AST.");
    }
}