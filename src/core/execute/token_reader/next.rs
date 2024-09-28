use std::fmt::Debug;
use regex::Regex;
use crate::core::execute::token_reader::TokenReaderStack;

pub fn token<T: Debug>(stack: &mut TokenReaderStack<T>) -> Option<String>{
    if stack.tokens.is_empty(){
        adjust_next_line(stack);
        adjust_tokens(stack);
    }

    stack.token = if stack.tokens.is_empty(){
        None
    } else {
        let new_token = stack.tokens.remove(0);

        adjust_pos(stack);
        adjust_end(stack, &new_token);
        Some(new_token)
    };
    stack.token.clone()
}

fn adjust_next_line<T: Debug>(stack: &mut TokenReaderStack<T>) {
    let next = stack.lines.next();
    
    stack.line = if let Some(Ok(line)) = stack.lines.next(){
        stack.line_number += 1;
    
        if line.is_empty() {
            return adjust_next_line(stack);
        }
    
        Some(line)
    } else {
        None
    }
}

fn adjust_tokens<T: Debug>(stack: &mut TokenReaderStack<T>) {
    if let Some(ref line) = stack.line{
        let regexp = Regex::new(r"(\w+|[{:}',;]|\s)").unwrap();
        stack.tokens = regexp.find_iter(line).map(|res| res.as_str().to_string()).collect();
    }
}

fn adjust_pos<T: Debug>(stack: &mut TokenReaderStack<T>) {
    stack.start = stack.end;
}

fn adjust_end<T: Debug>(stack: &mut TokenReaderStack<T>, new_token: &str) {
    stack.end += new_token.len();
}