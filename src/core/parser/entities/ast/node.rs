use crate::core::parser::{AstMainNode, AstModuleNode};
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AstNode {
    Main(AstMainNode),
    Module(AstModuleNode),
}