use crate::core::tokenizer::TokenDeclaration;

mod models;
use crate::core::feedback::error::ErrorCause;
pub use models::{ast::*, parser::*};

mod keywords;

pub fn parse_tokens<'a>(tokens: Vec<TokenDeclaration>) -> Result<AstNode, ErrorCause<'a>> {
    Parser::new(tokens).parse_program()
}

#[cfg(test)]
pub mod tests {
    use crate::core::feedback::error::ErrorCause;
    use crate::core::parser::{parse_tokens, AstNode, Parser};
    use crate::core::tokenizer::tests::create_token_declarations;
    use std::io::Cursor;

    pub fn create_ast_node(statements: Vec<AstNode>) -> AstNode {
        AstNode::Program { statements }
    }

    pub fn create_parser(data: &str) -> Parser{
        let cursor = Cursor::new(data);
        let token_declarations = create_token_declarations(cursor);

        Parser::new(token_declarations)
    }

    pub fn create_parsed(data: &str) -> Result<AstNode, ErrorCause> {
        let cursor = Cursor::new(data);
        let token_declarations = create_token_declarations(cursor);

        parse_tokens(token_declarations)
    }
}
