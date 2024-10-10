use crate::core::tokenizer::TokenDeclaration;

mod models;
pub use models::{ast::*, parser::*};

mod keywords;

pub fn parse_tokens(tokens: Vec<TokenDeclaration>) -> Result<AstNode, AstError> {
    Parser::new(tokens).parse_program()
}

#[cfg(test)]
pub mod tests {
    use crate::core::parser::{parse_tokens, AstNode};
    use crate::core::tokenizer::tests::create_token_declarations;
    use std::io::Cursor;

    pub fn create_parsed_tokens(data: &str) -> AstNode {
        let cursor = Cursor::new(data);
        let token_declarations = create_token_declarations(cursor);

        parse_tokens(token_declarations.expect("Missing import token declarations"))
            .expect("Import token declarations did not parse")
    }
}
