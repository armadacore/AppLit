use crate::feedback::error::ErrorFeedback;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;
use std::vec;

mod next;

mod next_literal;

pub type TokenReaderNodes<T> = Vec<T>;

#[derive(Debug)]
pub struct TokenReaderStack<T> {
    lines: Lines<BufReader<File>>,
    line: Option<String>,
    start: usize,
    end: usize,
    line_number: usize,
    tokens: Vec<String>,
    token: Option<String>,
    ast: Vec<T>,
}

#[derive(Debug)]
pub struct TokenReaderLocation {
    pub start: usize,
    pub end: isize,
    pub line_start: usize,
    pub line_end: isize,
}

#[derive(Debug)]
pub struct TokenReaderNextLiteral {
    pub prev_token: Option<String>,
    pub token: String,
}

impl<T: Debug> TokenReaderStack<T> {
    pub fn get_start_pos(&self) -> usize{
        self.start
    }

    pub fn get_end_pos(&self) -> usize{ self.end }

    pub fn get_line_number(&self) -> usize{
        self.line_number
    }

    pub fn get_token(&self) -> Option<String>{
        self.token.clone()
    }

    pub fn add_declaration(&mut self, value: T) {
        self.ast.push(value);
    }

    pub fn next(&mut self) -> Option<String>{ next::token(self) }

    pub fn next_literal(&mut self) -> Option<TokenReaderNextLiteral>{
        next_literal::token(self)
    }

    pub fn get_location(&self) -> TokenReaderLocation{
        TokenReaderLocation{
            start: self.get_start_pos(),
            end: self.get_end_pos() as isize,
            line_start: self.get_line_number(),
            line_end: self.get_line_number() as isize,
        }
    }

    pub fn update_location_end(&self, location: &mut TokenReaderLocation) {
        location.end = self.get_end_pos() as isize;
        location.line_end = self.get_line_number() as isize;
    }
}

pub fn run<T: Debug, F>(file_path: &Path, callback: F) -> Result<Vec<T>, ErrorFeedback>
where F: FnMut(&mut TokenReaderStack<T>) {
    new(file_path, callback)
}

pub fn run_tokens<T: Debug, F>(file_path: &Path, tokens: &mut [F]) -> Result<Vec<T>, ErrorFeedback>
where F: FnMut(&mut TokenReaderStack<T>) -> bool{
    new(file_path, |stack|{
        if let Some(token) = stack.get_token(){
            for cb in tokens.iter_mut(){
                if cb(stack) {
                    break;
                }
            }
        }
    })
}

fn new<T: Debug, F>(file_path: &Path, mut callback: F) -> Result<Vec<T>, ErrorFeedback>
where F: FnMut(&mut TokenReaderStack<T>){
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut stack: TokenReaderStack<T> = TokenReaderStack {
        lines: reader.lines(),
        line: None,
        start: 0,
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