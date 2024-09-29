use crate::bin::constants;
use crate::core::tokenizer::reader::{TokenReaderLocation, TokenReaderStack};
use crate::core::tokenizer::utils::location::{get_location, update_location_end};
use std::fmt::Debug;
use crate::core::feedback::error::{ErrorCause, SyntaxErrorCause, SyntaxErrorLocation};

const UNKNOWN_TOKEN: &str = "Unknown token";

pub fn report<T: Debug + Clone>(stack: &mut TokenReaderStack<T>) {
    if let Some(token) = &stack.get_token() {
        let mut location = get_location(stack);

        if token != constants::SEMICOLON_TOKEN {
            while let Some(toke) = stack.next() {
                if token == constants::SEMICOLON_TOKEN {
                    update_location_end(stack, &mut location);
                    break;
                }
            }
            
            push_error(stack, location, UNKNOWN_TOKEN);
        }
    }
}

pub fn declaration_report<T: Debug + Clone>(
    stack: &mut TokenReaderStack<T>,
    location: TokenReaderLocation,
    cause: &str,
) {
    if let Some(token) = &stack.get_token() {
        push_error(stack, location, cause);
    }
}

fn push_error<T: Debug + Clone>(stack: &mut TokenReaderStack<T>, location: TokenReaderLocation, cause: &str) {
    stack.error.push(ErrorCause::SyntaxError(SyntaxErrorCause {
        location: SyntaxErrorLocation {
            start: location.start,
            end: location.end,
            line_start: location.line_start,
            line_end: location.line_end,
        },
        cause: cause.to_owned(),
    }));
}
