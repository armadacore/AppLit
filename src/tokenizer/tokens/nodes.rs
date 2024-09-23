use crate::feedback::error::{unknown_token, ErrorFeedback};
use crate::tokenizer::tokens::Token;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;
use std::str::SplitWhitespace;
use std::vec;

pub mod import;
pub mod function;

pub fn new(file_path: &Path) -> Result<Vec<Token>, ErrorFeedback> {
    let mut lines = read_lines(file_path);
    let mut ast: Vec<Token> = vec![];

    while let Some(Ok(line)) = lines.next() {
        for token in read_tokens(&line) {
            if check_token(&mut ast, token).is_none() { return Err(unknown_token(token)) };
        }
    }

    Ok(ast)
}

fn read_lines(file_path: &Path) -> Lines<BufReader<File>>{
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    reader.lines()
}

fn read_tokens(line: &str) -> SplitWhitespace{
    line.split_whitespace()
}

fn check_token(ast: &mut Vec<Token>, token: &str) -> Option<()>{
    if import::check(ast, token) {
        return Some(());
    }

    if function::check() {
        return Some(());
    }
    
    None
}