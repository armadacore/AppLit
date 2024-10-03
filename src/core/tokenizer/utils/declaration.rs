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
    while let Some(snapshot) = &stack.next_literal() {
        for declaration_item in structure.iter_mut() {
            if declaration_item(snapshot.clone()) {
                return true;
            }
        }
    }

    false
}

pub fn expected_token<F>(token: String, mut callback: F) -> impl FnMut(TokenReaderSnapshot) -> bool
where
    F: FnMut(TokenReaderSnapshot),
{
    let mut can_skip = false;

    move |snapshot| {
        if can_skip {
            return false;
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

    move |snapshot| {
        if can_skip {
            return false;
        }

        if let Some(staring_curly_braces) = &snapshot.prev_token {
            if staring_curly_braces == constants::START_CURLY_BRACES_TOKEN {
                start_curly_braces = true;
            }
        }

        if let Some(ending_curly_braces) = &snapshot.prev_token {
            if ending_curly_braces == constants::END_CURLY_BRACES_TOKEN {
                end_curly_braces = true;
            }
        }

        if start_curly_braces && end_curly_braces {
            can_skip = true;
            start_curly_braces = false;
            end_curly_braces = false;
        }

        if start_curly_braces {
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

    move |snapshot| {
        if can_skip {
            return false;
        }

        if let Some(starting_single_quotes) = &snapshot.prev_token {
            if starting_single_quotes == constants::SINGLE_QUOTES_TOKEN {
                if start_single_quote && !end_single_quote {
                    end_single_quote = true;
                }

                if !start_single_quote {
                    start_single_quote = true;
                }
            }
        }

        if start_single_quote && end_single_quote {
            can_skip = true;
            start_single_quote = false;
            end_single_quote = false;
        }

        if start_single_quote {
            callback(snapshot.clone());
        }

        can_skip
    }
}
