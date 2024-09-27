use crate::bin::constants;
use crate::core::execute::token_reader::TokenReaderNextLiteral;

pub fn curly_braces<F>(mut callback: F) -> impl FnMut(&TokenReaderNextLiteral) -> bool 
where F: FnMut(Vec<TokenReaderNextLiteral>) {
    let mut done = false;
    let mut start_curly_braces = false;
    let mut end_curly_braces = false;
    let mut result: Vec<TokenReaderNextLiteral> = vec![];

    move |next_literal: &TokenReaderNextLiteral| {
        if done { return false }
        
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
            callback(result.clone());
            result = vec![];
            start_curly_braces = false;
            end_curly_braces = false;
            return true;
        }

        if start_curly_braces {
            result.push(next_literal.clone());
        }
        
        false
    }
}