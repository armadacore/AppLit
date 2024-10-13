use crate::core::parser::{AstMainNode, AstModuleNode};

#[derive(Debug, PartialEq)]
pub enum AstNode {
    Main(AstMainNode),
    Module(AstModuleNode),
}