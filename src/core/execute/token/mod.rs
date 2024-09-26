use crate::feedback::error::ErrorFeedback;

pub mod main;

pub mod module;

pub type DeclarationResult<T> = Result<Vec<T>, ErrorFeedback>;