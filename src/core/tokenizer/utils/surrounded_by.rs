use crate::bin::constants;
use crate::core::tokenizer::reader::{TokenReaderSnapshot};

pub fn curly_braces<F>(mut callback: F) -> impl FnMut(TokenReaderSnapshot) -> bool
where
    F: FnMut(TokenReaderSnapshot),
{
    let mut has_to_check = false;
    let mut start_curly_braces = false;
    let mut end_curly_braces = false;

    move |next_literal| {
        if has_to_check {
            return false;
        }
        
        if let Some(staring_curly_braces) = &next_literal.prev_token {
            if staring_curly_braces == constants::START_CURLY_BRACES_TOKEN {
                start_curly_braces = true;
            }
        }

        if let Some(ending_curly_braces) = &next_literal.prev_token {
            if ending_curly_braces == constants::END_CURLY_BRACES_TOKEN {
                end_curly_braces = true;
            }
        }

        if start_curly_braces && end_curly_braces {
            has_to_check = true;
            start_curly_braces = false;
            end_curly_braces = false;
        }

        if start_curly_braces {
            callback(next_literal);
        }

        if has_to_check {
            true
        } else {
            false
        }
    }
}

pub fn single_quotes<F>(mut callback: F) -> impl FnMut(TokenReaderSnapshot) -> bool
where
    F: FnMut(TokenReaderSnapshot),
{
    let mut has_to_check = false;
    let mut start_single_quote = false;
    let mut end_single_quote = false;

    move |next_literal| {
        if has_to_check {
            return false;
        }
        
        if let Some(starting_single_quotes) = &next_literal.prev_token {
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
            has_to_check = true;
            start_single_quote = false;
            end_single_quote = false;
        }

        if start_single_quote {
            callback(next_literal.clone());
        }

        if has_to_check {
            true
        } else {
            false
        }
    }
}
