use crate::bin::constants;
use crate::core::tokenizer::reader::{TokenReaderSnapshot};
use super::{DeclarationState, DeclarationValidation};

pub fn validate<F>(declaration: &mut DeclarationValidation, mut callback: F)
where
    F: 'static + FnMut(TokenReaderSnapshot),
{
    let mut can_skip = false;
    let mut start_curly_braces = false;
    let mut end_curly_braces = false;
    let ignore_tokens = [
        constants::START_CURLY_BRACES_TOKEN,
        constants::COMMA_TOKEN,
        constants::END_CURLY_BRACES_TOKEN,
    ];

    declaration.stack.push(Box::new(move |snapshot|{
        if can_skip {
            return DeclarationState::Found;
        }

        let token = snapshot.token.clone().unwrap_or_default();

        if token == constants::START_CURLY_BRACES_TOKEN {
            start_curly_braces = true;
        }

        if token == constants::END_CURLY_BRACES_TOKEN {
            end_curly_braces = true;
        }

        if start_curly_braces && end_curly_braces {
            can_skip = true;
            start_curly_braces = false;
            end_curly_braces = false;
            return DeclarationState::Found;
        }

        if start_curly_braces && !ignore_tokens.contains(&token.as_str()) {
            callback(snapshot);
        }

        DeclarationState::Search
    }));
}