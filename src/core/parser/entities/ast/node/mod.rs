use crate::core::parser::{AstMainNode, AstModuleNode};
use serde::{Deserialize, Serialize};

pub mod main;

pub mod module;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AstNode {
    Main(AstMainNode),
    Module(AstModuleNode),
}
