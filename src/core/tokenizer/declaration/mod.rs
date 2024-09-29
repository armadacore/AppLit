use crate::core::feedback::error::ErrorCause;
use crate::core::tokenizer::reader::TokenReaderStack;

pub mod main;

pub mod module;

pub type DeclarationResult<T> = Result<TokenReaderStack<T>, ErrorCause>;
