use crate::core::parser::{FunctionStatement, ImportStatement};

#[derive(Debug, PartialEq)]
pub enum AstModuleNode {
    Statements(Vec<AstModuleNode>),
    Import(ImportStatement),
    Function(FunctionStatement)
}