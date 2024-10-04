use crate::bin::constants;
use crate::core::feedback::error::ErrorCause;
use crate::core::tokenizer::reader::{TokenReaderSnapshot, TokenReaderStack};
use regex::Regex;
use std::fmt::Debug;
use crate::core::tokenizer::utils::location::get_location;

pub fn token<T: Debug + Clone>(stack: &mut TokenReaderStack<T>) -> Option<TokenReaderSnapshot> {
    if stack.tokens.is_empty() {
        adjust_next_line(stack);
        adjust_tokens(stack);
    }

    stack.token = if stack.tokens.is_empty() {
        None
    } else {
        let new_token = stack.tokens.remove(0);

        adjust_pos(stack);
        adjust_end(stack, &new_token);
        Some(new_token)
    };
    
    match &stack.token { 
        Some(token) => {
            Some(TokenReaderSnapshot {
                location: get_location(stack),
                token: stack.token.clone()
            })
        },
        None => None
    }
}

fn adjust_next_line<T: Debug + Clone>(stack: &mut TokenReaderStack<T>) {
    stack.line = match stack.lines.next() {
        None => None,
        Some(line_result) => match line_result {
            Err(error) => panic!("{}", ErrorCause::Unhandled(Box::new(error))),
            Ok(line) => {
                stack.line_number += 1;

                if line.is_empty() {
                    return adjust_next_line(stack);
                }

                Some(line)
            }
        },
    };
}

fn adjust_tokens<T: Debug + Clone>(stack: &mut TokenReaderStack<T>) {
    if let Some(ref line) = stack.line {
        let regex_tokens = [
            constants::START_CURLY_BRACES_TOKEN,
            constants::COLON_TOKEN,
            constants::END_CURLY_BRACES_TOKEN,
            constants::SINGLE_QUOTES_TOKEN,
            constants::COMMA_TOKEN,
            constants::SEMICOLON_TOKEN,
        ];
        let regex_pattern = format!("\\w+|[{}]|\\s", regex_tokens.join(""));
        let regexp = Regex::new(&regex_pattern).unwrap();
        stack.tokens = regexp
            .find_iter(line)
            .map(|res| res.as_str().to_string())
            .collect();
    }
}

fn adjust_pos<T: Debug + Clone>(stack: &mut TokenReaderStack<T>) {
    stack.start = stack.end;
}

fn adjust_end<T: Debug + Clone>(stack: &mut TokenReaderStack<T>, new_token: &str) {
    stack.end += new_token.len();
}
