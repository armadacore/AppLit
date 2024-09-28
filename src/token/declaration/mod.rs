use crate::feedback::error::ErrorCause;

pub mod main;

pub mod module;

pub type DeclarationResult<T> = Result<Vec<T>, ErrorCause>;
