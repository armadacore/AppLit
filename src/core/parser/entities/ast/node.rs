use crate::core::parser::{AstNodeMain, AstNodeModule};

#[derive(Debug, PartialEq)]
pub enum AstNode {
    Main(AstNodeMain),
    Module(AstNodeModule),
}