pub mod import;
pub mod function;
use crate::feedback::error::ErrorFeedback;
use crate::tokenizer::tokens::Token;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;
use std::vec;

pub fn new(file_path: &Path) -> Result<Vec<Token>, ErrorFeedback> {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let lines = reader
        .lines()
        .collect::<Result<Vec<String>, Error>>().unwrap_or_else(|_| vec![]);
    let tokens: Vec<String> = lines.iter()
        .flat_map(|l| l.split(|c: char| c.is_whitespace())
            .map(|s| s.to_string()))
        .collect();
    let mut t2a = TokenToAst{
        lines,
        line: None,
        pos: 0,
        end: 0,
        line_number: 0,
        tokens: vec![],
        token: None,
        ast: vec![]
    };

    transform_token_into_ast(&mut t2a);

    Ok(t2a.ast)
}

fn transform_token_into_ast(t2a: &mut TokenToAst){
    if let Some(token) = t2a.next(){
        if import::check(t2a) { return transform_token_into_ast(t2a); }
        if function::check() { return transform_token_into_ast(t2a); }
        transform_token_into_ast(t2a);
    }
}

#[derive(Debug)]
pub struct TokenToAst {
    lines: Vec<String>,
    line: Option<String>,
    pos: usize,
    end: usize,
    line_number: usize,
    tokens: Vec<String>,
    token: Option<String>,
    ast: Vec<Token>,
}

impl TokenToAst {
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

fn adjust_next_line(t2a: &mut TokenToAst){
    t2a.line = if t2a.lines.is_empty(){
        None
    } else {
        Some(t2a.lines.remove(0))
    }
}

fn adjust_line_number(t2a: &mut TokenToAst){
    if t2a.line.is_some(){
        t2a.line_number += 1;
    }
}

fn adjust_tokens(t2a: &mut TokenToAst) {
    if let Some(ref line) = t2a.line{
        t2a.tokens = line.split(|c: char| c.is_whitespace())
            .map(|s| s.to_string())
            .collect();
    }
}

fn adjust_pos(t2a: &mut TokenToAst) {
    t2a.pos = if t2a.end > 0 {
        if let Some(token) = &t2a.token{
            get_calc_position(t2a.pos, token.len())   
        } else { 
            t2a.pos
        }
    } else {
        t2a.pos
    };
}

fn adjust_end(t2a: &mut TokenToAst){
    t2a.end = if let Some(token) = &t2a.token{
        get_calc_position(t2a.end, token.len())
    } else {
        t2a.end
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