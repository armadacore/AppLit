use crate::bin::constants;
use crate::core::feedback::ErrorCause;
use crate::core::parser::{parse_import_statement, AstError, AstModuleNode, TreeBuilder};
use crate::core::tokenizer::TokenDeclaration;

pub fn parse_statement(builder: &mut TreeBuilder) -> Result<AstModuleNode, ErrorCause> {
    let peek = builder.tokens.peek();

    if peek.is_none() {
        return Err(ErrorCause::SyntaxError(AstError::UnexpectedEOF));
    }

    let peek = peek.unwrap();

    if let TokenDeclaration::Keyword(snapshot) = peek {
        return match snapshot.token.as_str() {
            constants::KEYWORD_IMPORT => Ok(AstModuleNode::Import(parse_import_statement(builder)?)),
            unknown_token => Err(ErrorCause::SyntaxError(AstError::UnexpectedToken(
                snapshot.clone(),
            ))),
        };
    }

    if let TokenDeclaration::Commitment(snapshot) = peek {
        return match snapshot.token.as_str() {
            constants::COMMITMENT_ID => todo!("commitment id not implemented"),
            unknown_token => Err(ErrorCause::SyntaxError(AstError::UnexpectedToken(
                snapshot.clone(),
            ))),
        };
    }

    panic!(
        "Try to parse on top level for unknown TokenDeclaration {:#?}",
        builder.tokens.peek().unwrap()
    );
}
