use crate::bin::constants;
use crate::core::execute::token_reader::{TokenReaderCleanToken, TokenReaderLocation, TokenReaderStack};
use std::fmt::Debug;

pub fn clean_it<T: Debug>(stack: &TokenReaderStack<T>, mut current_token: String) -> TokenReaderCleanToken {
    let mut location = stack.get_location();
    stack.update_location(&mut location);

    clean_at_start(stack, &mut location, &mut current_token);
    clean_at_end(stack, &mut location, &mut current_token);

    TokenReaderCleanToken {
        token: current_token,
        location
    }
}

fn clean_at_start<T: Debug>(stack: &TokenReaderStack<T>, location: &mut TokenReaderLocation, current_token: &mut String){
    let start_tokens = vec![constants::START_CURLY_BRACES_TOKEN];
    
    for start in start_tokens{
        if current_token.starts_with(start){
            location.start+=1;
            current_token.remove(0);
        }
    }
}

fn clean_at_end<T: Debug>(stack: &TokenReaderStack<T>, location: &mut TokenReaderLocation, current_token: &mut String){
    let end_tokens = vec![constants::COMMA_TOKEN, constants::END_CURLY_BRACES_TOKEN, constants::SEMICOLON_TOKEN];
    
    for end in end_tokens{
        if current_token.ends_with(end) {
            location.end-=1;
            current_token.pop();
        }
    }
}