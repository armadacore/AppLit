use crate::core::feedback::ErrorCause;
use crate::core::parser::AstError;
use crate::core::tokenizer::TokenDeclaration;

pub fn snapshot_error(token_declaration: Option<&TokenDeclaration>) -> ErrorCause {
    if let Some(token_declaration) = token_declaration {
        return ErrorCause::SyntaxError(AstError::UnexpectedToken(token_declaration.clone().extract_snapshot()));
    }

    ErrorCause::SyntaxError(AstError::UnexpectedError(None))
}
