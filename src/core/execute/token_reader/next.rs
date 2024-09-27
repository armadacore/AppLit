use std::fmt::Debug;
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
    stack.line = if let Some(Ok(line)) = stack.lines.next(){
        stack.line_number += 1;

        Some(line)
    } else {
        None
    }
}

fn adjust_tokens<T: Debug>(stack: &mut TokenReaderStack<T>) {
    if let Some(ref line) = stack.line{
        stack.tokens = line.split(|c: char| c.is_whitespace())
            .map(|s| s.to_string())
            .collect();
    }
}

fn adjust_pos<T: Debug>(stack: &mut TokenReaderStack<T>) {
    if stack.end > 0 {
        let end = stack.end;
        let len = if let Some(token) = &stack.token{ token.len() } else { 0 };
        let result = if end == len { end + 1} else { end };
        stack.start = result;
    }
}

fn adjust_end<T: Debug>(stack: &mut TokenReaderStack<T>, new_token: &str) {
    stack.end += if stack.end == 0 {
        new_token.len()
    } else {
        new_token.len() + 1
    };
}