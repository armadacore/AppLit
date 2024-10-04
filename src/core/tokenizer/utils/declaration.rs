use crate::bin::constants;
use crate::core::tokenizer::reader::{TokenReaderSnapshot, TokenReaderStack};
use std::fmt::Debug;

pub fn structure_validation<T: Debug + Clone, F>(
    stack: &mut TokenReaderStack<T>,
    structure: &mut [F],
) -> bool
where
    F: FnMut(TokenReaderSnapshot) -> bool,
{
    let structure_len = structure.len() - 1;
    let mut result = false;

    while let Some(snapshot) = &stack.next() {
        if let Some(token) = &snapshot.token {
            if token.trim().is_empty() {
                continue;
            }
        }

        for cb in structure.iter_mut() {
            result = cb(snapshot.clone());
        }
        
        if result {
            return true;
        }

        if let Some(token) = &snapshot.token {
            if token == constants::SEMICOLON_TOKEN {
                return result;
            }
        }
    }

    result
}

pub fn expected_token<F>(token: String, mut callback: F) -> impl FnMut(TokenReaderSnapshot) -> bool
where
    F: FnMut(TokenReaderSnapshot),
{
    let mut can_skip = false;

    move |snapshot| {
        if can_skip {
            return true;
        }

        if let Some(snapshot_token) = &snapshot.token {
            if snapshot_token.eq(&token) {
                can_skip = true;
                callback(snapshot);
            }
        }

        can_skip
    }
}

pub fn between_curly_braces<F>(mut callback: F) -> impl FnMut(TokenReaderSnapshot) -> bool
where
    F: FnMut(TokenReaderSnapshot),
{
    let mut can_skip = false;
    let mut start_curly_braces = false;
    let mut end_curly_braces = false;
    let ignore_tokens = [
        constants::START_CURLY_BRACES_TOKEN,
        constants::COMMA_TOKEN,
        constants::END_CURLY_BRACES_TOKEN,
    ];

    move |snapshot| {
        if can_skip {
            return true;
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
        }

        if start_curly_braces && !ignore_tokens.contains(&token.as_str()) {
            callback(snapshot);
        }

        can_skip
    }
}

pub fn between_single_quotes<F>(mut callback: F) -> impl FnMut(TokenReaderSnapshot) -> bool
where
    F: FnMut(TokenReaderSnapshot),
{
    let mut can_skip = false;
    let mut start_single_quote = false;
    let mut end_single_quote = false;
    let ignore_tokens = [constants::SINGLE_QUOTES_TOKEN];

    move |snapshot| {
        if can_skip {
            return true;
        }

        let token = snapshot.token.clone().unwrap_or_default();

        if token == constants::SINGLE_QUOTES_TOKEN {
            if start_single_quote && !end_single_quote {
                end_single_quote = true;
            }

            if !start_single_quote {
                start_single_quote = true;
            }
        }

        if start_single_quote && end_single_quote {
            can_skip = true;
            start_single_quote = false;
            end_single_quote = false;
        }

        if start_single_quote && !ignore_tokens.contains(&token.as_str()) {
            callback(snapshot.clone());
        }

        can_skip
    }
}
