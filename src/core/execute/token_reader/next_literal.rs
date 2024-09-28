use crate::bin::constants;
use crate::core::execute::token_reader::{TokenReaderNextLiteral, TokenReaderStack};
use std::fmt::Debug;
use crate::core::execute::token_reader::token_utils::location::get_location;

pub fn token<T: Debug>(stack: &mut TokenReaderStack<T>) -> Option<TokenReaderNextLiteral> {
    let ignore_tokens = [
        constants::EMPTY,
        constants::SPACE,
        constants::START_CURLY_BRACES_TOKEN,
        constants::SINGLE_QUOTES_TOKEN,
        constants::COLON_TOKEN,
        constants::COMMA_TOKEN,
        constants::END_CURLY_BRACES_TOKEN,
        constants::SEMICOLON_TOKEN,
    ];

    let mut prev_token = None;
    while let Some(token) = stack.next() {
        if !ignore_tokens.contains(&token.as_str()) {
            return Some(TokenReaderNextLiteral {
                location: get_location(stack),
                prev_token,
                token,
            });
        }

        if token == constants::SEMICOLON_TOKEN {
            break;
        }

        if token != constants::EMPTY && token != constants::SPACE {
            prev_token = Some(token);
        }
    }

    None
}
