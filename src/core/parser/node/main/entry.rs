use crate::core::feedback::error::Cause;
use crate::core::parser::node::main::statement_parser::{parse_main_statements, AstMainNode};
use crate::core::parser::node::{AstNode, TreeBuilder};
use crate::core::parser::statements::import::ImportStatement;
use crate::core::tokenizer::tokenize_file;

impl<'a> TreeBuilder<'a> {
    pub fn parse_main(&mut self) -> Result<(), Cause> {
        let path = "/main";
        if self.app_lit.exist_ast_node_item(path) {
            panic!("Main source already exists");
        }

        let mut tokens = tokenize_file(self.app_lit.get_entry())?;
        let ast_node = parse_main_statements(&mut tokens)?;
        let index = self.app_lit.add_ast_node_with_reference(path, ast_node);
        let import_statements = self.get_main_import_statements(index)?;

        self.parse_modules(import_statements)
    }
    
    fn get_main_import_statements(&mut self, index: usize) -> Result<Vec<ImportStatement>, Cause> {
        Ok(if let Some(AstNode::Main(AstMainNode::Statements(statements))) = self.app_lit.get_ast()?.nodes.get(index) {
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
        })
    }
}
