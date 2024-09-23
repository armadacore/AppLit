pub mod import;
pub mod function;
use crate::feedback::error::ErrorFeedback;
use crate::tokenizer::tokens::Token;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::vec;

#[derive(Debug)]
pub struct TokenToAst {
    tokens: Vec<String>,
    token: Option<String>,
    ast: Vec<Token>,
}

impl TokenToAst {
    pub fn current_token(&self) -> Option<&str>{
        if let Some(ref token) = self.token{
            return Some(token);
        }
    
        None
    }

    pub fn next(&mut self) -> Option<&str>{
        self.token = if self.tokens.is_empty() {
            None
        } else {
            Some(self.tokens.remove(0))
        };

        if let Some(ref token) = self.token{
            return Some(token);
        }

        None
    }
}

pub fn new(file_path: &Path) -> Result<Vec<Token>, ErrorFeedback> {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut token_to_ast: Option<TokenToAst> = None;

    while let Some(Ok(line)) = lines.next() {
        token_to_ast = Some(TokenToAst{
            tokens: line.split_whitespace().map(|s| s.to_string()).collect(),
            token: None,
            ast: if let Some(t2a) = token_to_ast {
                t2a.ast
            } else {
                vec![]
            }
        });

        if let Some(ref mut t2a) = token_to_ast{
            transform_token_into_ast(t2a);
        }
    }

    Ok(token_to_ast.unwrap().ast)
}

fn transform_token_into_ast(t2a: &mut TokenToAst){
    if let Some(token) = t2a.next(){
        if import::check(t2a) {
            return;
        }

        if function::check() {

        }
    }
}