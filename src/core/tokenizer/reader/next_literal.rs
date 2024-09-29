use crate::bin::constants;
use crate::core::tokenizer::reader::{TokenReaderNextLiteral, TokenReaderStack};
use crate::core::tokenizer::utils::location::get_location;
use std::fmt::Debug;

pub fn token<T: Debug + Clone>(stack: &mut TokenReaderStack<T>) -> Option<TokenReaderNextLiteral> {
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
