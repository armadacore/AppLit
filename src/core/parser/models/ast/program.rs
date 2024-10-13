use crate::core::parser::{FunctionStatement, ImportStatement};

#[derive(Debug, PartialEq)]
pub enum ModuleStatement {
    Import(ImportStatement),
    Function(FunctionStatement)
}

#[derive(Debug, PartialEq)]
pub enum AstNodeModule {
    Statements(Vec<ModuleStatement>)
}