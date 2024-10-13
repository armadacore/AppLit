use crate::core::parser::{FunctionStatement, ImportStatement};

#[derive(Debug, PartialEq)]
pub enum ProgramStatement {
    Import(ImportStatement),
    Function(FunctionStatement)
}

#[derive(Debug, PartialEq)]
pub enum AstNodeProgram{
    Statements(Vec<ProgramStatement>)
}