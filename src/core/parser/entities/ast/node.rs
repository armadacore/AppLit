use crate::core::parser::entities::ast::main::AstNodeMain;
use crate::core::parser::entities::ast::program::AstNodeModule;

#[derive(Debug, PartialEq)]
pub enum AstNode {
    Main(AstNodeMain),
    Program(AstNodeModule),
}