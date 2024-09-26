use std::fmt::Debug;
use crate::core::execute::token_reader::TokenReaderStack;

pub fn token<T: Debug>(stack: &mut TokenReaderStack<T>) -> Option<String>{
    if stack.tokens.is_empty(){
        adjust_next_line(stack);
        adjust_line_number(stack);
        adjust_tokens(stack);
    }

    stack.token = if stack.tokens.is_empty(){
        None
    } else {
        Some(stack.tokens.remove(0))
    };
    adjust_pos(stack);
    adjust_end(stack);
    stack.token.clone()
}

fn adjust_next_line<T: Debug>(stack: &mut TokenReaderStack<T>) {
    stack.line = if stack.lines.is_empty(){
        None
    } else {
        Some(stack.lines.remove(0))
    }
}

fn adjust_line_number<T: Debug>(stack: &mut TokenReaderStack<T>) {
    if stack.line.is_some(){
        stack.line_number += 1;
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
    stack.pos = if stack.end > 0 {
        if let Some(token) = &stack.token{
            get_calc_position(stack.pos, token.len())
        } else {
            stack.pos
        }
    } else {
        stack.pos
    };
}

fn adjust_end<T: Debug>(stack: &mut TokenReaderStack<T>) {
    stack.end = if let Some(token) = &stack.token{
        get_calc_position(stack.end, token.len())
    } else {
        stack.end
    };
}

fn get_calc_position(position: usize, token_len: usize) -> usize{
    let mut position= position;

    if position == 0{
        position += token_len;
    } else {
        position += token_len + 1;
    }

    position
}