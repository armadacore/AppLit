use crate::bin::constants;
use crate::core::execute::token_reader::TokenReaderNextLiteral;

pub fn curly_braces<F>(mut callback: F) -> impl FnMut(&TokenReaderNextLiteral) 
where F: FnMut(TokenReaderNextLiteral) {
    let mut done = false;
    let mut start_curly_braces = false;
    let mut end_curly_braces = false;

    move |next_literal: &TokenReaderNextLiteral| {
        if done { return }
        
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
            done = true;
            start_curly_braces = false;
            end_curly_braces = false;
        }

        if start_curly_braces {
            callback(next_literal.clone());
        }
    }
}

pub fn single_quotes<F>(mut callback: F) -> impl FnMut(&TokenReaderNextLiteral) 
where F: FnMut(TokenReaderNextLiteral) {
    let mut done = false;
    let mut start_single_quote = false;
    let mut end_single_quote = false;

    move |next_literal: &TokenReaderNextLiteral| {
        if done { return }

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
            done = true;
            start_single_quote = false;
            end_single_quote = false;
        }

        if start_single_quote {
            callback(next_literal.clone());
        }
    }
}