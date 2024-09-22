use crate::feedback::error::{unknown_token, ErrorFeedback};
use crate::tokenizer::tokens::Token;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;
use std::str::SplitWhitespace;

pub mod import;
pub mod function;

pub fn new(file_path: &Path) -> Result<Vec<Token>, ErrorFeedback> {
    let mut lines = read_lines(file_path);
    let mut ast: Vec<Token> = vec![];

    while let Some(Ok(line)) = lines.next() {
        for token in read_tokens(&line) {
            match check_token(token) { 
                Some(declaration) => ast.push(declaration),
                None => return Err(unknown_token(token)),
            };
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

fn check_token(token: &str) -> Option<Token>{
    if import::check(token) {
        return Some(Token::Import(import::Declaration{
            specifier: String::from("foobar"),
            from: String::from("foobar"),
        }));
    }

    if function::check() {
        return None;
    }
    
    None
}