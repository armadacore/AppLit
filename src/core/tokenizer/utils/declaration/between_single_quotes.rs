use crate::bin::constants;
use crate::core::tokenizer::reader::{TokenReaderSnapshot};
use super::DeclarationValidation;

pub fn validate<F>(declaration: &mut DeclarationValidation, mut callback: F)
where
    F: 'static + FnMut(TokenReaderSnapshot),
{
    let mut can_skip = false;
    let mut start_single_quote = false;
    let mut end_single_quote = false;
    let ignore_tokens = [constants::SINGLE_QUOTES_TOKEN];

    declaration.stack.push(Box::new(move |snapshot|{
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
    }));
}