use crate::core::parser::{FunctionStatement, ImportStatement};

#[derive(Debug, PartialEq)]
pub enum AstNodeModule {
    Statements(Vec<AstNodeModule>),
    Import(ImportStatement),
    Function(FunctionStatement)
}