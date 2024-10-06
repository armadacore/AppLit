use crate::core::parser::ast::{AstError, AstNode};
use crate::core::parser::ast::models::Parser;

pub fn parse(parser: &mut Parser) -> Result<AstNode, AstError> {
    println!("parse_import");
    println!("{:#?}", parser.tokens.next());

    Err(AstError::UnexpectedEOF)
}