use crate::feedback::error::ErrorFeedback;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;
use std::vec;
use std::fmt::Debug;

mod next;

#[derive(Debug)]
pub struct TokenReaderStack<T> {
    lines: Vec<String>,
    line: Option<String>,
    pos: usize,
    end: usize,
    line_number: usize,
    tokens: Vec<String>,
    token: Option<String>,
    ast: Vec<T>,
}

impl<T: Debug> TokenReaderStack<T> {
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

    pub fn next(&mut self) -> Option<String>{ next::token(self) }
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
    let lines = reader
        .lines()
        .collect::<Result<Vec<String>, Error>>().unwrap_or_else(|_| vec![]);
    let tokens: Vec<String> = lines.iter()
        .flat_map(|l| l.split(|c: char| c.is_whitespace())
            .map(|s| s.to_string()))
        .collect();
    let mut stack: TokenReaderStack<T> = TokenReaderStack {
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