use crate::core::parser::AstError;
use crate::core::tokenizer::TokenDeclaration;

pub fn try_snapshot_error(token_declaration: Option<&TokenDeclaration>) -> AstError {
    if let Some(token_declaration) = token_declaration {
        return AstError::UnexpectedToken(token_declaration.clone().extract_snapshot());
    }

    AstError::UnexpectedError
}
