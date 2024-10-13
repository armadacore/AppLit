use crate::core::parser::models::ast::main::AstNodeMain;
use crate::core::parser::models::ast::program::AstNodeProgram;

#[derive(Debug, PartialEq)]
pub enum AstNode {
    Main(AstNodeMain),
    Program(AstNodeProgram),
}