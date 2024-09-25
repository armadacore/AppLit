use crate::feedback::error::ErrorFeedback;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;
use std::vec;
use std::fmt::Debug;

//TokenStack

#[derive(Debug)]
pub struct LexerStack<T> {
    lines: Vec<String>,
    line: Option<String>,
    pos: usize,
    end: usize,
    line_number: usize,
    tokens: Vec<String>,
    token: Option<String>,
    ast: Vec<T>,
}

impl<T: Debug> LexerStack<T> {
    pub fn get_pos(&self) -> usize{
        self.pos
    }

    pub fn get_end(&self) -> usize{
        self.end
    }

    pub fn get_line_number(&self) -> usize{
        self.line_number
    }

    pub fn get_token(&self) -> Option<String>{
        self.token.clone()
    }

    pub fn ast_add(&mut self, value: T) {
        self.ast.push(value);
    }

    pub fn next(&mut self) -> Option<String>{
        if self.tokens.is_empty(){
            adjust_next_line(self);
            adjust_line_number(self);
            adjust_tokens(self);
        }

        self.token = if self.tokens.is_empty(){
            None
        } else {
            Some(self.tokens.remove(0))
        };
        adjust_pos(self);
        adjust_end(self);
        self.token.clone()
    }
}

fn adjust_next_line<T: Debug>(stack: &mut LexerStack<T>) {
    stack.line = if stack.lines.is_empty(){
        None
    } else {
        Some(stack.lines.remove(0))
    }
}

fn adjust_line_number<T: Debug>(stack: &mut LexerStack<T>) {
    if stack.line.is_some(){
        stack.line_number += 1;
    }
}

fn adjust_tokens<T: Debug>(stack: &mut LexerStack<T>) {
    if let Some(ref line) = stack.line{
        stack.tokens = line.split(|c: char| c.is_whitespace())
            .map(|s| s.to_string())
            .collect();
    }
}

fn adjust_pos<T: Debug>(stack: &mut LexerStack<T>) {
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

fn adjust_end<T: Debug>(stack: &mut LexerStack<T>) {
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

pub fn new<T: Debug, F>(file_path: &Path, mut callback: F) -> Result<Vec<T>, ErrorFeedback>
where F: FnMut(&mut LexerStack<T>){
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let lines = reader
        .lines()
        .collect::<Result<Vec<String>, Error>>().unwrap_or_else(|_| vec![]);
    let tokens: Vec<String> = lines.iter()
        .flat_map(|l| l.split(|c: char| c.is_whitespace())
            .map(|s| s.to_string()))
        .collect();
    let mut stack: LexerStack<T> = LexerStack {
        lines,
        line: None,
        pos: 0,
        end: 0,
        line_number: 0,
        tokens: vec![],
        token: None,
        ast: vec![],
    };

    while let Some(token) = &stack.next() {
        callback(&mut stack);
    }
    Ok(stack.ast)
}