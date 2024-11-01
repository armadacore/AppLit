use crate::core::feedback::error::Cause;
use crate::core::parser::error::AstError;
use crate::core::tokenizer::entities::declaration::TokenDeclaration;

pub fn snapshot_error(token_declaration: Option<&TokenDeclaration>) -> Cause {
    if let Some(token_declaration) = token_declaration {
        return Cause::SyntaxError(AstError::UnexpectedToken(token_declaration.clone().extract_snapshot()));
    }

    Cause::SyntaxError(AstError::UnexpectedError(None))
}
