use crate::core::parser::{FunctionStatement, ImportStatement};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AstModuleNode {
    Statements(Vec<AstModuleNode>),
    Import(ImportStatement),
    Function(FunctionStatement)
}