pub mod import;
pub mod function;
use crate::feedback::error::ErrorFeedback;
use crate::tokenizer::tokens::Token;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;
use std::vec;

#[derive(Debug)]
pub struct TokenToAst {
    lines: Vec<String>,
    line: Option<String>,
    tokens: Vec<String>,
    token: Option<String>,
    ast: Vec<Token>,
}

impl TokenToAst {
    pub fn next(&mut self) -> Option<String>{
        self.token = if self.tokens.is_empty() {
            self.line = if self.lines.is_empty(){
                None
            } else {
                Some(self.lines.remove(0))
            };
            
            match self.line { 
                Some(ref line) => {
                    self.tokens = line.split_whitespace().map(|s| s.to_string()).collect();
                    self.next()
                },
                None => None,
            }
        } else {
            Some(self.tokens.remove(0))
        };

        if let Some(ref token) = self.token{
            return Some(token.to_string());
        }

        None
    }
}

pub fn new(file_path: &Path) -> Result<Vec<Token>, ErrorFeedback> {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let lines = reader
        .lines()
        .collect::<Result<Vec<String>, Error>>().unwrap_or_else(|_| vec![]);
    let mut token_to_ast = TokenToAst{
        lines,
        line: None,
        tokens: vec![],
        token: None,
        ast: vec![]
    };

    transform_token_into_ast(&mut token_to_ast);

    Ok(token_to_ast.ast)
}

fn transform_token_into_ast(t2a: &mut TokenToAst){
    if let Some(token) = t2a.next(){
        if import::check(t2a) {
            return transform_token_into_ast(t2a);
        }

        if function::check() {
            return transform_token_into_ast(t2a);
        }

        transform_token_into_ast(t2a)
    }
}